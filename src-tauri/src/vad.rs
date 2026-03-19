#![allow(dead_code)]
/// Voice Activity Detection
/// Primary: Energy-based VAD (no external model required)
/// Upgrade path: Silero VAD for higher accuracy
///
/// Energy-based VAD works well for desktop plugin use cases
/// where the microphone is close to the speaker.

use std::collections::VecDeque;

/// Configuration for energy-based VAD
#[derive(Debug, Clone)]
pub struct VadConfig {
    /// Energy threshold (RMS) to detect speech [0.0 - 1.0]
    pub energy_threshold: f32,
    /// Minimum speech probability threshold (for Silero path)
    pub speech_threshold: f32,
    /// Minimum silence duration (ms) before ending utterance
    pub silence_threshold_ms: u32,
    /// Window size for smoothing (number of chunks)
    pub smoothing_window: usize,
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            energy_threshold: 0.02,   // RMS threshold for speech detection
            speech_threshold: 0.5,    // For Silero compatibility
            silence_threshold_ms: 500, // 500ms silence ends utterance
            smoothing_window: 3,      // 3-chunk smoothing window
        }
    }
}

/// Voice Activity Detection state
pub struct VadDetector {
    config: VadConfig,
    /// Rolling energy history for smoothing
    energy_history: VecDeque<f32>,
    /// Buffered audio samples for current utterance
    buffer: Vec<f32>,
    /// Consecutive silence samples
    silence_samples: usize,
    /// Whether we're currently in a speech segment
    in_speech: bool,
}

impl VadDetector {
    /// Create a new VAD detector with default config
    pub fn new() -> Self {
        Self::with_config(VadConfig::default())
    }

    /// Create a VAD detector with custom config
    pub fn with_config(config: VadConfig) -> Self {
        let smoothing_window = config.smoothing_window;
        Self {
            config,
            energy_history: VecDeque::with_capacity(smoothing_window),
            buffer: Vec::with_capacity(16000),
            silence_samples: 0,
            in_speech: false,
        }
    }

    /// Create from Silero VAD model file (upgrade path)
    /// Falls back to energy-based VAD if model_path is empty
    pub fn with_silero(_model_path: &str) -> Result<Self, String> {
        // Silero VAD integration point:
        // When we have a compatible silero-vad crate, load it here.
        // For now, use energy-based VAD.
        // Silero VAD models can be downloaded via model.rs:
        //   model.download_vad(None)?;
        Ok(Self::new())
    }

    /// Calculate RMS energy of an audio chunk
    fn compute_energy(audio: &[f32]) -> f32 {
        if audio.is_empty() {
            return 0.0;
        }
        let sum: f32 = audio.iter().map(|&s| s * s).sum();
        (sum / audio.len() as f32).sqrt()
    }

    /// Apply smoothing to energy reading using rolling average
    fn smoothed_energy(&self) -> f32 {
        if self.energy_history.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.energy_history.iter().sum();
        sum / self.energy_history.len() as f32
    }

    /// Detect if speech is present in a single audio chunk
    /// audio_chunk: single chunk of audio samples (16kHz mono)
    /// Returns true if speech detected
    pub fn detect(&mut self, audio_chunk: &[f32]) -> bool {
        let energy = Self::compute_energy(audio_chunk);

        // Update smoothing history
        self.energy_history.push_back(energy);
        if self.energy_history.len() > self.config.smoothing_window {
            self.energy_history.pop_front();
        }

        self.smoothed_energy() > self.config.energy_threshold
    }

    /// Process audio and return complete utterances when silence is detected
    /// audio: audio samples (16kHz mono PCM)
    /// Returns Some(Vec<f32>) when an utterance is complete (ended by silence)
    /// Returns None if still collecting
    pub fn process(&mut self, audio: &[f32]) -> Option<Vec<f32>> {
        // Process in 512-sample chunks (32ms at 16kHz)
        let chunk_size = 512;
        let silence_chunk_threshold = (self.config.silence_threshold_ms as f32 / 32.0) as usize;

        for chunk in audio.chunks(chunk_size) {
            if chunk.len() < chunk_size {
                break;
            }

            let energy = Self::compute_energy(chunk);
            self.energy_history.push_back(energy);
            if self.energy_history.len() > self.config.smoothing_window {
                self.energy_history.pop_front();
            }

            let smooth_energy = self.smoothed_energy();

            if smooth_energy > self.config.energy_threshold {
                // Speech detected
                self.buffer.extend_from_slice(chunk);
                self.silence_samples = 0;
                self.in_speech = true;
            } else {
                // Silence
                if self.in_speech {
                    self.silence_samples += chunk.len();

                    if self.silence_samples >= silence_chunk_threshold * chunk_size {
                        // End of utterance — flush buffered audio
                        self.in_speech = false;
                        let mut utterance = Vec::new();
                        std::mem::swap(&mut utterance, &mut self.buffer);
                        self.buffer.reserve(16000);
                        self.silence_samples = 0;
                        return Some(utterance);
                    }
                }
            }
        }

        None
    }

    /// Flush any remaining audio as a final utterance
    /// Call this when audio capture is stopped
    pub fn flush(&mut self) -> Option<Vec<f32>> {
        if self.buffer.is_empty() {
            return None;
        }
        let mut utterance = Vec::new();
        std::mem::swap(&mut utterance, &mut self.buffer);
        self.buffer.reserve(16000);
        self.in_speech = false;
        self.silence_samples = 0;
        Some(utterance)
    }

    /// Reset the detector state
    pub fn reset(&mut self) {
        self.energy_history.clear();
        self.buffer.clear();
        self.silence_samples = 0;
        self.in_speech = false;
    }
}

impl Default for VadDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_threshold() {
        let mut vad = VadDetector::new();
        // Silent audio should not trigger
        let silence = vec![0.001f32; 512];
        assert!(!vad.detect(&silence));
    }

    #[test]
    fn test_utterance_flush() {
        let mut vad = VadDetector::new();
        let speech = vec![0.1f32; 16000]; // 1 second of speech
        let result = vad.process(&speech);
        assert!(result.is_none()); // No silence, no output yet
        let flush = vad.flush();
        assert!(flush.is_some());
        assert_eq!(flush.unwrap().len(), 16000);
    }
}
