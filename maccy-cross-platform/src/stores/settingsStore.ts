import { create } from "zustand";

export interface AppSettings {
  clipboard_check_interval: number;
  history_size: number;
  launch_at_login: boolean;
  paste_and_hide: boolean;
  popup_position: string;
  window_width: number;
  window_height: number;
  theme: string;
  preview_width: number;
  ignore_patterns: string[];
  ignored_apps: string[];
  ignored_types: string[];
  global_hotkey: string;
  paste_hotkey: string;
}

interface SettingsState {
  settings: AppSettings | null;
  isLoading: boolean;
  setSettings: (settings: AppSettings) => void;
  setLoading: (loading: boolean) => void;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  settings: null,
  isLoading: true,
  setSettings: (settings) => set({ settings, isLoading: false }),
  setLoading: (isLoading) => set({ isLoading }),
}));
