# Noter 🎙️

> Cross-platform bilingual (EN/CN) Speech-to-Text — monorepo.

## Structure

```
noter/
├── desktop/          # Tauri desktop app (Windows, macOS, Linux)
├── mobile/           # React Native mobile app (iOS + Android) — Phase 3
├── whisper-core/     # Shared Rust inference library
└── models/           # Model download scripts + checksums
```

## Quick Start

```bash
cd desktop && pnpm install && pnpm dev
```

## Phases

| Phase | Scope | Status |
|-------|-------|--------|
| Phase 1 | whisper-core + Tauri desktop | ✅ Done |
| Phase 2 | Desktop UX polish | ✅ Done |
| Phase 3 | React Native + FFI | 🔜 Next |
| Phase 4 | Mobile polish | 🔜 |
| Phase 5 | CI/CD for all 5 platforms | 🔜 |

---
Built with 🦦 by the TaTa team.
