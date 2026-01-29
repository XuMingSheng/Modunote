import type { StateCreator } from "zustand";

export interface ErrorSlice {
  error: string | null;
  setError: (message: string | null) => void;
}

export const createErrorSlice: StateCreator<ErrorSlice, [], [], ErrorSlice> = (
  set
) => ({
  error: null,
  setError: (message: string | null) => set({ error: message }),
});
