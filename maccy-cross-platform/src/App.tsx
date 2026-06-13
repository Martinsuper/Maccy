import { useEffect, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import HeaderView from "./components/HeaderView";
import HistoryListView from "./components/HistoryListView";
import SettingsPanel from "./components/SettingsPanel";
import { useHistoryStore, HistoryItem } from "./stores/historyStore";

function App() {
  const { setItems } = useHistoryStore();
  const [showSettings, setShowSettings] = useState(false);

  const loadHistory = useCallback(async () => {
    try {
      const historyItems = await invoke<HistoryItem[]>("get_history_items");
      setItems(historyItems);
    } catch (error) {
      console.error("Failed to load history:", error);
    }
  }, [setItems]);

  useEffect(() => {
    loadHistory();
    const interval = setInterval(loadHistory, 2000);
    return () => clearInterval(interval);
  }, [loadHistory]);

  // Escape key to close window or dismiss settings
  useEffect(() => {
    function handleKeyDown(e: KeyboardEvent) {
      if (e.key === "Escape") {
        if (showSettings) {
          setShowSettings(false);
        } else {
          invoke("hide_window").catch(() => {});
        }
      }
    }
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [showSettings]);

  return (
    <div className="panel-container">
      <div className="title-bar">
        <span className="title-bar-title">Maccy</span>
        <button
          className="title-bar-btn close"
          onClick={() => invoke("hide_window").catch(() => {})}
          title="Close (Esc)"
        >
          <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
            <path d="M1 1L9 9M9 1L1 9" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
          </svg>
        </button>
      </div>

      <HeaderView onOpenSettings={() => setShowSettings(true)} />
      <HistoryListView />

      {showSettings && (
        <SettingsPanel onClose={() => setShowSettings(false)} />
      )}
    </div>
  );
}

export default App;
