#![allow(dead_code)]
/// Speech-to-text inference using whisper.cpp via whisper-rs
/// Supports bilingual EN/CN transcription

use whisper_rs::{
    get_lang_str, FullParams, SamplingStrategy, WhisperContext,
};

/// English language code
pub const LANG_EN: &str = "en";
/// Chinese language code
pub const LANG_ZH: &str = "zh";

/// Transcription result from whisper
#[derive(Debug, Clone)]
pub struct TranscriptionResult {
    /// The transcribed text
    pub text: String,
    /// Detected or specified language
    pub language: String,
    /// Start time in milliseconds
    pub start_ms: u64,
    /// End time in milliseconds
    pub end_ms: u64,
}

/// Whisper-based STT engine
pub struct SttEngine {
    ctx: WhisperContext,
}

impl SttEngine {
    /// Load a whisper model from disk
    pub fn new(model_path: &str) -> Result<Self, String> {
        let ctx = WhisperContext::new_with_params(model_path, Default::default())
            .map_err(|e| format!("Failed to load whisper model: {e}"))?;

        Ok(Self { ctx })
    }

    /// Run full transcription and return all results
    fn run_full(&self, params: FullParams, audio: &[f32]) -> Result<Vec<TranscriptionResult>, String> {
        let mut state = self
            .ctx
            .create_state()
            .map_err(|e| format!("Failed to create whisper state: {e}"))?;

        state
            .full(params, audio)
            .map_err(|e| format!("Whisper inference failed: {e}"))?;

        let num_segments = state
            .full_n_segments()
            .map_err(|e| format!("Failed to get segments: {e}"))? as usize;

        let mut results = Vec::with_capacity(num_segments);
        let mut full_text = String::new();

        for i in 0..num_segments {
            let text = state
                .full_get_segment_text(i as i32)
                .map_err(|e| format!("Failed to get segment text: {e}"))?;
            let t0 = state
                .full_get_segment_t0(i as i32)
                .map_err(|e| format!("Failed to get segment start: {e}"))?;
            let t1 = state
                .full_get_segment_t1(i as i32)
                .map_err(|e| format!("Failed to get segment end: {e}"))?;

            full_text.push_str(&text);
            full_text.push(' ');

            results.push(TranscriptionResult {
                text: text.clone(),
                language: String::new(), // filled below
                start_ms: (t0 as f64 * 10.0) as u64,
                end_ms: (t1 as f64 * 10.0) as u64,
            });
        }

        // Get detected language
        let lang_id = state
            .full_lang_id_from_state()
            .map_err(|e| format!("Failed to get language: {e}"))?;
        let lang_str = get_lang_str(lang_id as i32).unwrap_or(LANG_EN);

        // Fill language in results
        for r in &mut results {
            r.language = lang_str.to_string();
        }

        Ok(results)
    }

    /// Transcribe audio samples (16kHz mono PCM f32)
    pub fn transcribe(
        &self,
        audio: &[f32],
        language: Option<&str>,
    ) -> Result<TranscriptionResult, String> {
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        match language {
            Some(lang) if lang == LANG_EN || lang == LANG_ZH => {
                params.set_language(Some(lang));
                params.set_detect_language(false);
            }
            _ => {
                // Auto-detect language
                params.set_detect_language(true);
            }
        }

        params.set_no_timestamps(false);

        let results = self.run_full(params, audio)?;

        if results.is_empty() {
            return Ok(TranscriptionResult {
                text: String::new(),
                language: language.unwrap_or(LANG_EN).to_string(),
                start_ms: 0,
                end_ms: 0,
            });
        }

        // Merge all segments into single result
        let lang = results[0].language.clone();
        let start_ms = results.first().map(|r| r.start_ms).unwrap_or(0);
        let end_ms = results.last().map(|r| r.end_ms).unwrap_or(0);
        let text = results
            .iter()
            .map(|r| r.text.clone())
            .collect::<Vec<_>>()
            .join(" ");

        Ok(TranscriptionResult {
            text,
            language: lang,
            start_ms,
            end_ms,
        })
    }

    /// Transcribe with auto language detection
    #[allow(dead_code)]
    pub fn transcribe_auto(&self, audio: &[f32]) -> Result<TranscriptionResult, String> {
        self.transcribe(audio, None)
    }

    /// Transcribe English only
    #[allow(dead_code)]
    pub fn transcribe_en(&self, audio: &[f32]) -> Result<TranscriptionResult, String> {
        self.transcribe(audio, Some(LANG_EN))
    }

    /// Transcribe Chinese only
    #[allow(dead_code)]
    pub fn transcribe_zh(&self, audio: &[f32]) -> Result<TranscriptionResult, String> {
        self.transcribe(audio, Some(LANG_ZH))
    }
}
