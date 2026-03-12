use ort::session::Session;
use ort::value::Tensor;
use tokenizers::Tokenizer;
use std::path::Path;

/// A single translation model (encoder-decoder)
pub struct TranslationModel {
    pub name: String,
    encoder: Session,
    decoder: Session,
    tokenizer: Tokenizer,
}

impl TranslationModel {
    /// Load a translation model from a directory containing ONNX files
    pub fn load(model_dir: &Path, name: &str) -> Result<Self, String> {
        let encoder_path = model_dir.join("encoder_model.onnx");
        let decoder_path = model_dir.join("decoder_model.onnx");
        let tokenizer_path = model_dir.join("tokenizer.json");

        if !encoder_path.exists() {
            return Err(format!("Encoder not found at {}", encoder_path.display()));
        }
        if !decoder_path.exists() {
            return Err(format!("Decoder not found at {}", decoder_path.display()));
        }

        let encoder = Session::builder()
            .map_err(|e| format!("Session builder error: {e}"))?
            .with_intra_threads(4)
            .map_err(|e| format!("Thread config error: {e}"))?
            .commit_from_file(&encoder_path)
            .map_err(|e| format!("Failed to load encoder: {e}"))?;

        let decoder = Session::builder()
            .map_err(|e| format!("Session builder error: {e}"))?
            .with_intra_threads(4)
            .map_err(|e| format!("Thread config error: {e}"))?
            .commit_from_file(&decoder_path)
            .map_err(|e| format!("Failed to load decoder: {e}"))?;

        let tokenizer = if tokenizer_path.exists() {
            Tokenizer::from_file(&tokenizer_path)
                .map_err(|e| format!("Failed to load tokenizer: {e}"))?
        } else {
            return Err("tokenizer.json not found".to_string());
        };

        Ok(Self {
            name: name.to_string(),
            encoder,
            decoder,
            tokenizer,
        })
    }

    /// Translate a single text using encoder-decoder with greedy decoding
    pub fn translate(&mut self, text: &str) -> Result<String, String> {
        // 1. Tokenize input
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| format!("Tokenization error: {e}"))?;

        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
        let attention_mask: Vec<i64> = encoding.get_attention_mask().iter().map(|&m| m as i64).collect();
        let seq_len = input_ids.len() as i64;

        // 2. Create Tensor values for encoder
        let ids_tensor = Tensor::from_array((vec![1i64, seq_len], input_ids.clone()))
            .map_err(|e| format!("Tensor error: {e}"))?;
        let mask_tensor = Tensor::from_array((vec![1i64, seq_len], attention_mask.clone()))
            .map_err(|e| format!("Tensor error: {e}"))?;

        // 3. Run encoder
        let encoder_outputs = self.encoder.run(ort::inputs![ids_tensor, mask_tensor])
            .map_err(|e| format!("Encoder run error: {e}"))?;

        // Extract encoder hidden states shape and data
        let (enc_shape, enc_data) = encoder_outputs[0]
            .try_extract_tensor::<f32>()
            .map_err(|e| format!("Extract encoder hidden: {e}"))?;
        let enc_shape_vec: Vec<i64> = enc_shape.iter().copied().collect();
        let enc_data_vec: Vec<f32> = enc_data.to_vec();

        // 4. Auto-regressive greedy decoding
        let max_length = 512.min(input_ids.len() * 3);
        let mut generated_ids: Vec<i64> = vec![0]; // pad token as BOS for Marian

        for _ in 0..max_length {
            let dec_len = generated_ids.len() as i64;

            // Create decoder tensors
            let dec_input = Tensor::from_array((vec![1i64, dec_len], generated_ids.clone()))
                .map_err(|e| format!("Tensor error: {e}"))?;
            let enc_hidden = Tensor::from_array((enc_shape_vec.clone(), enc_data_vec.clone()))
                .map_err(|e| format!("Tensor error: {e}"))?;
            let dec_mask = Tensor::from_array((vec![1i64, seq_len], attention_mask.clone()))
                .map_err(|e| format!("Tensor error: {e}"))?;

            // Run decoder
            let decoder_outputs = self.decoder.run(ort::inputs![dec_input, enc_hidden, dec_mask])
                .map_err(|e| format!("Decoder run error: {e}"))?;

            // Get logits
            let (logit_shape, logit_data) = decoder_outputs[0]
                .try_extract_tensor::<f32>()
                .map_err(|e| format!("Extract logits: {e}"))?;

            // logits shape: [1, dec_len, vocab_size] — get last token
            let vocab_size = *logit_shape.last().unwrap_or(&0) as usize;
            let offset = (generated_ids.len() - 1) * vocab_size;
            let last_logits = &logit_data[offset..offset + vocab_size];

            // Greedy argmax
            let next_token = last_logits
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(idx, _)| idx as i64)
                .unwrap_or(0);

            // EOS check (0 = pad, 2 = </s> for Marian)
            if next_token == 0 || next_token == 2 {
                break;
            }

            generated_ids.push(next_token);
        }

        // 5. Decode output tokens (skip BOS)
        let output_ids: Vec<u32> = generated_ids.iter()
            .skip(1)
            .map(|&id| id as u32)
            .collect();
        
        let decoded = self.tokenizer.decode(&output_ids, true)
            .map_err(|e| format!("Decode error: {e}"))?;

        Ok(decoded.trim().to_string())
    }
}
