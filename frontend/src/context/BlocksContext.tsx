import { useEffect } from "react";
import { blockApi, type Block, type BlockUpdateRequest } from "../api/blockApi";
import { createContext, useContext, useState, type ReactNode } from "react";
export interface BlocksContextType {
  openBlocks: Block[];
  activeBlockId: string | null;

  setOpenBlocks: (blocks: Block[]) => void;
  setActiveBlockId: (id: string | null) => void;

  getActiveBlock: () => Block | null;
  createNewBlock: () => Promise<Block>;
  openBlock: (id: string) => void;
  closeBlock: (id: string) => void;
  updateBlock: (id: string, request: BlockUpdateRequest) => void;
  deleteBlock: (id: string) => void;
}

const BlocksContext = createContext<BlocksContextType | undefined>(undefined);

export function BlocksProvider({ children }: { children: ReactNode }) {
  const [openBlocks, setOpenBlocks] = useState<Block[]>([]);
  const [activeBlockId, setActiveBlockId] = useState<string | null>(null);

  // Load open blocks on mount
  useEffect(() => {
    const fetchOpenBlocks = async () => {
      const blocks = await blockApi.getOpen();
      setOpenBlocks(blocks);
      if (blocks.length) setActiveBlockId(blocks[0].id);
    };
    fetchOpenBlocks();
  }, []);

  function getActiveBlock() {
    return openBlocks.find((b) => b.id === activeBlockId) || null;
  }

  async function openBlock(id: string) {
    if (openBlocks.find((b) => b.id === id)) {
      setActiveBlockId(id);
      return;
    }

    const block = await blockApi.get(id);
    await blockApi.open(id);

    if (block) {
      setOpenBlocks([...openBlocks, block]);
      setActiveBlockId(block.id);
    }
  }

  async function closeBlock(id: string) {
    await blockApi.close(id);

    const filtered = openBlocks.filter((b) => b.id !== id);
    setOpenBlocks(filtered);

    if (activeBlockId === id) {
      setActiveBlockId(filtered.length ? filtered[0].id : null);
    }
  }

  async function createNewBlock() {
    const newBlock = await blockApi.create({
      title: `New Block ${Date.now()}`,
    });
    openBlock(newBlock.id);
    return newBlock;
  }

  async function updateBlock(id: string, request: BlockUpdateRequest) {
    const block = await blockApi.update(id, request);
    setOpenBlocks((prev) => prev.map((b) => (b.id === id ? block : b)));
  }

  async function deleteBlock(id: string) {
    await blockApi.delete(id);
    closeBlock(id);
  }

  return (
    <BlocksContext.Provider
      value={{
        openBlocks,
        activeBlockId,
        setOpenBlocks,
        setActiveBlockId,
        getActiveBlock,
        openBlock,
        closeBlock,
        createNewBlock,
        updateBlock,
        deleteBlock,
      }}
    >
      {children}
    </BlocksContext.Provider>
  );
}

export function useBlocks() {
  const context = useContext(BlocksContext);
  if (!context) {
    throw new Error("useBlocks must be used within a BlocksProvider");
  }
  return context;
}
