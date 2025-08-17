import { useState } from "react";
import { OpenBlocksSidebar } from "@/components/block/OpenBlockSidebar";
import { BlockEditor } from "@/components/block/BlockEditor";
import { LinkedBlockSidebar } from "@/components/block/LinkedBlockSidebar";
import { useError } from "@/context/ErrorContext";
import { blockApi } from "@/api/blockApi";
import { type Block } from "@/api/types/block";

export function BlocksPage() {
  const [activeBlock, setActiveBlock] = useState<Block | null>(null);
  const { setError } = useError();

  const openBlock = async (blockId: string) => {
    try {
      await blockApi.open(blockId);
    } catch (error) {
      console.error("Failed to open block: ", error);
      setError(
        `Failed to open block: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    }
  };

  const loadActiveBlock = async (blockId: string) => {
    try {
      const block = await blockApi.get(blockId);
      setActiveBlock(block);
    } catch (error) {
      console.error("Failed to load active block:", error);
      setError(
        `Failed to load active block: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    }
  };

  const handleActiveBlockIdChange = async (newId: string | null) => {
    if (newId) {
      await loadActiveBlock(newId);
      await openBlock(newId);
    } else {
      setActiveBlock(null);
    }
  };

  return (
    <div className="flex h-screen">
      {/* Sidebar for open blocks */}
      <OpenBlocksSidebar
        activeBlockId={activeBlock?.id ?? null}
        onActiveBlockIdChange={handleActiveBlockIdChange}
      />
      {/* Center editor */}
      <div className="flex-1 p-4 bg-white border-l border-gray-300 h-screen">
        {activeBlock?.id ? (
          <BlockEditor activeBlock={activeBlock} />
        ) : (
          <p className="text-gray-500">No block open. Create or import one.</p>
        )}
      </div>
      {/* Sidebar for linked blocks */}
      {activeBlock?.id && (
        <LinkedBlockSidebar
          activeBlockId={activeBlock.id}
          onActiveBlockIdChange={handleActiveBlockIdChange}
        />
      )}
    </div>
  );
}
