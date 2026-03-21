import { useState, useEffect, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { register, unregister, isRegistered } from "@tauri-apps/plugin-global-shortcut";

interface TranscriptionEvent {
  text: string;
  is_final: boolean;
  language: string;
}

interface Settings {
  model: string;
  language: string;
  hotkey: string;
  minimizeToTray: boolean;
}

const MODELS = [
  { id: "tiny", label: "Tiny (39MB)", description: "Fastest, lowest accuracy" },
  { id: "base", label: "Base (74MB)", description: "Good balance" },
  { id: "small", label: "Small (244MB)", description: "Better accuracy" },
  { id: "medium", label: "Medium (769MB)", description: "High accuracy" },
  { id: "large", label: "Large (1550MB)", description: "Best accuracy" },
];

const LANGUAGES = [
  { id: "auto", label: "Auto-detect" },
  { id: "en", label: "English" },
  { id: "zh", label: "中文" },
];

function App() {
  const [transcript, setTranscript] = useState<string>("");
  const [isListening, setIsListening] = useState(false);
  const [partialText, setPartialText] = useState<string>("");
  const [language, setLanguage] = useState<string>("auto");
  const [showSettings, setShowSettings] = useState(false);
  const [hotkeyError, setHotkeyError] = useState<string | null>(null);
  const [settings, setSettings] = useState<Settings>({
    model: "base",
    language: "auto",
    hotkey: "CmdOrCtrl+Shift+S",
    minimizeToTray: true,
  });

  const registerHotkey = useCallback(async (hotkey: string) => {
    try {
      // Unregister any existing shortcut first
      const alreadyRegistered = await isRegistered(hotkey);
      if (alreadyRegistered) {
        await unregister(hotkey);
      }
      await register(hotkey, () => {
        toggleListening();
      });
      setHotkeyError(null);
    } catch (error: unknown) {
      // Conflict detection: show user feedback
      const msg = error instanceof Error ? error.message : String(error);
      setHotkeyError(`Hotkey conflict: ${msg}`);
      setTimeout(() => setHotkeyError(null), 4000);
    }
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  const toggleListening = useCallback(async () => {
    try {
      if (!isListening) {
        await invoke("start_capture");
      } else {
        await invoke("stop_capture");
      }
      setIsListening(!isListening);
    } catch (error) {
      console.error("Failed to toggle capture:", error);
    }
  }, [isListening]);

  useEffect(() => {
    const unlistenTranscription = listen<TranscriptionEvent>(
      "transcription",
      (event) => {
        if (event.payload.is_final) {
          setTranscript((prev) => prev + event.payload.text + " ");
          setPartialText("");
          setLanguage(event.payload.language);
        } else {
          setPartialText(event.payload.text);
        }
      }
    );

    // Sync capture status from backend
    invoke<boolean>("get_capture_status")
      .then((status) => setIsListening(status))
      .catch(console.error);

    // Listen for tray status changes
    const unlistenStatus = listen<boolean>("capture-status-changed", (event) => {
      setIsListening(event.payload);
    });

    // Register global hotkey
    registerHotkey(settings.hotkey);

    return () => {
      unlistenTranscription.then((fn) => fn());
      unlistenStatus.then((fn) => fn());
      unregister(settings.hotkey).catch(() => {});
    };
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  // Re-register hotkey when it changes
  useEffect(() => {
    registerHotkey(settings.hotkey);
  }, [settings.hotkey, registerHotkey]);

  const copyToClipboard = async () => {
    await writeText(transcript);
  };

  const clearTranscript = () => {
    setTranscript("");
    setPartialText("");
  };

  const updateSetting = <K extends keyof Settings>(key: K, value: Settings[K]) => {
    setSettings((prev) => ({ ...prev, [key]: value }));
  };

  return (
    <div className="container">
      {/* Hotkey conflict toast */}
      {hotkeyError && (
        <div className="toast toast-error">
          ⚠️ {hotkeyError}
        </div>
      )}

      {/* Recording indicator toast */}
      {isListening && (
        <div className="toast toast-recording">
          🎙️ Recording...
        </div>
      )}

      {/* Settings Overlay */}
      {showSettings && (
        <div className="settings-overlay" onClick={() => setShowSettings(false)} />
      )}

      {/* Settings Panel */}
      <div className={`settings-panel ${showSettings ? "open" : ""}`}>
        <div className="settings-header">
          <h2>⚙️ Settings</h2>
          <button className="btn-close" onClick={() => setShowSettings(false)}>
            ✕
          </button>
        </div>

        <div className="settings-body">
          {/* Model Selector */}
          <section className="settings-section">
            <h3>🎙️ STT Model</h3>
            <p className="section-desc">
              Larger models are slower but more accurate. Download models first.
            </p>
            <div className="model-list">
              {MODELS.map((model) => (
                <label key={model.id} className={`model-option ${settings.model === model.id ? "selected" : ""}`}>
                  <input
                    type="radio"
                    name="model"
                    value={model.id}
                    checked={settings.model === model.id}
                    onChange={() => updateSetting("model", model.id)}
                  />
                  <span className="model-label">{model.label}</span>
                  <span className="model-desc">{model.description}</span>
                </label>
              ))}
            </div>
          </section>

          {/* Language */}
          <section className="settings-section">
            <h3>🌐 Language</h3>
            <div className="lang-options">
              {LANGUAGES.map((lang) => (
                <button
                  key={lang.id}
                  className={`btn-lang ${settings.language === lang.id ? "active" : ""}`}
                  onClick={() => updateSetting("language", lang.id)}
                >
                  {lang.label}
                </button>
              ))}
            </div>
          </section>

          {/* Hotkey */}
          <section className="settings-section">
            <h3>⌨️ Global Hotkey</h3>
            <div className="hotkey-display">
              <kbd>{settings.hotkey}</kbd>
              <span>Toggle recording</span>
            </div>
            <p className="hotkey-hint">
              Works even when app is minimized to tray.
            </p>
            {hotkeyError && (
              <p className="hotkey-error">{hotkeyError}</p>
            )}
          </section>

          {/* Behavior */}
          <section className="settings-section">
            <h3>🪟 Window Behavior</h3>
            <label className="toggle-option">
              <input
                type="checkbox"
                checked={settings.minimizeToTray}
                onChange={(e) => updateSetting("minimizeToTray", e.target.checked)}
              />
              <span>Minimize to system tray on close</span>
            </label>
          </section>
        </div>
      </div>

      {/* Main UI */}
      <header>
        <h1>🎙️ Noter</h1>
        <div className="header-actions">
          <span className={`status-dot ${isListening ? "recording" : "idle"}`} />
          <span className="lang-badge">{language}</span>
          <button
            className="btn-icon"
            onClick={() => setShowSettings(true)}
            title="Settings"
          >
            ⚙️
          </button>
        </div>
      </header>

      <main>
        <div className="transcript-box">
          <p className="final-text">{transcript}</p>
          <p className="partial-text">{partialText}</p>
          {!transcript && !partialText && (
            <p className="placeholder">
              Click "Start" or press <kbd>{settings.hotkey}</kbd> to begin transcription...
            </p>
          )}
        </div>
      </main>

      <footer>
        <button
          className={`btn-primary ${isListening ? "listening" : ""}`}
          onClick={toggleListening}
        >
          {isListening ? "⏹ Stop" : "🎙 Start"}
        </button>
        <button className="btn-secondary" onClick={copyToClipboard}>
          📋 Copy
        </button>
        <button className="btn-secondary" onClick={clearTranscript}>
          🗑 Clear
        </button>
      </footer>
    </div>
  );
}

export default App;
