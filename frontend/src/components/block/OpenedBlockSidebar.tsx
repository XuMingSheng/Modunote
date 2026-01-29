import { useState } from "react";
import { Search, Plus } from "lucide-react";
import { cn } from "@/lib/utils";

import { useAppStore } from "@/store/useAppStore";
import { BlockSearchModal } from "@/components/BlockSearchModal";
import { OpenedBlockList } from "./OpenedBlockList";
import { blockApi } from "@/api/blocks/blockApi";
import type { SearchBlocksResponseItem } from "@/api/search/types/searchBlocksResponse";

export const OpenedBlocksSidebar = () => {
  const [expanded, setExpanded] = useState(false);
  const [searchOpen, setSearchOpen] = useState(false);

  const activateBlock = useAppStore((state) => state.activateBlock);
  const setError = useAppStore((state) => state.setError);

  const handleCreate = async () => {
    try {
      const block = await blockApi.create();
      await activateBlock(block.id);
    } catch (error) {
      console.error("Failed to create new block:", error);
      setError(
        `Failed to create new block: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    }
  };

  const handleSearchSelect = async (block: SearchBlocksResponseItem) => {
    await activateBlock(block.id);
    setSearchOpen(false);
  };

  return (
    <>
      <aside
        className={cn(
          "left-14 top-0 h-screen bg-white border-r border-gray-300 flex flex-col transition-width duration-300",
          expanded ? "w-52" : "w-12"
        )}
        onMouseEnter={() => setExpanded(true)}
        onMouseLeave={() => setExpanded(false)}
      >
        {/* List of Open Blocks */}
        <OpenedBlockList expanded={expanded} />

        {/* Create New Block Button */}
        <button
          className={cn(
            "flex items-center justify-center h-12 border-b border-gray-300 hover:bg-gray-100 transition-colors",
            expanded ? "justify-start px-4 gap-2" : "justify-center"
          )}
          onClick={handleCreate}
          aria-label="Create new block"
        >
          <Plus className="w-6 h-6" />
          {expanded && <span>Create New Block</span>}
        </button>
        {/* Open Blocks Search Button */}
        <button
          className="flex items-center justify-center h-12 border-t border-gray-300 hover:bg-gray-100"
          onClick={() => setSearchOpen(true)}
          title="Open Blocks"
          aria-label="Open block search"
        >
          <Search className="w-6 h-6" />
          {expanded && <span className="ml-2">Open Blocks</span>}
        </button>
      </aside>

      <BlockSearchModal
        isOpen={searchOpen}
        onSelect={handleSearchSelect}
        onClose={() => setSearchOpen(false)}
      />
    </>
  );
};
