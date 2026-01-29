import { useEffect, useState } from "react";

import { useAppStore } from "@/store/useAppStore";
import { MarkdownRenderer } from "@/components/shared/MarkdownRenderer";
import { blockApi } from "@/api/blocks/blockApi";
import { type GetBlockResponse as Block } from "@/api/blocks/types/getBlockResponse";

interface BlockPreviewTooltipProps {
  blockId: string;
}

export const BlockPreviewTooltip = ({ blockId }: BlockPreviewTooltipProps) => {
  const setError = useAppStore((state) => state.setError);
  const [block, setBlock] = useState<Block | null>(null);

  useEffect(() => {
    loadBlock();
  }, [blockId]);

  const loadBlock = async () => {
    try {
      const block = await blockApi.get(blockId);
      setBlock(block);
    } catch (error) {
      console.error("Failed to load block: ", error);
      setError(
        `Failed to load block: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    }
  };

  if (!block) {
    return (
      <div className="bg-white shadow-lg rounded-md p-3 border border-gray-200 z-50">
        <p>Loading...</p>
      </div>
    );
  }

  return (
    <div className="bg-white shadow-lg rounded-md p-3 border border-gray-200 z-50">
      <h3 className="font-medium text-sm mb-2">{block.title}</h3>
      <div className="text-sm text-gray-600 w-[40vw] max-h-[80vh] overflow-y-auto">
        <MarkdownRenderer markdown={block.content} />
      </div>
    </div>
  );
};
