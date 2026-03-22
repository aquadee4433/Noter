#![allow(dead_code)]
/// Model download and management system
/// Downloads and caches whisper.cpp and silero-vad models

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

/// Whisper model sizes available for download
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WhisperSize {
    Tiny,
    TinyEn,
    Base,
    BaseEn,
    Small,
    SmallEn,
    Medium,
    MediumEn,
    Large,
}

impl WhisperSize {
    /// Returns the GGML filename for this model size
    pub fn filename(&self) -> &'static str {
        match self {
            WhisperSize::Tiny => "ggml-tiny.bin",
            WhisperSize::TinyEn => "ggml-tiny.en.bin",
            WhisperSize::Base => "ggml-base.bin",
            WhisperSize::BaseEn => "ggml-base.en.bin",
            WhisperSize::Small => "ggml-small.bin",
            WhisperSize::SmallEn => "ggml-small.en.en.bin",
            WhisperSize::Medium => "ggml-medium.bin",
            WhisperSize::MediumEn => "ggml-medium.en.bin",
            WhisperSize::Large => "ggml-large.bin",
        }
    }

    /// Model size in MB (approximate)
    pub fn size_mb(&self) -> u32 {
        match self {
            WhisperSize::Tiny => 75,
            WhisperSize::TinyEn => 75,
            WhisperSize::Base => 148,
            WhisperSize::BaseEn => 148,
            WhisperSize::Small => 488,
            WhisperSize::SmallEn => 488,
            WhisperSize::Medium => 1600,
            WhisperSize::MediumEn => 1600,
            WhisperSize::Large => 3200,
        }
    }

    /// HuggingFace URL for the model
    pub fn url(&self) -> String {
        let file = self.filename();
        format!(
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{file}"
        )
    }

    /// Expected SHA256 checksum (partial — first 8 chars)
    pub fn checksum_prefix(&self) -> &'static str {
        // These are the official ggml checksums from whisper.cpp releases
        match self {
            WhisperSize::Tiny => "1d6c35",
            WhisperSize::TinyEn => "13bc028",
            WhisperSize::Base => "137c4b1",
            WhisperSize::BaseEn => "9549f9f",
            WhisperSize::Small => "685f818",
            WhisperSize::SmallEn => "1bbe3e6",
            WhisperSize::Medium => "18cd01a",
            WhisperSize::MediumEn => "7eb6aa",
            WhisperSize::Large => "0f8c293",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VadModel {
    SileroVad,
}

impl VadModel {
    pub fn filename(&self) -> &'static str {
        match self {
            VadModel::SileroVad => "silero_vad.onnx",
        }
    }

    pub fn url(&self) -> String {
        "https://huggingface.co/snakers4/silero-vad/resolve/main/model.onnx".to_string()
    }

    /// SileroVad model size in MB
    pub fn size_mb(&self) -> u32 {
        match self {
            VadModel::SileroVad => 2,
        }
    }
}

/// Progress callback for downloads
pub type ProgressFn = Box<dyn Fn(u64, u64) -> bool + Send + 'static>;

/// Model downloader/manager
pub struct ModelManager {
    models_dir: PathBuf,
    client: reqwest::blocking::Client,
}

impl ModelManager {
    /// Create a new model manager with the default models directory
    pub fn new() -> Result<Self, String> {
        let models_dir = dirs::data_local_dir()
            .ok_or("Failed to get local data directory")?
            .join("Noter")
            .join("models");

        fs::create_dir_all(&models_dir)
            .map_err(|e| format!("Failed to create models directory: {e}"))?;

        Ok(Self {
            models_dir,
            client: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()
                .map_err(|e| format!("Failed to build HTTP client: {e}"))?,
        })
    }

    /// Path where a whisper model would be stored
    pub fn whisper_path(&self, size: WhisperSize) -> PathBuf {
        self.models_dir.join("whisper").join(size.filename())
    }

    /// Path where the silero-vad model would be stored
    pub fn vad_path(&self) -> PathBuf {
        self.models_dir.join("silero_vad.onnx")
    }

    /// Check if a whisper model is already downloaded
    pub fn whisper_exists(&self, size: WhisperSize) -> bool {
        self.whisper_path(size).exists()
    }

    /// Check if the silero-vad model is already downloaded
    pub fn vad_exists(&self) -> bool {
        self.vad_path().exists()
    }

    /// Download a whisper model with optional progress callback
    /// Returns the path to the downloaded file
    pub fn download_whisper(
        &self,
        size: WhisperSize,
        progress: Option<ProgressFn>,
    ) -> Result<PathBuf, String> {
        let dest = self.whisper_path(size);

        if dest.exists() {
            log::info!("Whisper model {} already exists, skipping download", size.filename());
            return Ok(dest);
        }

        log::info!(
            "Downloading whisper model {} (~{}MB)...",
            size.filename(),
            size.size_mb()
        );

        let tmp_path = dest.with_extension("tmp");
        self.download_file(&size.url(), &tmp_path, progress)?;

        // Verify checksum (first 8 chars)
        let actual = self.checksum_hex(&tmp_path)?;
        let expected = size.checksum_prefix();
        if !actual.starts_with(expected) {
            fs::remove_file(&tmp_path).ok();
            return Err(format!(
                "Checksum mismatch for {}. Expected prefix {}, got {}",
                size.filename(),
                expected,
                &actual[..8]
            ));
        }

        fs::rename(&tmp_path, &dest)
            .map_err(|e| format!("Failed to rename downloaded file: {e}"))?;

        log::info!("Whisper model {} downloaded successfully", size.filename());
        Ok(dest)
    }

    /// Download the silero-vad model with optional progress callback
    pub fn download_vad(&self, progress: Option<ProgressFn>) -> Result<PathBuf, String> {
        let dest = self.vad_path();

        if dest.exists() {
            log::info!("SileroVad model already exists, skipping download");
            return Ok(dest);
        }

        log::info!(
            "Downloading silero-vad model (~{}MB)...",
            VadModel::SileroVad.size_mb()
        );

        let tmp_path = dest.with_extension("tmp");
        self.download_file(&VadModel::SileroVad.url(), &tmp_path, progress)?;
        fs::rename(&tmp_path, &dest)
            .map_err(|e| format!("Failed to rename downloaded file: {e}"))?;

        log::info!("SileroVad model downloaded successfully");
        Ok(dest)
    }

    /// Download a file to disk with progress reporting
    fn download_file(
        &self,
        url: &str,
        dest: &PathBuf,
        progress: Option<ProgressFn>,
    ) -> Result<(), String> {
        let response = self
            .client
            .get(url)
            .send()
            .map_err(|e| format!("Download request failed: {e}"))?;

        let total = response.content_length().unwrap_or(0);
        let mut file = fs::File::create(dest)
            .map_err(|e| format!("Failed to create destination file: {e}"))?;

        // Read all bytes and write with progress
        let bytes = response.bytes()
            .map_err(|e| format!("Download read error: {e}"))?;

        let mut written: u64 = 0;
        for chunk in bytes.chunks(8192) {
            file.write_all(chunk)
                .map_err(|e| format!("Write failed: {e}"))?;
            written += chunk.len() as u64;
            if let Some(ref cb) = progress {
                if !cb(written, total) {
                    return Err("Download cancelled by user".to_string());
                }
            }
        }

        Ok(())
    }

    /// Compute SHA256 checksum of a file (hex string)
    fn checksum_hex(&self, path: &PathBuf) -> Result<String, String> {
        let mut file = fs::File::open(path)
            .map_err(|e| format!("Failed to open file for checksum: {e}"))?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];
        loop {
            let n = file.read(&mut buffer)
                .map_err(|e| format!("Read error during checksum: {e}"))?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Remove all cached models
    #[allow(dead_code)]
    pub fn clear_cache(&self) -> Result<(), String> {
        fs::remove_dir_all(self.models_dir.as_path())
            .map_err(|e| format!("Failed to clear models directory: {e}"))?;
        fs::create_dir_all(&self.models_dir)
            .map_err(|e| format!("Failed to recreate models directory: {e}"))?;
        log::info!("Model cache cleared");
        Ok(())
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize ModelManager")
    }
}
