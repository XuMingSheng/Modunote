import { useState, useEffect } from "react";
import { X, Trash2 } from "lucide-react";
import { cn } from "@/lib/utils";
import { useError } from "@/context/ErrorContext";
import { blockApi } from "@/api/blockApi";
import type { BlockGetOpenResponseItem } from "@/api/types/blockGetOpenResponse";

interface OpneBlockListProps {
  activeBlockId: string | null;
  onActiveBlockIdChange: (newId: string | null) => void;
  expanded: boolean;
}

export const OpenBlockList = ({
  activeBlockId,
  onActiveBlockIdChange,
  expanded,
}: OpneBlockListProps) => {
  const [openBlocks, setOpenBlocks] = useState<BlockGetOpenResponseItem[]>([]);
  const [loading, setLoading] = useState(true);

  const { setError } = useError();

  useEffect(() => {
    loadOpenBlocks();
  }, [activeBlockId]);

  const loadOpenBlocks = async () => {
    try {
      const openBlocks = await blockApi.getOpen();
      setOpenBlocks(openBlocks);
    } catch (error) {
      console.error("Failed to load open blocks: ", error);
      setError(
        `Failed to load open blocks: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    } finally {
      setLoading(false);
    }
  };

  const removeBlockFromList = (blockId: string) => {
    const filtered = openBlocks.filter((b) => b.id !== blockId);

    if (blockId == activeBlockId) {
      const newActiveBlockId = filtered.length ? filtered[0].id : null;
      onActiveBlockIdChange(newActiveBlockId);
    } else {
      setOpenBlocks(filtered);
    }
  };

  const handleClose = async (blockId: string) => {
    try {
      await blockApi.close(blockId);
      removeBlockFromList(blockId);
    } catch (error) {
      console.error("Failed to close block: ", error);
      setError(
        `Failed to close block: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    }
  };

  const handleDelete = async (blockId: string, blockTitle: string) => {
    const confirmation = window.confirm(
      `Are you sure you want to DELETE block "${blockTitle}"? This action cannot be undone.`
    );

    if (confirmation) {
      try {
        await blockApi.delete(blockId);
        removeBlockFromList(blockId);
      } catch (error) {
        console.error("Failed to delete block: ", error);
        setError(
          `Failed to delete block: ${error instanceof Error ? error.message : "Unknown error"}`
        );
      }
    }
  };

  const renderListItem = (block: BlockGetOpenResponseItem) => {
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
              handleClose(block.id);
            }}
            aria-label={`Close block ${block.title}`}
          >
            <X className="w-4 h-4" />
          </button>
          <button
            className="text-red-700 hover:text-red-900"
            onClick={(e) => {
              e.stopPropagation();
              handleDelete(block.id, block.title);
            }}
            aria-label={`Delete block ${block.title}`}
          >
            <Trash2 className="w-4 h-4" />
          </button>
        </div>
      </>
    );
  };

  if (loading) {
    <div className="p-3">
      <p className="text-sm text-gray-500">Loading blocks ...</p>
    </div>;
  }

  return (
    <div className="flex-1 overflow-y-auto">
      {openBlocks.map((block) => (
        <div
          key={block.id}
          className={cn(
            "flex items-center justify-between px-3 py-2 cursor-pointer border-b border-gray-200 hover:bg-gray-100",
            block.id === activeBlockId ? "bg-gray-200 font-semibold" : ""
          )}
          onClick={() => onActiveBlockIdChange(block.id)}
        >
          {renderListItem(block)}
        </div>
      ))}
    </div>
  );
};
