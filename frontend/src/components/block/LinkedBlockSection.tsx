import { useState, useEffect } from "react";
import { Plus, FileSearch, X } from "lucide-react";

import { useError } from "@/context/ErrorContext";
import { BlockSearchModal } from "@/components/BlockSearchModal";
import { blockApi } from "@/api/blockApi";
import type { BlockLink } from "@/api/types/blockLink";
import type { BlockSearchResponseItem } from "@/api/types/blockSearchResponse";
import type { BlockUpdateRequest } from "@/api/types/blockUpdateRequest";

interface LinkedBlockSectionProps {
  title: string;
  blockLinkType: "parentBlocks" | "childBlocks" | "relatedBlocks";
  activeBlockId: string;
  onActiveBlockIdChange: (newId: string) => void;
  onHoverBlockChange: (blockId: string | null) => void;
}

export const LinkedBlockSection = ({
  title,
  blockLinkType,
  activeBlockId,
  onActiveBlockIdChange,
  onHoverBlockChange,
}: LinkedBlockSectionProps) => {
  const [searchOpen, setSearchOpen] = useState(false);
  const [blockLinks, setBlockLinks] = useState<BlockLink[]>([]);

  const { setError } = useError();

  useEffect(() => {
    loadBlockLinks();
  }, [activeBlockId]);

  const loadBlockLinks = async () => {
    try {
      const block = await blockApi.getLinks(activeBlockId);
      setBlockLinks(block[blockLinkType]);
    } catch (error) {
      console.error("Failed to load block links:", error);
      setError(
        `Failed to load block links: ${error instanceof Error ? error.message : "Unknown error"}`
      );
      return null;
    }
  };

  const createBlock = async () => {
    try {
      const block = await blockApi.create();
      return block;
    } catch (error) {
      console.error("Failed to create new block:", error);
      setError(
        `Failed to create new block: ${error instanceof Error ? error.message : "Unknown error"}`
      );
      return null;
    }
  };

  const updateBlock = async (request: BlockUpdateRequest) => {
    try {
      await blockApi.update(activeBlockId, request);
      await loadBlockLinks();
    } catch (error) {
      console.error("Failed to update block:", error);
      setError(
        `Failed to update block: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    }
  };

  const handleAdd = async () => {
    const newblock = await createBlock();
    if (newblock) {
      const newLink = { id: newblock.id, title: newblock.title };
      await updateBlock({ [blockLinkType]: [...blockLinks, newLink] });
    }
  };

  const handleSearchSelect = async (block: BlockSearchResponseItem) => {
    if (blockLinks.findIndex((b) => b.id == block.id) !== -1) {
      return;
    }
    const newLink = { id: block.id, title: block.title };
    await updateBlock({ [blockLinkType]: [...blockLinks, newLink] });
  };

  function handleRemove(blockId: string) {
    updateBlock({
      [blockLinkType]: blockLinks.filter((b) => b.id !== blockId),
    });
  }

  return (
    <div>
      {/* Title and actions */}
      <div className="flex items-center justify-between mb-2">
        <div className="text-xs font-semibold text-gray-500 uppercase tracking-wide">
          {title}
        </div>
        <div className="flex gap-1">
          <button
            onClick={handleAdd}
            className="p-1 rounded hover:bg-gray-200"
            title={`Create new ${title}`}
          >
            <Plus className="w-3 h-3 text-gray-500" />
          </button>
          <button
            onClick={() => setSearchOpen(true)}
            className="p-1 rounded hover:bg-gray-200"
            title={`Add existing block to ${title}`}
          >
            <FileSearch className="w-3 h-3 text-gray-500" />
          </button>
        </div>
      </div>
      {/* Linked block list */}
      <div className="space-y-1">
        {blockLinks.map((block) => (
          <div
            key={block.id}
            className="flex items-center justify-between px-3 py-2 cursor-pointer border-b border-gray-200 hover:bg-gray-100"
            onMouseEnter={() => onHoverBlockChange(block.id)}
            onMouseLeave={() => onHoverBlockChange(null)}
          >
            <span
              className="min-w-0 flex-1 truncate text-sm"
              onClick={() => onActiveBlockIdChange(block.id)}
            >
              {block.title}
            </span>
            <div className="flex items-center gap-1">
              <button
                className="ml-2 text-red-500 hover:text-red-700"
                onClick={(e) => {
                  e.stopPropagation();
                  handleRemove(block.id);
                }}
                aria-label={`Close block ${block.title}`}
              >
                <X className="w-4 h-4" />
              </button>
            </div>
          </div>
        ))}
      </div>
      {/* Modal for searching existing blocks */}
      <BlockSearchModal
        isOpen={searchOpen}
        onSelect={handleSearchSelect}
        onClose={() => setSearchOpen(false)}
      />
    </div>
  );
};
