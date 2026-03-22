//! whisper-core — shared Rust inference library
//!
//! Provides a clean public API for whisper.cpp-based speech-to-text,
//! audio capture, voice activity detection, and model management.
//! Used by the Noter desktop app and future mobile client.

pub mod audio;
pub mod model;
pub mod stt;
pub mod vad;

pub use model::{ModelManager, WhisperSize};
pub use stt::{SttEngine, TranscriptionResult};
pub use vad::VadDetector;

/// High-level whisper-core engine combining model management + STT.
pub struct WhisperCore {
    pub model_manager: ModelManager,
    pub stt: SttEngine,
}

impl WhisperCore {
    /// Create a new WhisperCore instance, downloading the model if needed.
    ///
    /// # Arguments
    /// * `size` - The Whisper model size to use (e.g. `WhisperSize::Base`)
    ///
    /// # Errors
    /// Returns an error string if model download or STT init fails.
    pub fn new(size: WhisperSize) -> Result<Self, String> {
        let model_manager = ModelManager::new()?;
        let model_path = if !model_manager.whisper_exists(size) {
            model_manager.download_whisper(size, None)?
        } else {
            model_manager.whisper_path(size)
        };
        let stt = SttEngine::new(model_path.to_str().unwrap_or_default())?;
        Ok(Self { model_manager, stt })
    }

    /// Transcribe raw audio samples (f32, 16kHz mono), auto-detecting language.
    pub fn transcribe(&self, samples: &[f32]) -> Result<TranscriptionResult, String> {
        self.stt.transcribe_auto(samples)
    }

    /// Transcribe with explicit English language hint.
    pub fn transcribe_en(&self, samples: &[f32]) -> Result<TranscriptionResult, String> {
        self.stt.transcribe_en(samples)
    }

    /// Transcribe with explicit Chinese (Mandarin) language hint.
    pub fn transcribe_zh(&self, samples: &[f32]) -> Result<TranscriptionResult, String> {
        self.stt.transcribe_zh(samples)
    }
}
