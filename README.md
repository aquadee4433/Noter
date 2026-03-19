# Noter 🎙️

Cross-platform bilingual (EN/CN) speech-to-text desktop plugin powered by whisper.cpp.

## Features

- 🌐 **Bilingual**: English + Chinese (Mandarin) with auto code-switching
- ⚡ **Real-time**: Low-latency chunked streaming transcription
- 🔒 **Offline**: All inference runs locally, zero network dependency
- 🖥️ **Cross-platform**: Windows, macOS, Linux via Tauri
- 📋 **Clipboard**: One-click copy to system clipboard
- ⌨️ **Hotkey**: Global keyboard shortcut activation
- 🎯 **System Tray**: Background operation support

## Tech Stack

| Component | Technology |
|-----------|------------|
| App Framework | Tauri (Rust) |
| Speech Engine | whisper.cpp + whisper-rs |
| Audio Capture | cpal (Rust) |
| VAD | silero-vad |
| Frontend | TypeScript + React |

## Whisper Model Selection

| Model | Size | English | Chinese | RT Factor |
|-------|------|---------|---------|-----------|
| tiny | ~75MB | OK | Poor | ~10x |
| base | ~150MB | Good | Mediocre | ~7x |
| small | ~500MB | Great | Good | ~3x |
| **medium** (default) | ~1.5GB | Excellent | Great | ~1x |
| large-v3 | ~3GB | Excellent | Excellent | ~0.5x |

## Downloads

> **Sprint 1** builds are available via GitHub Actions CI. Download the artifacts from the latest workflow run, or trigger a release build manually.

### Manual Release Build

1. Go to **Actions** tab → **Release Build** workflow
2. Click **Run workflow** → select branch `main`
3. Wait for all 3 platform builds to complete
4. Download artifacts from each job:
   - **macOS**: `.dmg` installer
   - **Linux**: `.deb` or `.AppImage`
   - **Windows**: `.msi` or `.exe` (NSIS)

### Latest Release Artifacts

| Platform | Artifact | Status |
|----------|----------|--------|
| macOS | `.dmg` | 🔨 CI |
| Linux | `.deb` / `.AppImage` | 🔨 CI |
| Windows | `.msi` / `.exe` | 🔨 CI |

## Development

### Prerequisites

- Rust (latest stable)
- Node.js (>= 18)
- pnpm

### Setup

```bash
# Clone
git clone https://github.com/aquadee4433/Noter.git
cd Noter

# Install frontend deps
pnpm install

# Run in dev mode
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

## Performance Targets

- Transcription latency: < 3 seconds
- English WER: < 8%
- Chinese CER: < 12%
- Memory usage: < 2 GB (with medium model)
- Installer size: < 30 MB (excluding model)
- Cold start time: < 5 seconds

## Architecture

```
┌─────────────────────────────────────────────────┐
│                 Tauri (Rust)                     │
│  ┌───────────┐  ┌───────────┐  ┌─────────────┐  │
│  │   cpal    │→ │ silero-vad│→ │ whisper.cpp │  │
│  │ Mic Input │  │   VAD     │  │  Inference   │  │
│  └───────────┘  └───────────┘  └──────┬──────┘  │
│                                       │         │
│              ┌────────────────────────┘         │
│              ▼                                   │
│        Tauri IPC Events                         │
│              │                                   │
├──────────────┼───────────────────────────────────┤
│              ▼                                   │
│     React Frontend (TypeScript)                  │
│  ┌─────────────────────────────────────────┐    │
│  │  Transcription UI / Settings / Tray     │    │
│  └─────────────────────────────────────────┘    │
└─────────────────────────────────────────────────┘
```

## Downloads

> ⚠️ Release artifacts are built by CI on every merge to `main`.  
> Check the [Actions tab](https://github.com/aquadee4433/Noter/actions) for the latest build status.

| Platform | Artifact | Status |
|----------|----------|--------|
| macOS 🍎 | `.dmg` | CI `build-macos` |
| Linux 🐧 | `.deb` / `.AppImage` | CI `build-linux` |
| Windows 🪟 | `.msi` / `.exe` | CI `build-windows` |

**Latest release:** [v0.1.0](https://github.com/aquadee4433/Noter/releases/tag/v0.1.0)

## License

MIT
