import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, Save, RotateCcw } from "lucide-react";
import { useSettingsStore, AppSettings } from "../stores/settingsStore";

interface SettingsPanelProps {
  onClose: () => void;
}

export default function SettingsPanel({ onClose }: SettingsPanelProps) {
  const { setSettings, isLoading } = useSettingsStore();
  const [localSettings, setLocalSettings] = useState<AppSettings | null>(null);
  const [activeTab, setActiveTab] = useState<"general" | "appearance" | "ignore" | "hotkeys">("general");
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    loadSettings();
  }, []);

  async function loadSettings() {
    try {
      const data = await invoke<AppSettings>("get_settings");
      setSettings(data);
      setLocalSettings(data);
    } catch (error) {
      console.error("Failed to load settings:", error);
    }
  }

  async function handleSave() {
    if (!localSettings) return;

    setIsSaving(true);
    try {
      await invoke("update_settings", { settings: localSettings });
      setSettings(localSettings);
      onClose();
    } catch (error) {
      console.error("Failed to save settings:", error);
      alert("Failed to save settings");
    } finally {
      setIsSaving(false);
    }
  }

  async function handleReset() {
    if (!confirm("Are you sure you want to reset all settings to default?")) return;

    try {
      await invoke("reset_settings");
      await loadSettings();
    } catch (error) {
      console.error("Failed to reset settings:", error);
    }
  }

  function updateSetting<K extends keyof AppSettings>(
    key: K,
    value: AppSettings[K]
  ) {
    if (!localSettings) return;
    setLocalSettings({ ...localSettings, [key]: value });
  }

  if (isLoading || !localSettings) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-gray-500">Loading settings...</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-white dark:bg-gray-900">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
          Settings
        </h2>
        <button
          onClick={onClose}
          className="p-1 hover:bg-gray-100 dark:hover:bg-gray-800 rounded transition-colors"
        >
          <X size={20} className="text-gray-600 dark:text-gray-400" />
        </button>
      </div>

      {/* Tabs */}
      <div className="flex border-b border-gray-200 dark:border-gray-700">
        {(["general", "appearance", "ignore", "hotkeys"] as const).map((tab) => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            className={`flex-1 px-4 py-2 text-sm font-medium transition-colors ${
              activeTab === tab
                ? "text-primary-600 border-b-2 border-primary-600 dark:text-primary-400 dark:border-primary-400"
                : "text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100"
            }`}
          >
            {tab.charAt(0).toUpperCase() + tab.slice(1)}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-4">
        {activeTab === "general" && (
          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Clipboard Check Interval (ms)
              </label>
              <input
                type="number"
                value={localSettings.clipboard_check_interval}
                onChange={(e) =>
                  updateSetting("clipboard_check_interval", Number(e.target.value))
                }
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
                min="100"
                max="5000"
                step="100"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                How often to check for clipboard changes (default: 500ms)
              </p>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                History Size
              </label>
              <input
                type="number"
                value={localSettings.history_size}
                onChange={(e) => updateSetting("history_size", Number(e.target.value))}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
                min="10"
                max="10000"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Maximum number of clipboard history items (default: 200)
              </p>
            </div>

            <div className="flex items-center justify-between">
              <div>
                <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Launch at Login
                </label>
                <p className="text-xs text-gray-500 dark:text-gray-400">
                  Start Maccy when you log in
                </p>
              </div>
              <input
                type="checkbox"
                checked={localSettings.launch_at_login}
                onChange={(e) => updateSetting("launch_at_login", e.target.checked)}
                className="w-5 h-5 text-primary-600 rounded"
              />
            </div>

            <div className="flex items-center justify-between">
              <div>
                <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Paste and Hide
                </label>
                <p className="text-xs text-gray-500 dark:text-gray-400">
                  Hide window after pasting
                </p>
              </div>
              <input
                type="checkbox"
                checked={localSettings.paste_and_hide}
                onChange={(e) => updateSetting("paste_and_hide", e.target.checked)}
                className="w-5 h-5 text-primary-600 rounded"
              />
            </div>
          </div>
        )}

        {activeTab === "appearance" && (
          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Popup Position
              </label>
              <select
                value={localSettings.popup_position}
                onChange={(e) => updateSetting("popup_position", e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
              >
                <option value="cursor">Near Cursor</option>
                <option value="menubar">Menu Bar</option>
                <option value="center">Center of Screen</option>
                <option value="last">Last Position</option>
              </select>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Theme
              </label>
              <select
                value={localSettings.theme}
                onChange={(e) => updateSetting("theme", e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
              >
                <option value="auto">Auto (System)</option>
                <option value="light">Light</option>
                <option value="dark">Dark</option>
              </select>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Window Width
              </label>
              <input
                type="number"
                value={localSettings.window_width}
                onChange={(e) => updateSetting("window_width", Number(e.target.value))}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
                min="300"
                max="800"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Window Height
              </label>
              <input
                type="number"
                value={localSettings.window_height}
                onChange={(e) => updateSetting("window_height", Number(e.target.value))}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
                min="300"
                max="1000"
              />
            </div>
          </div>
        )}

        {activeTab === "ignore" && (
          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Ignored Clipboard Types
              </label>
              <textarea
                value={localSettings.ignored_types.join("\n")}
                onChange={(e) =>
                  updateSetting(
                    "ignored_types",
                    e.target.value.split("\n").filter((t) => t.trim())
                  )
                }
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 font-mono text-sm"
                rows={6}
                placeholder="One type per line..."
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Clipboard types to ignore (one per line)
              </p>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Ignored Applications
              </label>
              <textarea
                value={localSettings.ignored_apps.join("\n")}
                onChange={(e) =>
                  updateSetting(
                    "ignored_apps",
                    e.target.value.split("\n").filter((t) => t.trim())
                  )
                }
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 font-mono text-sm"
                rows={4}
                placeholder="com.apple.finder&#10;com.google.Chrome"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Application bundle IDs to ignore (one per line)
              </p>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Ignore Patterns (Regex)
              </label>
              <textarea
                value={localSettings.ignore_patterns.join("\n")}
                onChange={(e) =>
                  updateSetting(
                    "ignore_patterns",
                    e.target.value.split("\n").filter((t) => t.trim())
                  )
                }
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 font-mono text-sm"
                rows={4}
                placeholder="^password.*&#10;^secret.*"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Regex patterns to match and ignore (one per line)
              </p>
            </div>
          </div>
        )}

        {activeTab === "hotkeys" && (
          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Global Hotkey
              </label>
              <input
                type="text"
                value={localSettings.global_hotkey}
                onChange={(e) => updateSetting("global_hotkey", e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 font-mono"
                placeholder="Cmd+Shift+C"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Shortcut to show/hide Maccy (default: Cmd+Shift+C)
              </p>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Paste Hotkey
              </label>
              <input
                type="text"
                value={localSettings.paste_hotkey}
                onChange={(e) => updateSetting("paste_hotkey", e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 font-mono"
                placeholder="Cmd+Shift+V"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Shortcut to paste from clipboard (default: Cmd+Shift+V)
              </p>
            </div>

            <div className="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
              <p className="text-sm text-blue-800 dark:text-blue-200">
                <strong>Note:</strong> Hotkey changes require app restart to take effect.
              </p>
            </div>
          </div>
        )}
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between p-4 border-t border-gray-200 dark:border-gray-700">
        <button
          onClick={handleReset}
          className="flex items-center gap-2 px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
        >
          <RotateCcw size={16} />
          Reset to Default
        </button>
        <div className="flex items-center gap-2">
          <button
            onClick={onClose}
            className="px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleSave}
            disabled={isSaving}
            className="flex items-center gap-2 px-4 py-2 text-sm text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors disabled:opacity-50"
          >
            <Save size={16} />
            {isSaving ? "Saving..." : "Save"}
          </button>
        </div>
      </div>
    </div>
  );
}
