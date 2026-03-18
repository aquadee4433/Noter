export interface TranscriptionResult {
  text: string;
  isFinal: boolean;
  language: string;
  timestamp: number;
}

export interface ModelInfo {
  name: string;
  size: string;
  path: string;
  downloaded: boolean;
}

export interface AppConfig {
  model: string;
  hotkey: string;
  autoStart: boolean;
  language: "auto" | "en" | "zh";
}
