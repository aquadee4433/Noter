import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from "@tauri-apps/api/clipboard";

interface TranscriptionEvent {
  text: string;
  is_final: boolean;
  language: string;
}

interface Settings {
  model: string;
  language: string;
  hotkey: string;
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
  const [settings, setSettings] = useState<Settings>({
    model: "base",
    language: "auto",
    hotkey: "CmdOrCtrl+Shift+S",
  });

  useEffect(() => {
    const unlisten = listen<TranscriptionEvent>(
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

    invoke<boolean>("get_capture_status")
      .then((status) => setIsListening(status))
      .catch(console.error);

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const toggleListening = async () => {
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
  };

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
          </section>
        </div>
      </div>

      {/* Main UI */}
      <header>
        <h1>🎙️ Noter</h1>
        <div className="header-actions">
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
              Click "Start" or press hotkey to begin transcription...
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
