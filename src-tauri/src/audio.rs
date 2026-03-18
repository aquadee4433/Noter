/// Audio capture module using cpal
/// Captures 16kHz mono PCM from system microphone

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct AudioCapture {
    stream: Option<cpal::Stream>,
    running: Arc<AtomicBool>,
}

impl AudioCapture {
    pub fn new() -> Self {
        Self {
            stream: None,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start<F>(&mut self, callback: F) -> Result<(), String>
    where
        F: FnMut(Vec<f32>) + Send + 'static,
    {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("No input device available")?;

        let supported_config = device
            .default_input_config()
            .map_err(|e| format!("Failed to get default input config: {}", e))?;

        // Ensure 16kHz mono
        let sample_rate = cpal::SampleRate(16000);
        let channels = 1;

        let stream_config = cpal::StreamConfig {
            channels,
            sample_rate,
            buffer_size: cpal::BufferSize::Default,
        };

        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);

        let err_fn = |err| eprintln!("Audio stream error: {}", err);

        let sample_format = supported_config.sample_format();
        
        let stream = match sample_format {
            cpal::SampleFormat::F32 => {
                let callback = std::sync::Mutex::new(callback);
                device.build_input_stream(
                    &stream_config,
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        if running.load(Ordering::SeqCst) {
                            let mut cb = callback.lock().unwrap();
                            cb(data.to_vec());
                        }
                    },
                    err_fn,
                    None,
                )
            }
            _ => return Err("Unsupported sample format".to_string()),
        }
        .map_err(|e| format!("Failed to build input stream: {}", e))?;

        stream
            .play()
            .map_err(|e| format!("Failed to start stream: {}", e))?;

        self.stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        self.stream = None;
    }
}

impl Default for AudioCapture {
    fn default() -> Self {
        Self::new()
    }
}
