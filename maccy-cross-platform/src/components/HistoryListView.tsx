import { invoke } from "@tauri-apps/api/core";
import { Pin, Trash2, ClipboardPaste } from "lucide-react";
import { useHistoryStore } from "../stores/historyStore";

export default function HistoryListView() {
  const { filteredItems, selectedId, setSelectedId, removeItem, items } =
    useHistoryStore();
  const displayItems = filteredItems();

  const pinnedItems = displayItems.filter((item) => item.pin);
  const unpinnedItems = displayItems.filter((item) => !item.pin);

  async function handleCopy(item: HistoryItem) {
    try {
      await invoke("set_clipboard_content", { text: item.title });
    } catch (error) {
      console.error("Failed to copy:", error);
    }
  }

  async function handlePaste(item: HistoryItem) {
    try {
      await invoke("set_clipboard_content", { text: item.title });
      await new Promise((resolve) => setTimeout(resolve, 100));
      await invoke("paste_from_clipboard");
      // Hide window after paste
      await invoke("hide_window").catch(() => {});
    } catch (error) {
      console.error("Failed to paste:", error);
    }
  }

  async function handleTogglePin(id: string) {
    try {
      await invoke("toggle_pin", { id });
    } catch (error) {
      console.error("Failed to toggle pin:", error);
    }
  }

  async function handleDelete(id: string) {
    try {
      await invoke("delete_history_item", { id });
      removeItem(id);
    } catch (error) {
      console.error("Failed to delete:", error);
    }
  }

  async function handleClearAll() {
    try {
      await invoke("clear_history");
    } catch (error) {
      console.error("Failed to clear history:", error);
    }
  }

  function formatTime(timestamp: number): string {
    const now = Date.now() / 1000;
    const diff = now - timestamp;
    if (diff < 60) return "刚刚";
    if (diff < 3600) return `${Math.floor(diff / 60)} 分钟前`;
    if (diff < 86400) return `${Math.floor(diff / 3600)} 小时前`;
    if (diff < 604800) return `${Math.floor(diff / 86400)} 天前`;
    return new Date(timestamp * 1000).toLocaleDateString("zh-CN", {
      month: "short",
      day: "numeric",
    });
  }

  function getAppName(app?: string): string {
    if (!app) return "";
    // Extract just the app name from bundle ID or path
    const parts = app.split(".");
    return parts[parts.length - 1] || app;
  }

  // Empty state
  if (items.length === 0) {
    return (
      <div className="empty-state">
        <div className="empty-state-icon">
          <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="1.5"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <rect x="8" y="2" width="8" height="4" rx="1" ry="1" />
            <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
          </svg>
        </div>
        <div className="empty-state-title">暂无剪贴板历史</div>
        <div className="empty-state-desc">复制内容后将自动记录在这里</div>
      </div>
    );
  }

  // No search results
  if (displayItems.length === 0) {
    return (
      <div className="empty-state">
        <div className="empty-state-title">未找到匹配项</div>
        <div className="empty-state-desc">尝试其他搜索关键词</div>
      </div>
    );
  }

  return (
    <>
      <div className="history-list">
        {/* Pinned section */}
        {pinnedItems.length > 0 && (
          <>
            <div className="section-header pinned">
              <Pin size={10} />
              <span>已置顶 · {pinnedItems.length}</span>
            </div>
            {pinnedItems.map((item) => (
              <HistoryItemRow
                key={item.id}
                item={item}
                isSelected={selectedId === item.id}
                onSelect={() => setSelectedId(item.id)}
                onDoubleClick={() => handleCopy(item)}
                onPaste={() => handlePaste(item)}
                onTogglePin={() => handleTogglePin(item.id)}
                onDelete={() => handleDelete(item.id)}
                formatTime={formatTime}
                getAppName={getAppName}
              />
            ))}
          </>
        )}

        {/* Recent section */}
        {unpinnedItems.length > 0 && (
          <>
            {pinnedItems.length > 0 && (
              <div className="section-header">
                <span>最近 · {unpinnedItems.length}</span>
              </div>
            )}
            {unpinnedItems.map((item) => (
              <HistoryItemRow
                key={item.id}
                item={item}
                isSelected={selectedId === item.id}
                onSelect={() => setSelectedId(item.id)}
                onDoubleClick={() => handleCopy(item)}
                onPaste={() => handlePaste(item)}
                onTogglePin={() => handleTogglePin(item.id)}
                onDelete={() => handleDelete(item.id)}
                formatTime={formatTime}
                getAppName={getAppName}
              />
            ))}
          </>
        )}
      </div>

      {/* Footer */}
      <div className="panel-footer">
        <span className="footer-stats">
          {items.length} 条记录
          {pinnedItems.length > 0 && ` · ${pinnedItems.length} 已置顶`}
        </span>
        <button className="footer-btn" onClick={handleClearAll}>
          清空全部
        </button>
      </div>
    </>
  );
}

// ─── Individual history item row ───

interface HistoryItem {
  id: string;
  title: string;
  content_type: string;
  application?: string;
  first_copied_at: number;
  last_copied_at: number;
  number_of_copies: number;
  pin?: string;
}

interface HistoryItemRowProps {
  item: HistoryItem;
  isSelected: boolean;
  onSelect: () => void;
  onDoubleClick: () => void;
  onPaste: () => void;
  onTogglePin: () => void;
  onDelete: () => void;
  formatTime: (ts: number) => string;
  getAppName: (app?: string) => string;
}

function HistoryItemRow({
  item,
  isSelected,
  onSelect,
  onDoubleClick,
  onPaste,
  onTogglePin,
  onDelete,
  formatTime,
  getAppName,
}: HistoryItemRowProps) {
  const className = [
    "history-item",
    isSelected ? "selected" : "",
    item.pin ? "pinned" : "",
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <div
      className={className}
      onClick={onSelect}
      onDoubleClick={onDoubleClick}
    >
      <div className="history-item-content">
        <div className="history-item-text">{item.title}</div>
        <div className="history-item-meta">
          {item.application && (
            <span className="app">{getAppName(item.application)}</span>
          )}
          <span>{formatTime(item.last_copied_at)}</span>
          {item.pin && <span className="pin-badge">📌</span>}
          {item.number_of_copies > 1 && (
            <span className="count">×{item.number_of_copies}</span>
          )}
        </div>
      </div>

      <div className="item-actions">
        <button
          className={`action-btn pin ${item.pin ? "active" : ""}`}
          onClick={(e) => {
            e.stopPropagation();
            onTogglePin();
          }}
          title={item.pin ? "取消置顶" : "置顶"}
        >
          <Pin size={13} fill={item.pin ? "currentColor" : "none"} />
        </button>
        <button
          className="action-btn copy"
          onClick={(e) => {
            e.stopPropagation();
            onPaste();
          }}
          title="粘贴 (⌘V)"
        >
          <ClipboardPaste size={13} />
        </button>
        <button
          className="action-btn delete"
          onClick={(e) => {
            e.stopPropagation();
            onDelete();
          }}
          title="删除"
        >
          <Trash2 size={13} />
        </button>
      </div>
    </div>
  );
}
