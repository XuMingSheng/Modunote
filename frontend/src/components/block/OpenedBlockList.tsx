import { X, Trash2 } from "lucide-react";
import { cn } from "@/lib/utils";

import { useAppStore } from "@/store/useAppStore";
import type { OpenedBlock } from "@/api/blocks/types/getOpenedblocksResponse";

interface OpenedBlockListProps {
  expanded: boolean;
}

export const OpenedBlockList = ({ expanded }: OpenedBlockListProps) => {
  const activeBlockId = useAppStore((state) => state.activeBlock?.id);
  const openedBlocks = useAppStore((state) => state.openedBlocks);
  const activateBlock = useAppStore((state) => state.activateBlock);
  const closeBlock = useAppStore((state) => state.closeBlock);
  const deleteBlock = useAppStore((state) => state.deleteBlock);

  const handleSelect = async (blockId: string) => {
    await activateBlock(blockId);
  };

  const handleClose = async (blockId: string) => {
    await closeBlock(blockId);
  };

  const handleDelete = async (blockId: string, blockTitle: string) => {
    const confirmation = window.confirm(
      `Are you sure you want to DELETE block "${blockTitle}"? This action cannot be undone.`
    );

    if (confirmation) {
      await deleteBlock(blockId);
    }
  };

  const renderListItem = (block: OpenedBlock) => {
    if (!expanded) {
      return <span className="truncate">{block.title[0]}</span>;
    }

    return (
      <>
        <div className="flex items-center flex-1 min-w-0">
          <span className="truncate">{block.title}</span>
        </div>
        <div className="flex items-center gap-1">
          <button
            className="ml-2 text-red-500 hover:text-red-700"
            onClick={(e) => {
              e.stopPropagation();
              handleClose(block.blockId);
            }}
            aria-label={`Close block ${block.title}`}
          >
            <X className="w-4 h-4" />
          </button>
          <button
            className="text-red-700 hover:text-red-900"
            onClick={(e) => {
              e.stopPropagation();
              handleDelete(block.blockId, block.title);
            }}
            aria-label={`Delete block ${block.title}`}
          >
            <Trash2 className="w-4 h-4" />
          </button>
        </div>
      </>
    );
  };

  return (
    <div className="flex-1 overflow-y-auto">
      {openedBlocks.map((block) => (
        <div
          key={block.blockId}
          className={cn(
            "flex items-center justify-between px-3 py-2 cursor-pointer border-b border-gray-200 hover:bg-gray-100",
            block.blockId === activeBlockId ? "bg-gray-200 font-semibold" : ""
          )}
          onClick={() => handleSelect(block.blockId)}
        >
          {renderListItem(block)}
        </div>
      ))}
    </div>
  );
};
