# Noter Test Matrix — Issue #9

## Cross-Platform Coverage

| Platform | OS Version | Architecture | Audio API |
|----------|------------|---------------|-----------|
| macOS    | 13+ (Ventura+) | arm64, x64 | CoreAudio |
| Windows  | 10/11 | x64 | WASAPI |
| Linux    | Ubuntu 22.04+ | x64, arm64 | ALSA/PulseAudio |

## Feature Test Matrix

### 1. Audio Capture (cpal)
| Test Case | Platform | Input Source | Expected Result |
|-----------|----------|--------------|-----------------|
| TC-AC-01 | macOS | Built-in mic | Captures audio |
| TC-AC-02 | macOS | USB headset | Switches input |
| TC-AC-03 | Windows | Built-in mic | Captures audio |
| TC-AC-04 | Windows | Bluetooth audio | Handles latency |
| TC-AC-05 | Linux | ALSA default | Captures audio |
| TC-AC-06 | Linux | PulseAudio | Captures audio |
| TC-AC-07 | All | No input device | Graceful error |

### 2. Voice Activity Detection (silero-vad)
| Test Case | Platform | Scenario | Expected Result |
|-----------|----------|----------|-----------------|
| TC-VAD-01 | All | Speech detected | Triggers STT |
| TC-VAD-02 | All | Silence (>2s) | Stops capture |
| TC-VAD-03 | All | Background noise | Filters correctly |
| TC-VAD-04 | All | Music playing | Doesn't trigger |

### 3. Speech-to-Text (whisper.cpp)
| Test Case | Platform | Language | Audio Quality | Expected Result |
|-----------|----------|----------|---------------|-----------------|
| TC-STT-01 | All | English | Clear | >95% accuracy |
| TC-STT-02 | All | Chinese (CN) | Clear | >95% accuracy |
| TC-STT-03 | All | English | Noisy | >80% accuracy |
| TC-STT-04 | All | Chinese (CN) | Noisy | >80% accuracy |
| TC-STT-05 | All | Mixed EN/CN | Clear | Detects language |
| TC-STT-06 | All | Empty audio | N/A | Returns empty |
| TC-STT-07 | All | Very long speech | Clear | Handles >5min |

### 4. Tauri IPC Integration
| Test Case | Platform | Action | Expected Result |
|-----------|----------|--------|-----------------|
| TC-IPC-01 | All | Frontend → Backend | Event reaches Rust |
| TC-IPC-02 | All | Backend → Frontend | UI updates |
| TC-IPC-03 | All | Error propagation | Error displayed |
| TC-IPC-04 | All | Rapid events | No memory leak |

### 5. Settings Page
| Test Case | Platform | Setting | Expected Result |
|-----------|----------|---------|-----------------|
| TC-SET-01 | All | Language switch EN | UI updates to English |
| TC-SET-02 | All | Language switch CN | UI updates to Chinese |
| TC-SET-03 | All | Audio input select | Changes device |
| TC-SET-04 | All | Theme toggle | Dark/Light switches |
| TC-SET-05 | All | Save settings | Persists on restart |

### 6. System Tray
| Test Case | Platform | Action | Expected Result |
|-----------|----------|--------|-----------------|
| TC-TRAY-01 | macOS | Minimize to tray | Icon appears |
| TC-TRAY-02 | Windows | Minimize to tray | Icon appears |
| TC-TRAY-03 | Linux | Minimize to tray | Icon appears |
| TC-TRAY-04 | All | Click tray icon | Window restores |
| TC-TRAY-05 | All | Right-click menu | Shows options |

### 7. Clipboard Integration
| Test Case | Platform | Action | Expected Result |
|-----------|----------|--------|-----------------|
| TC-CLIP-01 | All | Copy transcript | Text in clipboard |
| TC-CLIP-02 | All | Auto-copy toggle | Respects setting |

## Performance Targets
| Metric | Target |
|--------|--------|
| Cold start | <3s |
| Audio latency | <200ms |
| STT processing | <2x realtime |
| Memory usage | <500MB |

## Bug Severity Classification
- **Critical**: Crash, data loss, security issue
- **High**: Feature broken, workarounds difficult
- **Medium**: Feature partially works, workaround exists
- **Low**: Cosmetic, UX friction

## Test Reports Location
`/tests/reports/`

---
*Created: 2026-03-18*
*QA: YvonneTaTaQA*
