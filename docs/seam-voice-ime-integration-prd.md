# PRD: Seam Voice IME Integration

- **Project:** Seam × Noter
- **Feature:** Voice IME / Dictation Input for Seam
- **Author:** Yvonne
- **Status:** Draft v1
- **Date:** 2026-03-24

---

## 1. Overview

### 1.1 Background
Noter already provides the core local speech pipeline:
- audio capture
- voice activity detection (VAD)
- speech-to-text (STT)
- model management
- hotkey-driven transcription UX

Seam is the long-term messaging surface for:
- human-to-human messaging
- human-to-agent messaging
- agent-to-agent messaging
- routed, auditable, context-aware communication

The opportunity is to integrate **Noter's voice IME capability** directly into Seam so voice becomes a **native message input modality**, not a separate tool.

### 1.2 Vision
A user can press a shortcut in Seam, speak naturally, and have speech transformed into:
- a text draft in the active chat composer
- optionally, a structured command
- eventually, an agent instruction routed via Seam

This makes voice a first-class input channel for human and agent messaging.

---

## 2. Problem Statement

Today:
- Noter's dictation exists as a separate capability
- Seam has text-first message composition
- there is no native, reusable voice input layer inside Seam

This causes:
- duplicated UX between apps
- weak integration with conversation context
- difficulty routing voice-derived content into chat, agents, and structured actions
- extra maintenance if voice remains external

We need a Seam-native voice input architecture that reuses Noter's proven speech engine.

---

## 3. Goals

### 3.1 Primary Goal
Integrate Noter's voice IME into Seam so users can dictate into the active Seam composer using a hotkey.

### 3.2 Secondary Goals
- reuse `whisper-core` as a shared transcription engine
- support bilingual EN/CN dictation
- preserve local-first, privacy-preserving processing
- enable future voice commands and agent instructions
- avoid duplicating speech logic across products

### 3.3 Non-Goals
This phase does **not** aim to:
- replace system-wide IMEs
- auto-send messages by default
- support cloud transcription as the primary mode
- build full voice calling / real-time conversation
- add autonomous action execution from voice without confirmation

---

## 4. Users

### 4.1 Primary Users
- power users sending many chat messages
- bilingual EN/CN users
- users who want hands-light interaction
- users directing agents in Seam

### 4.2 Secondary Users
- users on mobile/desktop who prefer speech input
- accessibility users
- multitasking users while coding, browsing, or working

---

## 5. User Stories

### 5.1 MVP Stories
1. As a user, I can press a hotkey in Seam and start dictating.
2. As a user, I can see partial/final transcript appear in the active composer.
3. As a user, I can edit the dictated message before sending.
4. As a user, I can choose language/model/hotkey in settings.
5. As a user, I can see when the microphone is active.
6. As a user, I can recover gracefully if speech recognition fails.

### 5.2 Phase 2 Stories
1. As a user, I can use voice to draft bilingual content naturally.
2. As a user, I can keep dictation local without sending audio externally.
3. As a user, I can resume dictation after interruptions.

### 5.3 Future Stories
1. As a user, I can say "send to Alice: meeting moved to 3" and Seam parses it as a command.
2. As a user, I can say "reply: sounds good" and Seam prepares a structured reply.
3. As a user, I can say "ask BE to review router design" and Seam routes it to an agent.

---

## 6. Product Principles

1. **Dictation first, commands later**  
   Default behavior should produce editable draft text, not actions.

2. **Local by default**  
   Audio and transcription should stay on-device unless explicitly configured otherwise.

3. **Visible recording state**  
   Users must always know when the mic is active.

4. **Confirm before action**  
   Any interpreted command should require review/confirmation unless explicitly enabled later.

5. **One engine, many surfaces**  
   Speech logic belongs in shared infrastructure (`whisper-core`), not duplicated per app.

---

## 7. Scope

### 7.1 MVP Scope
- Seam desktop integration
- hotkey to start/stop dictation
- local transcription via shared speech engine
- partial + final transcript events
- active composer insertion
- settings for language/model/hotkey
- recording indicator
- error states
- manual send only

### 7.2 Out of MVP
- system-wide dictation outside Seam
- auto-send after transcription
- voice command execution
- mobile app dictation integration
- cloud STT fallback
- conversation summarization from raw audio

---

## 8. Solution Overview

### 8.1 High-Level Flow

```text
User presses hotkey
→ Seam opens voice session
→ whisper-core starts capture + VAD + STT
→ partial transcript events stream to Seam UI
→ final transcript inserted into composer
→ user edits
→ user sends manually
→ Seam routes message normally
```

### 8.2 Integration Model
Noter's value becomes a reusable speech subsystem, while Seam remains the orchestration layer.

#### Noter / whisper-core owns
- capture
- VAD
- STT
- models
- transcript events

#### Seam owns
- active chat context
- composer state
- routing
- send / ack / history
- permissions UI
- metadata and audit

---

## 9. Functional Requirements

### 9.1 Voice Session Lifecycle
The system must:
- start a voice session on hotkey press
- stop on hotkey release or toggle action
- expose state: `idle` / `listening` / `transcribing` / `completed` / `error`
- cancel gracefully if user aborts

### 9.2 Transcription
The system must:
- transcribe spoken input locally
- emit partial and final transcript events
- support EN, ZH, and auto-detect
- support configurable model size
- return transcript text to the Seam composer

### 9.3 Composer Integration
The system must:
- insert transcript into the currently active chat composer
- preserve existing draft text
- support append mode for repeated dictation
- allow manual editing after transcript insertion

### 9.4 Settings
The system must support:
- language preference
- model selection
- hotkey selection
- auto-copy toggle (optional compatibility behavior)
- punctuation/formatting preferences (future-ready)

### 9.5 UI Feedback
The system must show:
- recording indicator
- processing state
- transcript preview
- error/toast states
- confirmation state if command parsing is later enabled

### 9.6 Privacy & Permissions
The system must:
- request microphone permission explicitly
- visibly indicate active recording
- keep audio local by default
- not store raw audio unless explicitly enabled later

---

## 10. UX Specification

### 10.1 MVP User Journey
1. User opens any Seam conversation.
2. User presses configured dictation hotkey.
3. Recording indicator appears.
4. User speaks.
5. Partial transcript appears inline or near the composer.
6. User stops speaking or releases the hotkey.
7. Final transcript lands in the composer.
8. User edits if needed.
9. User sends manually.

### 10.2 UI Components
- mic button / status chip
- recording indicator
- inline transcript preview
- composer insertion logic
- settings screen section for Voice Input

### 10.3 Error UX
Examples:
- mic permission denied
- model unavailable
- no speech detected
- transcription timeout
- model loading failed

Each should provide:
- plain-language error
- retry path
- no silent failure

---

## 11. Voice Modes

### 11.1 Mode A — Dictation Mode (MVP)
Purpose: produce text draft only.

Behavior:
- no action parsing
- no send automation
- safest default

### 11.2 Mode B — Command Mode (Future)
Purpose: transform voice into a structured Seam action.

Examples:
- "send to Alice: on my way"
- "reply: sounds good"
- "ask QA to validate build"

Behavior:
- parse into structured action
- show confirmation UI
- require explicit user confirmation

### 11.3 Mode C — Agent Instruction Mode (Future)
Purpose: speak instructions to agents.

Examples:
- "ask BE to summarize the router design"
- "tell PM to create a milestone for mobile polish"

Behavior:
- transcript routed into Seam agent messaging flow
- preserves source metadata
- may use templates/prompts

---

## 12. Technical Architecture

### 12.1 Components

#### Shared Engine
**`whisper-core`**
- audio capture
- VAD
- transcription
- model management
- transcript event emission

#### Seam Desktop Adapter
**`seam-desktop-voice`** or an equivalent feature module
- hotkey binding
- session management
- event subscription
- bridge into composer state

#### Seam App Layer
- compose store
- active conversation context
- message metadata attachment
- UI rendering

### 12.2 Event Contract

```ts
type VoiceSessionEvent =
  | { type: "started"; sessionId: string }
  | { type: "partial"; sessionId: string; text: string }
  | { type: "final"; sessionId: string; text: string; language?: string; confidence?: number }
  | { type: "error"; sessionId: string; error: string }
  | { type: "stopped"; sessionId: string };
```

This contract should be stable, minimal, and UI-friendly.

### 12.3 Suggested Message Metadata

```json
{
  "inputSource": "voice",
  "language": "en",
  "model": "base",
  "transcriptConfidence": 0.91
}
```

Optional later additions:
- `commandParsed`
- `commandType`
- `durationMs`

---

## 13. Repo / Code Organization

### Recommended structure in Seam
- `libs/whisper-core/` or shared dependency reference
- `apps/desktop/src/features/voice/`
- `apps/desktop/src/features/compose/`
- `docs/voice-ime-prd.md`

### Minimum modules
- `voiceSessionManager`
- `voiceHotkeyRegistry`
- `voiceComposerBridge`
- `voiceSettingsStore`
- `voiceUIState`

---

## 14. Data Model

### 14.1 Settings
```json
{
  "language": "auto",
  "model_size": "base",
  "hotkey": "CmdOrCtrl+Shift+Space",
  "auto_copy": false
}
```

### 14.2 Session State
```json
{
  "sessionId": "uuid",
  "status": "listening",
  "partialText": "",
  "finalText": "",
  "language": "auto",
  "startedAt": 0,
  "endedAt": 0
}
```

---

## 15. Success Metrics

### MVP Success Metrics
- user can dictate into the Seam composer in under 5 seconds
- transcription success rate > 90% in normal conditions
- hotkey-to-listening startup < 500ms target
- crash-free voice sessions > 99%
- most dictated messages can still be lightly edited before send

### Quality Metrics
- EN/CN mixed dictation works acceptably in most tests
- no silent microphone activation incidents
- no unintended sends from the dictation flow

---

## 16. Acceptance Criteria

### MVP Acceptance Criteria
1. User can start/stop dictation with a configured hotkey.
2. User sees microphone-active state while recording.
3. Seam receives partial/final transcript from the local engine.
4. Final transcript inserts into the active composer.
5. Existing draft is preserved and appended correctly.
6. Settings persist across restart.
7. CI passes on supported desktop targets.
8. No auto-send occurs in MVP.
9. Audio is processed locally by default.
10. Error states are surfaced to the user.

---

## 17. Risks

### 17.1 Latency Risk
If transcription is too slow, users will not trust it.

Mitigation:
- partial transcript
- a fast default model
- optional model preload on startup

### 17.2 Command Misfire Risk
Users may think dictation equals command execution.

Mitigation:
- separate dictation vs command UX
- manual send default
- confirmation gates

### 17.3 Platform Shortcut Conflict
Global shortcuts may conflict with the OS or other apps.

Mitigation:
- validation
- conflict detection
- fallback shortcuts

### 17.4 Privacy Risk
Voice data is sensitive.

Mitigation:
- local-first processing
- no background capture
- visible recording indicator
- explicit permission prompts

### 17.5 Architecture Duplication
Speech logic may fork across apps.

Mitigation:
- shared engine ownership
- no copy-pasted STT stacks inside Seam

---

## 18. Dependencies

### Internal
- `whisper-core`
- Seam desktop compose UI
- Seam settings panel
- Seam desktop shortcut system

### External
- microphone permissions
- `whisper-rs` / model files
- VAD runtime
- desktop Tauri plugins as needed

---

## 19. Rollout Plan

### Phase 1 — Internal Alpha
- desktop-only
- dictation into active chat
- manual send
- team testing

### Phase 2 — Dogfood
- EN/CN mixed speech validation
- settings polish
- hotkey reliability
- longer message testing

### Phase 3 — Broader Beta
- command mode experiments
- agent instruction mode prototype
- telemetry / feedback loop

---

## 20. Milestones

### Milestone 1 — Shared Engine Stabilization
- finalize `whisper-core` public API
- define event model
- finalize model management contract

### Milestone 2 — Seam Desktop Dictation MVP
- hotkey
- session manager
- composer insertion
- settings
- UI feedback

### Milestone 3 — QA Hardening
- noisy-room tests
- bilingual tests
- long-session tests
- conflict/error tests

### Milestone 4 — Command Layer
- parse / confirm / send
- structured voice action model

---

## 21. Team Breakdown

### PM
- scope phases
- define acceptance criteria
- track milestone dependencies
- keep dictation and command mode separated in planning

### BE
- expose `whisper-core` session/event API
- integrate engine with the Seam desktop backend
- define message metadata contract

### FE
- compose integration
- transcript preview
- voice UI states
- settings UX

### Design
- microphone / recording states
- transcript preview treatment
- command confirmation flow
- error UX

### QA
- EN/CN mixed speech tests
- device / microphone matrix
- shortcut conflicts
- interruption / recovery tests
- privacy-state checks

---

## 22. Proposed GitHub Epics / Issues

### Epic A — Voice Engine Integration
- expose `whisper-core` API for Seam
- transcript event contract
- model lifecycle integration

### Epic B — Desktop Voice UX
- hotkey session control
- recording indicator
- transcript preview
- composer insertion

### Epic C — Settings & Persistence
- language/model/hotkey settings
- persistence
- validation

### Epic D — QA & Hardening
- noisy environment test plan
- bilingual test plan
- long transcript robustness
- permission and privacy tests

### Epic E — Future Command Layer
- voice command grammar
- confidence thresholds
- confirmation UI
- agent routing integration

---

## 23. Recommended First Sprint

### Sprint Goal
**User can dictate a message into the active Seam conversation and edit/send it normally.**

### Sprint Deliverables
- Seam desktop hotkey
- voice session manager
- transcript streaming from `whisper-core`
- final transcript insertion into the composer
- basic settings page
- recording indicator
- QA smoke matrix

---

## 24. Open Questions

1. Should `whisper-core` live physically in the Seam repo, or remain shared from Noter first?
2. Should partial transcript render inline in the composer or in a floating overlay?
3. Should repeated dictation append to the existing draft or replace a selected region?
4. What is the default model for Seam: `base`, `small`, or configurable per platform?
5. Should agent instruction mode be a separate toggle or inferred by active conversation type?

---

## 25. Recommendation

Recommended direction:
- use Noter as the proven engine source
- use `whisper-core` as the reusable speech layer
- integrate into Seam desktop first
- ship dictation MVP before any command automation
- treat voice as a Seam-native input mode, not a bolt-on external app

This gives the cleanest architecture and the lowest long-term maintenance burden.
