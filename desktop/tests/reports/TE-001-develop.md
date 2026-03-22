# Noter — Test Execution Report
**Branch:** `develop`
**Date:** 2026-03-19 00:23 GMT+8
**QA:** YvonneTaTaQA
**Commits tested:** `afabf6f` → `4d92ae5`

---

## 🔴 Critical Bugs Found: 4

---

### BUG-01: IPC `start_capture` / `stop_capture` are FAKE

**Severity:** Critical  
**Module:** Tauri IPC / Audio  
**File:** `src-tauri/src/main.rs`  
**Test Case:** TC-IPC-01, TC-IPC-02  

**Description:**  
The `start_capture` and `stop_capture` Tauri commands update the `is_recording` boolean in `AppState` but **never actually instantiate or use the `AudioCapture` struct**. The `audio.rs` module has a complete cpal implementation sitting unused.

```rust
// main.rs — start_capture command:
#[tauri::command]
fn start_capture(state: State<AppState>) -> Result<String, String> {
    let mut is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
    *is_recording = true;
    // TODO: Initialize audio capture   ← BROKEN: AudioCapture is never called!
    Ok("Capture started".to_string())
}
```

**Repro Steps:**
1. Run `npm run tauri dev`
2. Click "Start Capture" button
3. IPC returns `"Capture started"` ✅
4. But no audio device is opened, no cpal stream is created ❌

**Fix Required:** Instantiate `AudioCapture` in `AppState`, call `.start()` in `start_capture`, call `.stop()` in `stop_capture`.

---

### BUG-02: Transcription events never emitted to frontend

**Severity:** Critical  
**Module:** IPC  
**File:** `src-tauri/src/main.rs`, `src/App.tsx`  
**Test Case:** TC-IPC-02  

**Description:**  
`App.tsx` subscribes to `event: "transcription"` but `main.rs` never emits this event. Even if audio were running, no text would ever appear in the UI.

**Repro Steps:**
1. Run the app
2. Start capture (IPC call succeeds)
3. Speak into mic
4. UI status shows "recording" but no transcription ever appears

---

### BUG-03: VAD is a complete stub

**Severity:** High  
**Module:** VAD (silero-vad)  
**File:** `src-tauri/src/vad.rs`  
**Test Case:** TC-VAD-01, TC-VAD-02, TC-VAD-03, TC-VAD-04  

```rust
pub fn detect(&self, audio_chunk: &[f32]) -> bool {
    // TODO: Return true if speech detected
    false   // ← Always returns false, never detects speech
}

pub fn process(&mut self, audio: &[f32]) -> Option<Vec<f32>> {
    // TODO: Buffer audio and return complete utterances
    None    // ← Never returns anything
}
```

**Impact:** Voice activity detection is completely non-functional. Audio capture will never know when the user stops speaking.

---

### BUG-04: STT is a complete stub

**Severity:** High  
**Module:** STT (whisper-rs)  
**File:** `src-tauri/src/stt.rs`  

```rust
pub fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, String> {
    // TODO: Run whisper inference
    Ok(TranscriptionResult {
        text: String::new(),       // ← Always empty
        language: String::new(),   // ← Always empty
    })
}
```

**Impact:** Even if audio is captured and VAD triggers, transcription will always be empty string.

---

### BUG-05: Model selector missing from UI

**Severity:** Medium  
**Module:** Frontend / Settings  
**File:** `src/App.tsx`  

**Description:**  
The settings panel has a language selector but no model selector. Issue #7 spec (settings page with model selector) is incomplete.

**Missing:** Dropdown to select whisper model size (tiny, base, small, medium, large) and to display download progress.

---

## 🟡 Partial / At Risk

### IPC state is disconnected from audio capture
The `AudioCapture` struct in `audio.rs` is well-implemented (16kHz mono, cpal, proper error handling) but it's unreachable from the IPC commands. This is a wiring problem, not a logic problem.

### No backend → frontend audio data flow
Even if BUG-01 were fixed, there's no mechanism to emit captured audio chunks to the frontend for VAD/STT processing. Need either:
- Stream audio events from Rust to TS, or
- Process everything in Rust and emit transcription events

---

## ✅ Working Correctly

- **AudioCapture struct (audio.rs):** cpal setup, 16kHz mono, f32 callback — looks solid
- **System tray:** Full implementation with show/hide/quit ✅
- **Window close → minimize to tray:** Works ✅
- **Frontend IPC wiring:** Commands registered, status syncs on mount ✅
- **UI styling:** Dark theme, coral accent, responsive layout ✅
- **Icons:** All platform formats committed ✅

---

## Summary

| Bug | Severity | Status |
|-----|----------|--------|
| BUG-01: IPC fake capture | 🔴 Critical | Open |
| BUG-02: No transcription events | 🔴 Critical | Open |
| BUG-03: VAD stub | 🟡 High | Open |
| BUG-04: STT stub | 🟡 High | Open |
| BUG-05: Model selector missing | 🟡 Medium | Open |

**Verdict: `develop` has IPC and audio infrastructure — but functional capture/transcription is blocked by 4 bugs.**

---

*QA Report — YvonneTaTaQA — 2026-03-19*
