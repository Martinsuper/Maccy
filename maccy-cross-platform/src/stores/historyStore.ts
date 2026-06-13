import { create } from "zustand";

export interface HistoryItem {
  id: string;
  title: string;
  content_type: string;
  application?: string;
  first_copied_at: number;
  last_copied_at: number;
  number_of_copies: number;
  pin?: string;
}

interface HistoryState {
  items: HistoryItem[];
  searchQuery: string;
  selectedId: string | null;
  setItems: (items: HistoryItem[]) => void;
  addItem: (item: HistoryItem) => void;
  removeItem: (id: string) => void;
  clearItems: () => void;
  setSearchQuery: (query: string) => void;
  setSelectedId: (id: string | null) => void;
  filteredItems: () => HistoryItem[];
}

export const useHistoryStore = create<HistoryState>((set, get) => ({
  items: [],
  searchQuery: "",
  selectedId: null,

  setItems: (items) => set({ items }),

  addItem: (item) =>
    set((state) => ({
      items: [item, ...state.items],
    })),

  removeItem: (id) =>
    set((state) => ({
      items: state.items.filter((item) => item.id !== id),
    })),

  clearItems: () => set({ items: [] }),

  setSearchQuery: (query) => set({ searchQuery: query }),

  setSelectedId: (id) => set({ selectedId: id }),

  filteredItems: () => {
    const { items, searchQuery } = get();
    if (!searchQuery) return items;

    return items.filter((item) =>
      item.title.toLowerCase().includes(searchQuery.toLowerCase())
    );
  },
}));
