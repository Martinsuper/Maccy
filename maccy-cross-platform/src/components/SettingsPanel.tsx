import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, RotateCcw } from "lucide-react";
import { useSettingsStore, AppSettings } from "../stores/settingsStore";

interface SettingsPanelProps {
  onClose: () => void;
}

export default function SettingsPanel({ onClose }: SettingsPanelProps) {
  const { setSettings } = useSettingsStore();
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
    } finally {
      setIsSaving(false);
    }
  }

  async function handleReset() {
    try {
      await invoke("reset_settings");
      await loadSettings();
    } catch (error) {
      console.error("Failed to reset:", error);
    }
  }

  function update<K extends keyof AppSettings>(key: K, value: AppSettings[K]) {
    if (!localSettings) return;
    setLocalSettings({ ...localSettings, [key]: value });
  }

  if (!localSettings) {
    return (
      <div className="settings-overlay">
        <div className="settings-header">
          <span className="settings-header-title">设置</span>
        </div>
        <div className="empty-state">
          <div className="empty-state-desc">加载中...</div>
        </div>
      </div>
    );
  }

  const tabLabels: Record<string, string> = {
    general: "通用",
    appearance: "外观",
    ignore: "忽略",
    hotkeys: "快捷键",
  };

  return (
    <div className="settings-overlay">
      {/* Header */}
      <div className="settings-header">
        <span className="settings-header-title">设置</span>
        <button className="title-bar-btn close" onClick={onClose}>
          <X size={12} />
        </button>
      </div>

      {/* Tabs */}
      <div className="settings-tabs">
        {(["general", "appearance", "ignore", "hotkeys"] as const).map((tab) => (
          <button
            key={tab}
            className={`settings-tab ${activeTab === tab ? "active" : ""}`}
            onClick={() => setActiveTab(tab)}
          >
            {tabLabels[tab]}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="settings-content">
        {activeTab === "general" && (
          <>
            <div className="settings-group">
              <label className="settings-label">剪贴板检测间隔 (毫秒)</label>
              <input
                type="number"
                className="settings-input"
                value={localSettings.clipboard_check_interval}
                onChange={(e) => update("clipboard_check_interval", Number(e.target.value))}
                min={100}
                max={5000}
                step={100}
              />
              <div className="settings-hint">检测剪贴板变化的频率（默认：500ms）</div>
            </div>

            <div className="settings-group">
              <label className="settings-label">历史记录上限</label>
              <input
                type="number"
                className="settings-input"
                value={localSettings.history_size}
                onChange={(e) => update("history_size", Number(e.target.value))}
                min={10}
                max={10000}
              />
              <div className="settings-hint">最大保存的剪贴板历史条数（默认：200）</div>
            </div>

            <div className="settings-toggle">
              <div className="settings-toggle-info">
                <div className="settings-toggle-label">开机自启动</div>
                <div className="settings-toggle-desc">登录时自动启动 Maccy</div>
              </div>
              <label className="toggle-switch">
                <input
                  type="checkbox"
                  checked={localSettings.launch_at_login}
                  onChange={(e) => update("launch_at_login", e.target.checked)}
                />
                <span className="toggle-slider" />
              </label>
            </div>

            <div className="settings-toggle">
              <div className="settings-toggle-info">
                <div className="settings-toggle-label">粘贴后隐藏</div>
                <div className="settings-toggle-desc">粘贴操作后自动隐藏窗口</div>
              </div>
              <label className="toggle-switch">
                <input
                  type="checkbox"
                  checked={localSettings.paste_and_hide}
                  onChange={(e) => update("paste_and_hide", e.target.checked)}
                />
                <span className="toggle-slider" />
              </label>
            </div>
          </>
        )}

        {activeTab === "appearance" && (
          <>
            <div className="settings-group">
              <label className="settings-label">弹出位置</label>
              <select
                className="settings-select"
                value={localSettings.popup_position}
                onChange={(e) => update("popup_position", e.target.value)}
              >
                <option value="cursor">跟随光标</option>
                <option value="menubar">菜单栏</option>
                <option value="center">屏幕中央</option>
                <option value="last">上次位置</option>
              </select>
            </div>

            <div className="settings-group">
              <label className="settings-label">主题</label>
              <select
                className="settings-select"
                value={localSettings.theme}
                onChange={(e) => update("theme", e.target.value)}
              >
                <option value="auto">跟随系统</option>
                <option value="light">浅色</option>
                <option value="dark">深色</option>
              </select>
            </div>

            <div className="settings-group">
              <label className="settings-label">窗口宽度</label>
              <input
                type="number"
                className="settings-input"
                value={localSettings.window_width}
                onChange={(e) => update("window_width", Number(e.target.value))}
                min={300}
                max={800}
              />
            </div>

            <div className="settings-group">
              <label className="settings-label">窗口高度</label>
              <input
                type="number"
                className="settings-input"
                value={localSettings.window_height}
                onChange={(e) => update("window_height", Number(e.target.value))}
                min={300}
                max={1000}
              />
            </div>
          </>
        )}

        {activeTab === "ignore" && (
          <>
            <div className="settings-group">
              <label className="settings-label">忽略的剪贴板类型</label>
              <textarea
                className="settings-textarea"
                rows={4}
                value={localSettings.ignored_types.join("\n")}
                onChange={(e) =>
                  update(
                    "ignored_types",
                    e.target.value.split("\n").filter((t) => t.trim())
                  )
                }
                placeholder="NSColor&#10;NSFilenamesPboardType"
              />
              <div className="settings-hint">每行一个类型名称</div>
            </div>

            <div className="settings-group">
              <label className="settings-label">忽略的应用</label>
              <textarea
                className="settings-textarea"
                rows={4}
                value={localSettings.ignored_apps.join("\n")}
                onChange={(e) =>
                  update(
                    "ignored_apps",
                    e.target.value.split("\n").filter((t) => t.trim())
                  )
                }
                placeholder="com.apple.finder&#10;com.google.Chrome"
              />
              <div className="settings-hint">每行一个应用 Bundle ID</div>
            </div>

            <div className="settings-group">
              <label className="settings-label">忽略规则（正则表达式）</label>
              <textarea
                className="settings-textarea"
                rows={4}
                value={localSettings.ignore_patterns.join("\n")}
                onChange={(e) =>
                  update(
                    "ignore_patterns",
                    e.target.value.split("\n").filter((t) => t.trim())
                  )
                }
                placeholder="^password.*&#10;^secret.*"
              />
              <div className="settings-hint">匹配的内容将不会被记录</div>
            </div>
          </>
        )}

        {activeTab === "hotkeys" && (
          <>
            <div className="settings-group">
              <label className="settings-label">全局快捷键</label>
              <input
                type="text"
                className="settings-input"
                value={localSettings.global_hotkey}
                onChange={(e) => update("global_hotkey", e.target.value)}
                placeholder="Cmd+Shift+C"
              />
              <div className="settings-hint">显示/隐藏 Maccy 的快捷键</div>
            </div>

            <div className="settings-group">
              <label className="settings-label">粘贴快捷键</label>
              <input
                type="text"
                className="settings-input"
                value={localSettings.paste_hotkey}
                onChange={(e) => update("paste_hotkey", e.target.value)}
                placeholder="Cmd+Shift+V"
              />
              <div className="settings-hint">直接粘贴选中内容的快捷键</div>
            </div>

            <div className="info-box">
              💡 修改快捷键后需要重启应用才能生效
            </div>
          </>
        )}
      </div>

      {/* Footer */}
      <div className="settings-footer">
        <button
          className="settings-footer-btn secondary"
          onClick={handleReset}
        >
          <RotateCcw size={12} />
          恢复默认
        </button>
        <div style={{ display: "flex", gap: 6 }}>
          <button
            className="settings-footer-btn secondary"
            onClick={onClose}
          >
            取消
          </button>
          <button
            className="settings-footer-btn primary"
            onClick={handleSave}
            disabled={isSaving}
          >
            {isSaving ? "保存中..." : "保存"}
          </button>
        </div>
      </div>
    </div>
  );
}
