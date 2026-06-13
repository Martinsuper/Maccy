import { invoke } from "@tauri-apps/api/core";
import { Copy, Trash2, Pin, Play } from "lucide-react";
import { useHistoryStore } from "../stores/historyStore";

export default function HistoryListView() {
  const { filteredItems, selectedId, setSelectedId, removeItem } = useHistoryStore();
  const displayItems = filteredItems();

  // Separate pinned and unpinned items
  const pinnedItems = displayItems.filter(item => item.pin);
  const unpinnedItems = displayItems.filter(item => !item.pin);

  async function handleCopy(item: any) {
    try {
      await invoke("set_clipboard_content", { text: item.title });
    } catch (error) {
      console.error("Failed to copy to clipboard:", error);
    }
  }

  async function handlePaste(item: any) {
    try {
      await invoke("set_clipboard_content", { text: item.title });
      await new Promise(resolve => setTimeout(resolve, 100));
      await invoke("paste_from_clipboard");
    } catch (error) {
      console.error("Failed to paste:", error);
      alert(`Failed to paste: ${error}`);
    }
  }

  async function handleTogglePin(id: string, e: React.MouseEvent) {
    e.stopPropagation();
    try {
      await invoke("toggle_pin", { id });
    } catch (error) {
      console.error("Failed to toggle pin:", error);
    }
  }

  async function handleDelete(id: string, e: React.MouseEvent) {
    e.stopPropagation();
    try {
      await invoke("delete_history_item", { id });
      removeItem(id);
    } catch (error) {
      console.error("Failed to delete item:", error);
    }
  }

  function formatTimestamp(timestamp: number): string {
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return "Just now";
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;
    return date.toLocaleDateString();
  }

  function renderHistoryItem(item: any, isPinned: boolean = false) {
    return (
      <div
        key={item.id}
        onClick={() => setSelectedId(item.id)}
        onDoubleClick={() => handleCopy(item)}
        className={`p-3 border-b border-gray-100 dark:border-gray-800 cursor-pointer transition-colors ${
          selectedId === item.id
            ? "bg-primary-50 dark:bg-primary-900/20"
            : "hover:bg-gray-50 dark:hover:bg-gray-800/50"
        } ${isPinned ? "bg-yellow-50 dark:bg-yellow-900/10" : ""}`}
      >
        <div className="flex items-start justify-between gap-2">
          <div className="flex-1 min-w-0">
            <p className="text-sm text-gray-900 dark:text-gray-100 truncate">
              {item.title}
            </p>
            <div className="flex items-center gap-2 mt-1">
              {item.application && (
                <span className="text-xs text-gray-500 dark:text-gray-400">
                  {item.application}
                </span>
              )}
              <span className="text-xs text-gray-400 dark:text-gray-500">
                {formatTimestamp(item.last_copied_at)}
              </span>
              {item.pin && (
                <span className="text-xs text-yellow-600 dark:text-yellow-400 font-medium">
                  📌 Pinned
                </span>
              )}
              {item.number_of_copies > 1 && (
                <span className="text-xs text-gray-400">
                  ×{item.number_of_copies}
                </span>
              )}
            </div>
          </div>
          <div className="flex items-center gap-1">
            <button
              onClick={(e) => handleTogglePin(item.id, e)}
              className={`p-1.5 hover:bg-yellow-100 dark:hover:bg-yellow-900/20 rounded transition-colors ${
                item.pin ? "text-yellow-600" : "text-gray-400"
              }`}
              title={item.pin ? "Unpin" : "Pin to top"}
            >
              <Pin size={14} className={item.pin ? "fill-current" : ""} />
            </button>
            <button
              onClick={(e) => {
                e.stopPropagation();
                handlePaste(item);
              }}
              className="p-1.5 hover:bg-primary-100 dark:hover:bg-primary-900/20 rounded transition-colors"
              title="Paste (simulates Cmd+V)"
            >
              <Play size={14} className="text-primary-600 dark:text-primary-400" />
            </button>
            <button
              onClick={(e) => {
                e.stopPropagation();
                handleCopy(item);
              }}
              className="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-700 rounded transition-colors"
              title="Copy to clipboard"
            >
              <Copy size={14} className="text-gray-600 dark:text-gray-400" />
            </button>
            <button
              onClick={(e) => handleDelete(item.id, e)}
              className="p-1.5 hover:bg-red-100 dark:hover:bg-red-900/20 rounded transition-colors"
              title="Delete"
            >
              <Trash2 size={14} className="text-red-500" />
            </button>
          </div>
        </div>
      </div>
    );
  }

  if (displayItems.length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center text-gray-400 dark:text-gray-600">
        <div className="text-center">
          <p className="text-lg font-medium">No clipboard history</p>
          <p className="text-sm mt-1">Copy something to get started</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto">
      {/* Pinned Items Section */}
      {pinnedItems.length > 0 && (
        <>
          <div className="px-3 py-2 bg-yellow-50 dark:bg-yellow-900/10 border-b border-yellow-200 dark:border-yellow-800">
            <span className="text-xs font-semibold text-yellow-700 dark:text-yellow-400 uppercase tracking-wide">
              📌 Pinned ({pinnedItems.length})
            </span>
          </div>
          {pinnedItems.map(item => renderHistoryItem(item, true))}
        </>
      )}

      {/* Unpinned Items Section */}
      {unpinnedItems.length > 0 && (
        <>
          {pinnedItems.length > 0 && (
            <div className="px-3 py-2 bg-gray-50 dark:bg-gray-800/50 border-b border-gray-200 dark:border-gray-700">
              <span className="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide">
                Recent ({unpinnedItems.length})
              </span>
            </div>
          )}
          {unpinnedItems.map(item => renderHistoryItem(item, false))}
        </>
      )}
    </div>
  );
}
