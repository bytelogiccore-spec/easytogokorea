use ort::session::Session;
use std::path::Path;
use tokenizers::Tokenizer;

/// NLLB-200 language codes (BCP-47 style used by NLLB)
pub fn lang_code_to_nllb(code: &str) -> Option<&'static str> {
    match code {
        "ko" => Some("kor_Hang"),
        "en" => Some("eng_Latn"),
        "zh" => Some("zho_Hans"),
        "ja" => Some("jpn_Jpan"),
        "fr" => Some("fra_Latn"),
        "de" => Some("deu_Latn"),
        "es" => Some("spa_Latn"),
        "ru" => Some("rus_Cyrl"),
        "ar" => Some("arb_Arab"),
        "vi" => Some("vie_Latn"),
        "th" => Some("tha_Thai"),
        "id" => Some("ind_Latn"),
        "pt" => Some("por_Latn"),
        "it" => Some("ita_Latn"),
        "tr" => Some("tur_Latn"),
        "pl" => Some("pol_Latn"),
        "nl" => Some("nld_Latn"),
        "sv" => Some("swe_Latn"),
        "hi" => Some("hin_Deva"),
        "bn" => Some("ben_Beng"),
        "ms" => Some("zsm_Latn"),
        "tl" => Some("tgl_Latn"),
        "mn" => Some("khk_Cyrl"),
        "uk" => Some("ukr_Cyrl"),
        _ => None,
    }
}

/// NLLB Translation Model using ONNX Runtime (non-merged decoder)
pub struct NllbModel {
    encoder: Session,
    decoder: Session,
    tokenizer: Tokenizer,
}

impl NllbModel {
    pub fn load(model_dir: &Path) -> Result<Self, String> {
        let encoder_path = model_dir.join("encoder_model_quantized.onnx");
        // Use non-merged decoder (no KV cache complexity)
        let decoder_path = model_dir.join("decoder_model_quantized.onnx");
        let tokenizer_path = model_dir.join("tokenizer.json");

        if !decoder_path.exists() {
            return Err(
                "decoder_model_quantized.onnx not found. Please re-download from settings.".to_string()
            );
        }

        eprintln!("[NLLB] Loading encoder from {:?}", encoder_path);
        let encoder = Session::builder()
            .map_err(|e| format!("Encoder session builder error: {e}"))?
            .with_intra_threads(4)
            .map_err(|e| format!("Thread config error: {e}"))?
            .commit_from_file(&encoder_path)
            .map_err(|e| format!("Failed to load encoder: {e}"))?;

        eprintln!("[NLLB] Loading decoder from {:?}", decoder_path);
        let decoder = Session::builder()
            .map_err(|e| format!("Decoder session builder error: {e}"))?
            .with_intra_threads(4)
            .map_err(|e| format!("Thread config error: {e}"))?
            .commit_from_file(&decoder_path)
            .map_err(|e| format!("Failed to load decoder: {e}"))?;

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {e}"))?;

        eprintln!("[NLLB] Model loaded successfully");
        Ok(Self { encoder, decoder, tokenizer })
    }

    pub fn translate(
        &mut self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<String, String> {
        let src_nllb = lang_code_to_nllb(source_lang)
            .ok_or_else(|| format!("Unsupported source language: {source_lang}"))?;
        let tgt_nllb = lang_code_to_nllb(target_lang)
            .ok_or_else(|| format!("Unsupported target language: {target_lang}"))?;

        // NLLB tokenization: prepend source language token
        let input_text = format!("{} {}", src_nllb, text);

        let encoding = self.tokenizer.encode(input_text.as_str(), true)
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

        // Get target language token ID
        let tgt_token_id = self.tokenizer.token_to_id(tgt_nllb)
            .ok_or_else(|| format!("Target language token '{tgt_nllb}' not in vocabulary"))?;

        // Greedy decoding with non-merged decoder
        let eos_token_id = self.tokenizer.token_to_id("</s>").unwrap_or(2) as i64;
        let max_length = (seq_len * 3).min(512);
        let mut generated_ids: Vec<i64> = vec![eos_token_id, tgt_token_id as i64]; // </s> <tgt_lang>

        for _ in 0..max_length {
            let dec_len = generated_ids.len();

            let decoder_input = ort::value::Tensor::from_array(
                (vec![1i64, dec_len as i64], generated_ids.clone())
            ).map_err(|e| format!("Dec input tensor error: {e}"))?;

            let enc_attn_data: Vec<i64> = vec![1i64; seq_len];
            let enc_attn_tensor = ort::value::Tensor::from_array(
                (vec![1i64, seq_len as i64], enc_attn_data)
            ).map_err(|e| format!("Enc attn tensor error: {e}"))?;

            // Non-merged decoder inputs: input_ids, encoder_hidden_states, encoder_attention_mask
            let decoder_output = self.decoder.run(
                ort::inputs![decoder_input, &encoder_output[0], enc_attn_tensor]
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

        let output_ids: Vec<u32> = generated_ids[2..].iter().map(|&id| id as u32).collect();
        let decoded = self.tokenizer.decode(&output_ids, true)
            .map_err(|e| format!("Decoding error: {e}"))?;

        Ok(decoded.trim().to_string())
    }
}
