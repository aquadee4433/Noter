import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from "@tauri-apps/api/clipboard";


interface TranscriptionEvent {
  text: string;
  is_final: boolean;
  language: string;
}

function App() {
  const [transcript, setTranscript] = useState<string>("");
  const [isListening, setIsListening] = useState(false);
  const [partialText, setPartialText] = useState<string>("");
  const [language, setLanguage] = useState<string>("auto");

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

    // Get initial capture status
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

  return (
    <div className="container">
      <header>
        <h1>🎙️ Noter</h1>
        <span className="lang-badge">{language}</span>
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
