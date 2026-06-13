import { Search, X, Settings } from "lucide-react";
import { useHistoryStore } from "../stores/historyStore";
import { useState } from "react";
import SettingsPanel from "./SettingsPanel";

export default function HeaderView() {
  const { searchQuery, setSearchQuery } = useHistoryStore();
  const [showSettings, setShowSettings] = useState(false);

  return (
    <>
      <div className="p-3 border-b border-gray-200 dark:border-gray-700">
        <div className="relative flex items-center gap-2">
          <div className="relative flex-1">
            <Search
              className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400"
              size={16}
            />
            <input
              type="text"
              placeholder="Search clipboard history..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-10 pr-10 py-2 bg-gray-100 dark:bg-gray-800 border border-transparent focus:border-primary-500 rounded-lg text-sm focus:outline-none transition-colors"
              autoFocus
            />
            {searchQuery && (
              <button
                onClick={() => setSearchQuery("")}
                className="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
              >
                <X size={16} />
              </button>
            )}
          </div>
          <button
            onClick={() => setShowSettings(true)}
            className="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
            title="Settings"
          >
            <Settings size={18} className="text-gray-600 dark:text-gray-400" />
          </button>
        </div>
      </div>

      {showSettings && (
        <div className="fixed inset-0 z-50 bg-black bg-opacity-50 flex items-center justify-center">
          <div className="w-full h-full max-w-2xl max-h-[600px] bg-white dark:bg-gray-900 rounded-lg shadow-xl overflow-hidden">
            <SettingsPanel onClose={() => setShowSettings(false)} />
          </div>
        </div>
      )}
    </>
  );
}
