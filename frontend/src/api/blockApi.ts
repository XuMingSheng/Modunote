import { type Block } from "@/context/BlocksContext";
import { ApiError } from "./errors";

let blocksDb: Block[] = [
  { id: "b1", title: "Block One", content: "Content 1" },
  { id: "b2", title: "Block Two", content: "Content 2" },
  { id: "b3", title: "Block Three", content: "Content 3" },
  { id: "b4", title: "Block Four", content: "Content 4" },
];

let openBlockIds = new Set<string>(["b1", "b2"]);

function delay<T>(data: T, ms = 300): Promise<T> {
  return new Promise((resolve) => setTimeout(() => resolve(data), ms));
}

export interface BlockCreateRequest {
  title: string;
  content?: string;
}

export interface BlockUpdateRequest {
  title?: string;
  content?: string;
}

export const blockApi = {
  async get(id: string): Promise<Block> {
    const block = blocksDb.find((b) => b.id === id);
    if (block) {
      return delay(block);
    }
    throw new ApiError(404, "blockApi.get");
  },

  async getOpen(): Promise<Block[]> {
    const openBlocks = blocksDb.filter((b) => openBlockIds.has(b.id));
    return delay(openBlocks);
  },

  async search(query: string): Promise<Block[]> {
    const q = query.toLowerCase();
    const results = blocksDb.filter(
      (b) =>
        b.title.toLowerCase().includes(q) || b.content.toLowerCase().includes(q)
    );
    return delay(results);
  },

  async create(request: BlockCreateRequest): Promise<Block> {
    const newBlock: Block = {
      id: `b${Date.now()}`,
      title: request.title,
      content: request.content ?? "",
    };
    blocksDb.push(newBlock);
    openBlockIds.add(newBlock.id);
    return delay(newBlock);
  },

  async open(id: string): Promise<void> {
    openBlockIds.add(id);
    return delay(undefined);
  },

  async update(id: string, request: BlockUpdateRequest): Promise<Block> {
    const block = blocksDb.find((b) => b.id === id) ?? null;
    if (block) {
      block.title = request.title ?? block.title;
      block.content = request.content ?? block.content;
      return delay(block);
    }
    throw new ApiError(404, "blockApi.update");
  },

  async close(id: string): Promise<void> {
    openBlockIds.delete(id);
    return delay(undefined);
  },

  async delete(id: string): Promise<void> {
    blocksDb = blocksDb.filter((b) => b.id !== id);
    openBlockIds.delete(id);
    return delay(undefined);
  },
};
