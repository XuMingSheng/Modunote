import type { StateCreator } from "zustand";
import { type ErrorSlice } from "./errorSlice";
import { workspaceApi } from "@/api/workspace/workspaceApi";
import { blockApi } from "@/api/blocks/blockApi";
import { blockLinkApi } from "@/api/blocks/blockLinkApi";
import { type GetBlockResponse } from "@/api/blocks/types/getBlockResponse";
import { type OpenedBlock } from "@/api/workspace/types/getOpenedBlocksResponse";

export type BlockLinkType = "parents" | "children" | "related";
type Block = GetBlockResponse;

export interface BlocksSlice {
  activeBlock: Block | null;
  openedBlocks: OpenedBlock[];
  loadOpenedBlocks: () => Promise<void>;
  activateBlock: (blockId: string) => Promise<void>;
  openBlock: (blockId: string) => Promise<void>;
  closeBlock: (blockId: string) => Promise<void>;
  deleteBlock: (blockId: string) => Promise<void>;
  createLinkForActiveBlock: (
    linkType: BlockLinkType,
    linkedBlockId: string,
  ) => Promise<void>;
  deleteLinkForActiveBlock: (
    linkType: BlockLinkType,
    linkedBlockId: string,
  ) => Promise<void>;
}

const loadActiveBlock = async (blockId: string) => {
  try {
    const block = await blockApi.get(blockId);
    return block;
  } catch (error) {
    console.error("Failed to load active block:", error);
    throw new Error(
      `Failed to load active block: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const loadOpenedBlocks = async () => {
  try {
    const response = await workspaceApi.getOpenedBlocks();
    return response.openedBlocks;
  } catch (error) {
    console.error("Failed to load opened blocks:", error);
    throw new Error(
      `Failed to load opened blocks: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const isOpen = (blockId: string, openedBlocks: OpenedBlock[]) => {
  return openedBlocks.findIndex((b) => b.blockId === blockId) !== -1;
};

const openBlock = async (blockId: string) => {
  try {
    await workspaceApi.openBlock({ blockId });
  } catch (error) {
    console.error("Failed to open block:", error);
    throw new Error(
      `Failed to open block: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const closeBlock = async (blockId: string) => {
  try {
    await workspaceApi.closeBlock(blockId);
  } catch (error) {
    console.error("Failed to close block:", error);
    throw new Error(
      `Failed to close block: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const deleteBlock = async (blockId: string) => {
  try {
    await blockApi.delete(blockId);
  } catch (error) {
    console.error("Failed to delete block:", error);
    throw new Error(
      `Failed to delete block: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const loadParentBlocks = async (blockId: string) => {
  try {
    const response = await blockLinkApi.getParents(blockId);
    return response.parentBlocks;
  } catch (error) {
    console.error("Failed to load parent blocks:", error);
    throw new Error(
      `Failed to load parent blocks: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const loadChildBlocks = async (blockId: string) => {
  try {
    const response = await blockLinkApi.getChldren(blockId);
    return response.childBlocks;
  } catch (error) {
    console.error("Failed to load child blocks:", error);
    throw new Error(
      `Failed to load child blocks: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const loadRelatedBlocks = async (blockId: string) => {
  try {
    const response = await blockLinkApi.getRelated(blockId);
    return response.relatedBlocks;
  } catch (error) {
    console.error("Failed to load related blocks:", error);
    throw new Error(
      `Failed to load related blocks: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const createParentLink = async (blockId: string, parentBlockId: string) => {
  try {
    await blockLinkApi.createParentLink(blockId, parentBlockId);
  } catch (error) {
    console.error("Failed to create parent link:", error);
    throw new Error(
      `Failed to create parent link: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const createChildLink = async (blockId: string, childBlockId: string) => {
  try {
    await blockLinkApi.createChildLink(blockId, childBlockId);
  } catch (error) {
    console.error("Failed to create child link:", error);
    throw new Error(
      `Failed to create child link: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const createRelatedLink = async (blockId: string, relatedBlockId: string) => {
  try {
    await blockLinkApi.createRelatedLink(blockId, relatedBlockId);
  } catch (error) {
    console.error("Failed to create related link:", error);
    throw new Error(
      `Failed to create related link: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const deleteParentLink = async (blockId: string, parentBlockId: string) => {
  try {
    await blockLinkApi.deleteParentLink(blockId, parentBlockId);
  } catch (error) {
    console.error("Failed to delete parent link:", error);
    throw new Error(
      `Failed to delete parent link: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const deleteChildLink = async (blockId: string, childBlockId: string) => {
  try {
    await blockLinkApi.deleteChildLink(blockId, childBlockId);
  } catch (error) {
    console.error("Failed to delete child link:", error);
    throw new Error(
      `Failed to delete child link: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const deleteRelatedLink = async (blockId: string, relatedBlockId: string) => {
  try {
    await blockLinkApi.deleteRelatedLink(blockId, relatedBlockId);
  } catch (error) {
    console.error("Failed to delete related link:", error);
    throw new Error(
      `Failed to delete related link: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};

const loadNextActiveBlock = async (
  activeBlockId: string,
  openedBlocks: OpenedBlock[],
) => {
  if (openedBlocks.length == 1) {
    return null;
  }

  const activeBlockIndex = openedBlocks.findIndex(
    (b) => b.blockId == activeBlockId,
  );
  const nextActiveIndex =
    activeBlockIndex == openedBlocks.length - 1
      ? activeBlockIndex - 1
      : activeBlockIndex + 1;
  const nextActiveId = openedBlocks[nextActiveIndex].blockId;

  return await loadActiveBlock(nextActiveId);
};

const reloadActiveBlockLinksAfterDeletion = async (
  changedBlockId: string,
  activeBlock: Block,
) => {
  const isParent =
    activeBlock.parentBlocks.findIndex((b) => b.blockId === changedBlockId) !==
    -1;
  const isChild =
    activeBlock.childBlocks.findIndex((b) => b.blockId === changedBlockId) !==
    -1;
  const isRelated =
    activeBlock.relatedBlocks.findIndex((b) => b.blockId === changedBlockId) !==
    -1;

  if (isParent) {
    activeBlock.parentBlocks = await loadParentBlocks(activeBlock.id);
  } else if (isChild) {
    activeBlock.childBlocks = await loadChildBlocks(activeBlock.id);
  }
  if (isRelated) {
    activeBlock.relatedBlocks = await loadRelatedBlocks(activeBlock.id);
  }

  return activeBlock;
};

export const createBlockSlice: StateCreator<
  BlocksSlice & ErrorSlice,
  [],
  [],
  BlocksSlice
> = (set, get) => ({
  activeBlock: null,
  openedBlocks: [],

  loadOpenedBlocks: async () => {
    try {
      const openedBlocks = await loadOpenedBlocks();
      set(() => ({ openedBlocks }));
    } catch (error) {
      set(() => ({
        error: `${error instanceof Error ? error.message : "Unknown error"}`,
      }));
    }
  },

  activateBlock: async (blockId: string) => {
    try {
      let openedBlocks = get().openedBlocks;
      let activeBlock = get().activeBlock;

      if (!isOpen(blockId, openedBlocks)) {
        await openBlock(blockId);
        openedBlocks = await loadOpenedBlocks();
        set(() => ({ openedBlocks }));
      }

      if (activeBlock?.id !== blockId) {
        activeBlock = await loadActiveBlock(blockId);
        set(() => ({ activeBlock }));
      }
    } catch (error) {
      set(() => ({
        error: `${error instanceof Error ? error.message : "Unknown error"}`,
      }));
    }
  },

  openBlock: async (blockId: string) => {
    try {
      await openBlock(blockId);
      const openedBlocks = await loadOpenedBlocks();
      set(() => ({ openedBlocks }));
    } catch (error) {
      set(() => ({
        error: `${error instanceof Error ? error.message : "Unknown error"}`,
      }));
    }
  },

  closeBlock: async (blockId: string) => {
    try {
      let activeBlock = get().activeBlock;
      let openedBlocks = get().openedBlocks;

      if (activeBlock?.id == blockId) {
        activeBlock = await loadNextActiveBlock(activeBlock.id, openedBlocks);
        set(() => ({ activeBlock }));
      }

      await closeBlock(blockId);
      openedBlocks = await loadOpenedBlocks();
      set(() => ({ openedBlocks }));
    } catch (error) {
      set(() => ({
        error: `${error instanceof Error ? error.message : "Unknown error"}`,
      }));
    }
  },

  deleteBlock: async (blockId: string) => {
    try {
      await deleteBlock(blockId);

      let activeBlock = get().activeBlock;
      let openedBlocks = get().openedBlocks;

      if (activeBlock?.id == blockId) {
        activeBlock = await loadNextActiveBlock(activeBlock.id, openedBlocks);
        set(() => ({ activeBlock }));
      } else if (activeBlock) {
        activeBlock = await reloadActiveBlockLinksAfterDeletion(
          blockId,
          activeBlock,
        );
        set(() => ({ activeBlock }));
      }

      openedBlocks = await loadOpenedBlocks();
      set(() => ({ openedBlocks }));
    } catch (error) {
      set(() => ({
        error: `${error instanceof Error ? error.message : "Unknown error"}`,
      }));
    }
  },

  createLinkForActiveBlock: async (
    linkType: BlockLinkType,
    linkedBlockId: string,
  ) => {
    const activeBlock = get().activeBlock;
    if (!activeBlock) {
      return;
    }

    try {
      switch (linkType) {
        case "parents": {
          await createParentLink(activeBlock.id, linkedBlockId);
          activeBlock.parentBlocks = await loadParentBlocks(activeBlock.id);
          break;
        }
        case "children": {
          await createChildLink(activeBlock.id, linkedBlockId);
          activeBlock.childBlocks = await loadChildBlocks(activeBlock.id);
          break;
        }
        case "related": {
          await createRelatedLink(activeBlock.id, linkedBlockId);
          activeBlock.relatedBlocks = await loadRelatedBlocks(activeBlock.id);
          break;
        }
      }
      set(() => ({ activeBlock }));
    } catch (error) {
      set(() => ({
        error: `${error instanceof Error ? error.message : "Unknown error"}`,
      }));
    }
  },

  deleteLinkForActiveBlock: async (
    linkType: BlockLinkType,
    linkedBlockId: string,
  ) => {
    const activeBlock = get().activeBlock;
    if (!activeBlock) {
      return;
    }

    try {
      switch (linkType) {
        case "parents": {
          await deleteParentLink(activeBlock.id, linkedBlockId);
          activeBlock.parentBlocks = await loadParentBlocks(activeBlock.id);
          break;
        }
        case "children": {
          await deleteChildLink(activeBlock.id, linkedBlockId);
          activeBlock.childBlocks = await loadChildBlocks(activeBlock.id);
          break;
        }
        case "related": {
          await deleteRelatedLink(activeBlock.id, linkedBlockId);
          activeBlock.relatedBlocks = await loadRelatedBlocks(activeBlock.id);
          break;
        }
      }
      set(() => ({ activeBlock }));
    } catch (error) {
      set(() => ({
        error: `${error instanceof Error ? error.message : "Unknown error"}`,
      }));
    }
  },
});
