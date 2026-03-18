/// Audio capture module using cpal
/// Captures 16kHz mono PCM from system microphone

pub struct AudioCapture {
    // TODO: Implement cpal audio capture
}

impl AudioCapture {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&mut self) -> Result<(), String> {
        // TODO: Initialize cpal stream at 16kHz mono
        Ok(())
    }

    pub fn stop(&mut self) {
        // TODO: Stop audio capture
    }
}
