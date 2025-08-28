import { create } from "zustand";
import { type ErrorSlice, createErrorSlice } from "./slices/errorSlice";
import { type BlocksSlice, createBlockSlice } from "./slices/blocksSlice";

type AppState = ErrorSlice & BlocksSlice;

export const useAppStore = create<AppState>()((...a) => ({
  ...createErrorSlice(...a),
  ...createBlockSlice(...a),
}));
