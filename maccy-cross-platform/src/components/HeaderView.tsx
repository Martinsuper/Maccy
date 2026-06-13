import { Search, X, Settings } from "lucide-react";
import { useHistoryStore } from "../stores/historyStore";
import { useEffect, useRef } from "react";

interface HeaderViewProps {
  onOpenSettings: () => void;
}

export default function HeaderView({ onOpenSettings }: HeaderViewProps) {
  const { searchQuery, setSearchQuery } = useHistoryStore();
  const inputRef = useRef<HTMLInputElement>(null);

  // Auto-focus search on mount
  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  return (
    <div className="search-section">
      <div className="search-wrapper">
        <Search className="search-icon" size={14} />
        <input
          ref={inputRef}
          type="text"
          placeholder="Search history..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="search-input"
        />
        {searchQuery ? (
          <button
            className="search-clear"
            onClick={() => setSearchQuery("")}
          >
            <X size={12} />
          </button>
        ) : (
          <span className="search-shortcut">⌘K</span>
        )}
      </div>
      <div style={{ display: "flex", justifyContent: "flex-end", marginTop: 4 }}>
        <button
          className="title-bar-btn"
          onClick={onOpenSettings}
          title="Settings"
        >
          <Settings size={13} />
        </button>
      </div>
    </div>
  );
}
