use ort::session::Session;
use std::path::Path;
use tokenizers::Tokenizer;

/// Opus-MT Translation Model (single language pair)
pub struct OpusMtModel {
    encoder: Session,
    decoder: Session,
    tokenizer: Tokenizer,
}

impl OpusMtModel {
    pub fn load(model_dir: &Path) -> Result<Self, String> {
        let encoder_path = model_dir.join("encoder_model.onnx");
        let decoder_path = model_dir.join("decoder_model_merged.onnx");
        let tokenizer_path = model_dir.join("tokenizer.json");

        eprintln!("[OpusMT] Loading encoder: {:?}", encoder_path);
        let encoder = Session::builder()
            .map_err(|e| format!("Session builder error: {e}"))?
            .with_intra_threads(4)
            .map_err(|e| format!("Thread config error: {e}"))?
            .commit_from_file(&encoder_path)
            .map_err(|e| format!("Failed to load encoder: {e}"))?;

        eprintln!("[OpusMT] Encoder loaded");

        let decoder = Session::builder()
            .map_err(|e| format!("Session builder error: {e}"))?
            .with_intra_threads(4)
            .map_err(|e| format!("Thread config error: {e}"))?
            .commit_from_file(&decoder_path)
            .map_err(|e| format!("Failed to load decoder: {e}"))?;

        eprintln!("[OpusMT] Decoder loaded");

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {e}"))?;

        Ok(Self { encoder, decoder, tokenizer })
    }

    pub fn translate(&mut self, text: &str) -> Result<String, String> {
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| format!("Tokenization error: {e}"))?;

        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
        let attention_mask: Vec<i64> = encoding.get_attention_mask().iter().map(|&m| m as i64).collect();
        let seq_len = input_ids.len();

        let input_ids_tensor = ort::value::Tensor::from_array(
            (vec![1i64, seq_len as i64], input_ids)
        ).map_err(|e| format!("Tensor error: {e}"))?;

        let attention_mask_tensor = ort::value::Tensor::from_array(
            (vec![1i64, seq_len as i64], attention_mask)
        ).map_err(|e| format!("Tensor error: {e}"))?;

        // Run encoder
        let encoder_output = self.encoder.run(
            ort::inputs![input_ids_tensor, attention_mask_tensor]
        ).map_err(|e| format!("Encoder run error: {e}"))?;

        // Greedy decoding
        let eos_token_id: i64 = 0; // Opus-MT uses 0 as EOS
        let pad_token_id: i64 = eos_token_id;
        let max_length = (seq_len * 3).min(512);

        // Start with decoder_start_token (pad token for Marian)
        let mut generated_ids: Vec<i64> = vec![pad_token_id];

        for _ in 0..max_length {
            let dec_len = generated_ids.len();

            let decoder_input = ort::value::Tensor::from_array(
                (vec![1i64, dec_len as i64], generated_ids.clone())
            ).map_err(|e| format!("Dec input tensor error: {e}"))?;

            let enc_attn_data: Vec<i64> = vec![1i64; seq_len];
            let enc_attn_tensor = ort::value::Tensor::from_array(
                (vec![1i64, seq_len as i64], enc_attn_data)
            ).map_err(|e| format!("Enc attn tensor error: {e}"))?;

            // use_cache_branch = false (no KV cache)
            let use_cache = ort::value::Tensor::from_array(
                (vec![1i64], vec![0.0f32])
            ).map_err(|e| format!("Cache flag error: {e}"))?;

            // Try with use_cache_branch first, fall back without
            let decoder_output = self.decoder.run(
                ort::inputs![decoder_input, enc_attn_tensor, &encoder_output[0], use_cache]
            ).map_err(|e| format!("Decoder run error: {e}"))?;

            let (_shape, logits_data) = decoder_output[0]
                .try_extract_tensor::<f32>()
                .map_err(|e| format!("Extract logits error: {e}"))?;

            let vocab_size = logits_data.len() / dec_len;
            let last_logits_start = (dec_len - 1) * vocab_size;
            if last_logits_start + vocab_size > logits_data.len() {
                return Err("Logits shape mismatch".to_string());
            }
            let last_logits = &logits_data[last_logits_start..last_logits_start + vocab_size];

            let next_token = last_logits
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(idx, _)| idx as i64)
                .unwrap_or(eos_token_id);

            if next_token == eos_token_id {
                break;
            }

            generated_ids.push(next_token);
        }

        let output_ids: Vec<u32> = generated_ids[1..].iter().map(|&id| id as u32).collect();
        let decoded = self.tokenizer.decode(&output_ids, true)
            .map_err(|e| format!("Decoding error: {e}"))?;

        Ok(decoded.trim().to_string())
    }
}
