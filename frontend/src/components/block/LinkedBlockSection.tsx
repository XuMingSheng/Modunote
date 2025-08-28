import { useState } from "react";
import { Plus, FileSearch, X } from "lucide-react";

import { useAppStore } from "@/store/useAppStore";
import { type BlockLinkType } from "@/store/slices/blocksSlice";
import { BlockSearchModal } from "@/components/BlockSearchModal";
import { blockApi } from "@/api/blocks/blockApi";
import type { BlockLink } from "@/api/blocks/types/getBlockLinksResponse";
import type { SearchBlocksResponseItem } from "@/api/types/searchBlocksResponse";
interface LinkedBlockSectionProps {
  title: string;
  linkType: BlockLinkType;
  onChangeHoverBlock: (blockId: string | null) => void;
}

export const LinkedBlockSection = ({
  title,
  linkType,
  onChangeHoverBlock,
}: LinkedBlockSectionProps) => {
  const activateBlock = useAppStore((state) => state.activateBlock);
  const createLink = useAppStore((state) => state.createLinkForActiveBlock);
  const deleteLink = useAppStore((state) => state.deleteLinkForActiveBlock);
  let blockLinks: BlockLink[] | null;

  switch (linkType) {
    case "parents": {
      blockLinks = useAppStore(
        (state) => state.activeBlock?.parentBlocks ?? null
      );
      break;
    }
    case "children": {
      blockLinks = useAppStore(
        (state) => state.activeBlock?.childBlocks ?? null
      );
      break;
    }
    case "related": {
      blockLinks = useAppStore(
        (state) => state.activeBlock?.relatedBlocks ?? null
      );
    }
  }

  const [searchOpen, setSearchOpen] = useState(false);

  if (!blockLinks) {
    return null;
  }

  const handleSelect = async (blockId: string) => {
    await activateBlock(blockId);
  };

  const handleCreate = async () => {
    const block = await blockApi.create();

    if (!block) {
      return;
    }

    await createLink(linkType, block.id);
    await activateBlock(block.id);
  };

  const handleSearchSelect = async (block: SearchBlocksResponseItem) => {
    const linkExists =
      blockLinks.findIndex((b) => b.blockId == block.id) !== -1;

    if (linkExists) {
      return;
    }

    await createLink(linkType, block.id);
  };

  const handleDelete = async (blockId: string) => {
    await deleteLink(linkType, blockId);
  };

  return (
    <div>
      {/* Title and actions */}
      <div className="flex items-center justify-between mb-2">
        <div className="text-xs font-semibold text-gray-500 uppercase tracking-wide">
          {title}
        </div>
        <div className="flex gap-1">
          <button
            onClick={handleCreate}
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
        {blockLinks.map((link) => (
          <div
            key={link.blockId}
            className="flex items-center justify-between px-3 py-2 cursor-pointer border-b border-gray-200 hover:bg-gray-100"
            onMouseEnter={() => onChangeHoverBlock(link.blockId)}
            onMouseLeave={() => onChangeHoverBlock(null)}
          >
            <span
              className="min-w-0 flex-1 truncate text-sm"
              onClick={() => handleSelect(link.blockId)}
            >
              {link.title}
            </span>
            <div className="flex items-center gap-1">
              <button
                className="ml-2 text-red-500 hover:text-red-700"
                onClick={(e) => {
                  e.stopPropagation();
                  handleDelete(link.blockId);
                }}
                aria-label={`Close block ${link.title}`}
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
