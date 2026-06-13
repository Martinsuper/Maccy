import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import HeaderView from "./components/HeaderView";
import HistoryListView from "./components/HistoryListView";
import { useHistoryStore, HistoryItem } from "./stores/historyStore";

function App() {
  const { setItems } = useHistoryStore();

  useEffect(() => {
    loadHistory();

    // Poll for new history items every 2 seconds
    const interval = setInterval(() => {
      loadHistory();
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  async function loadHistory() {
    try {
      const historyItems = await invoke<HistoryItem[]>("get_history_items");
      setItems(historyItems);
    } catch (error) {
      console.error("Failed to load history:", error);
    }
  }

  return (
    <div className="container w-full h-full flex flex-col bg-white dark:bg-gray-900">
      <HeaderView />
      <HistoryListView />
    </div>
  );
}

export default App;
