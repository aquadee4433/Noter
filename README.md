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

## Downloads

### Latest Release (v0.1.0 — Sprint 1)
Go to the **Actions** tab → **Release Build** workflow → **Run workflow** → select branch `main`.

| Platform | Artifact | Download Location |
|----------|----------|-------------------|
| 🍎 macOS | `.dmg` | Actions → Release Build → `noter-macos` |
| 🐧 Linux | `.deb` / `.AppImage` | Actions → Release Build → `noter-linux` |
| 🪟 Windows | `.msi` / `.exe` | Actions → Release Build → `noter-windows` |

> **Note:** Builds are bundled without whisper models (~75MB–3GB). On first run, Noter will automatically download the selected model.

## Tech Stack

| Component | Technology |
|-----------|------------|
| App Framework | Tauri (Rust) |
| Speech Engine | whisper.cpp + whisper-rs |
| Audio Capture | cpal (Rust) |
| VAD | Energy-based (32ms chunks) |
| Frontend | TypeScript + React |

## Whisper Model Selection

| Model | Size | English | Chinese | Speed |
|-------|------|---------|---------|-------|
| tiny | ~75MB | OK | Poor | ⚡⚡⚡⚡ |
| base | ~150MB | Good | Mediocre | ⚡⚡⚡ |
| small | ~500MB | Great | Good | ⚡⚡ |
| **medium** (default) | ~1.5GB | Excellent | Great | ⚡ |
| large | ~3GB | Excellent | Excellent | ⚡ |

## Development

### Prerequisites

- Rust (latest stable)
- Node.js (>= 18)
- npm or pnpm

### Setup

```bash
git clone https://github.com/aquadee4433/Noter.git
cd Noter
npm install
npm run tauri dev    # dev mode with hot reload
npm run tauri build  # production build
```

### CI / Release Build

Release builds run automatically via GitHub Actions on push to `main`. To trigger manually:

1. Go to **Actions** → **Release Build** → **Run workflow**
2. Select branch: `main`
3. Optionally specify version tag (e.g. `v0.1.0`)
4. Download artifacts from each platform job

## Performance Targets

| Metric | Target |
|--------|--------|
| Transcription latency | < 3s |
| English WER | < 8% |
| Chinese CER | < 12% |
| Memory usage | < 2 GB (medium model) |
| Installer size | < 30 MB |
| Cold start | < 5s |

## Architecture

```
┌─────────────────────────────────────────┐
│              Tauri (Rust)                │
│  ┌──────────┐  ┌─────────┐  ┌─────────┐  │
│  │   cpal   │→ │   VAD   │→ │ whisper │  │
│  │ Mic 16kHz│  │EnergyBrk│  │   STT   │  │
│  └──────────┘  └─────────┘  └────┬────┘  │
│                                   │       │
│              Tauri IPC Events     │       │
├───────────────┬───────────────────┼───────┤
│               ▼                   │       │
│         React Frontend             │       │
│  🎙 Start/Stop │ 📋 Copy │ ⚙ Settings     │
└───────────────────────────────────────────┘
```

## License

MIT
