import { type Canvas } from "./types/canvas";
import { ApiError } from "./errors";

function delay<T>(data: T, ms = 300): Promise<T> {
  return new Promise((resolve) => setTimeout(() => resolve(data), ms));
}

export let pinnedCanvasIds = new Set<string>([
  "canvas-1",
  "canvas-2",
  "canvas-3",
  "canvas-4",
  "canvas-5",
  "canvas-6",
  "canvas-7",
  "canvas-8",
  "canvas-9",
  "canvas-10",
]);

export let canvasDb: Canvas[] = [
  {
    id: "canvas-1",
    name: "Standalone Canvas (No Pinned Blocks)",
    blockPlacements: [
      {
        id: "p-1",
        blockId: "b2",
        x: 4,
        y: 2,
        width: 4,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-2",
        blockId: "b3",
        x: 10,
        y: 5,
        width: 4,
        height: 3,
        collapsed: false,
      },
    ],
  },
  {
    id: "canvas-2",
    name: "Complex: Root + Middle + Leaf Blocks",
    blockPlacements: [
      {
        id: "p-3",
        blockId: "b1",
        x: 2,
        y: 1,
        width: 6,
        height: 4,
        collapsed: false,
      },
      {
        id: "p-4",
        blockId: "b4",
        x: 10,
        y: 1,
        width: 4,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-5",
        blockId: "b11",
        x: 10,
        y: 5,
        width: 4,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-6",
        blockId: "b6",
        x: 16,
        y: 1,
        width: 5,
        height: 4,
        collapsed: false,
      },
      {
        id: "p-7",
        blockId: "b2",
        x: 2,
        y: 6,
        width: 4,
        height: 3,
        collapsed: false,
      },
    ],
  },
  {
    id: "canvas-3",
    name: "Middle Level: Both b4 and b9",
    blockPlacements: [
      {
        id: "p-8",
        blockId: "b4",
        x: 3,
        y: 2,
        width: 5,
        height: 4,
        collapsed: false,
      },
      {
        id: "p-9",
        blockId: "b9",
        x: 10,
        y: 2,
        width: 4,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-10",
        blockId: "b12",
        x: 3,
        y: 7,
        width: 5,
        height: 3,
        collapsed: false,
      },
    ],
  },
  {
    id: "canvas-4",
    name: "Pure Leaf: Only b11",
    blockPlacements: [
      {
        id: "p-12",
        blockId: "b11",
        x: 2,
        y: 1,
        width: 5,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-13",
        blockId: "b3",
        x: 9,
        y: 1,
        width: 5,
        height: 4,
        collapsed: false,
      },
      {
        id: "p-14",
        blockId: "b5",
        x: 2,
        y: 5,
        width: 4,
        height: 3,
        collapsed: false,
      },
    ],
  },
  {
    id: "canvas-5",
    name: "Pure Leaf: Only b6",
    blockPlacements: [
      {
        id: "p-16",
        blockId: "b6",
        x: 4,
        y: 2,
        width: 5,
        height: 4,
        collapsed: false,
      },
      {
        id: "p-17",
        blockId: "b10",
        x: 11,
        y: 2,
        width: 4,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-18",
        blockId: "b8",
        x: 4,
        y: 7,
        width: 4,
        height: 3,
        collapsed: false,
      },
    ],
  },
  {
    id: "canvas-6",
    name: "Parent-Child Combo: b1 + b4",
    blockPlacements: [
      {
        id: "p-20",
        blockId: "b1",
        x: 3,
        y: 3,
        width: 3,
        height: 2,
        collapsed: true,
      },
      {
        id: "p-21",
        blockId: "b4",
        x: 8,
        y: 3,
        width: 4,
        height: 3,
        collapsed: false,
      },
    ],
  },
  {
    id: "canvas-7",
    name: "Another Standalone (No Pinned)",
    blockPlacements: [
      {
        id: "p-22",
        blockId: "b7",
        x: 2,
        y: 2,
        width: 4,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-23",
        blockId: "b8",
        x: 8,
        y: 2,
        width: 4,
        height: 3,
        collapsed: false,
      },
    ],
  },
  {
    id: "canvas-8",
    name: "Mixed Hierarchy: b9 + b11 + b6",
    blockPlacements: [
      {
        id: "p-24",
        blockId: "b9",
        x: 1,
        y: 1,
        width: 5,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-25",
        blockId: "b11",
        x: 8,
        y: 1,
        width: 4,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-26",
        blockId: "b6",
        x: 1,
        y: 6,
        width: 4,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-27",
        blockId: "b13",
        x: 14,
        y: 1,
        width: 3,
        height: 4,
        collapsed: false,
      },
    ],
  },
  {
    id: "canvas-9",
    name: "Root Only: Pure b1",
    blockPlacements: [
      {
        id: "p-28",
        blockId: "b1",
        x: 3,
        y: 2,
        width: 6,
        height: 4,
        collapsed: false,
      },
      {
        id: "p-29",
        blockId: "b2",
        x: 11,
        y: 2,
        width: 4,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-30",
        blockId: "b3",
        x: 3,
        y: 7,
        width: 5,
        height: 3,
        collapsed: false,
      },
    ],
  },
  {
    id: "canvas-10",
    name: "All Leaves: b11 + b6 Only",
    blockPlacements: [
      {
        id: "p-31",
        blockId: "b11",
        x: 2,
        y: 3,
        width: 4,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-32",
        blockId: "b6",
        x: 8,
        y: 3,
        width: 4,
        height: 3,
        collapsed: false,
      },
      {
        id: "p-33",
        blockId: "b15",
        x: 14,
        y: 3,
        width: 3,
        height: 3,
        collapsed: false,
      },
    ],
  },
];

export const canvasApi = {
  async get(id: string): Promise<Canvas> {
    const canvas = canvasDb.find((c) => c.id === id);
    if (canvas) {
      return delay(canvas);
    }
    throw new ApiError(404, "canvasApi.get");
  },

  async getAll(): Promise<Canvas[]> {
    return delay([...canvasDb]);
  },

  async search(query: string): Promise<Canvas[]> {
    const q = query.toLowerCase();
    const results = canvasDb.filter((canvas) =>
      canvas.name.toLowerCase().includes(q)
    );
    return delay(results);
  },

  async create(name: string): Promise<Canvas> {
    const newCanvas: Canvas = {
      id: `canvas-${Date.now()}`,
      name,
      blockPlacements: [],
    };
    canvasDb.push(newCanvas);
    return delay(newCanvas);
  },

  async update(id: string, updates: Partial<Canvas>): Promise<Canvas> {
    const canvasIndex = canvasDb.findIndex((c) => c.id === id);
    if (canvasIndex === -1) {
      throw new ApiError(404, "canvasApi.update");
    }

    canvasDb[canvasIndex] = { ...canvasDb[canvasIndex], ...updates };
    return delay(canvasDb[canvasIndex]);
  },

  async delete(id: string): Promise<void> {
    const canvasIndex = canvasDb.findIndex((c) => c.id === id);
    if (canvasIndex === -1) {
      throw new ApiError(404, "canvasApi.delete");
    }

    canvasDb.splice(canvasIndex, 1);
    pinnedCanvasIds.delete(id);
    return delay(undefined);
  },

  async pin(id: string): Promise<void> {
    pinnedCanvasIds.add(id);
    return delay(undefined);
  },

  async unpin(id: string): Promise<void> {
    pinnedCanvasIds.delete(id);
    return delay(undefined);
  },
};
