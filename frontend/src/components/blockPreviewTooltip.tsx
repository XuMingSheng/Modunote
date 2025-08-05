import { type FC, useEffect, useState } from "react";
import { MarkdownRenderer } from "./MarkdownRenderer";
import { type Block, blockApi } from "@/api/blockApi";

interface BlockPreviewTooltipProps {
  blockId: string;
}

export const BlockPreviewTooltip: FC<BlockPreviewTooltipProps> = ({
  blockId,
}) => {
  const [block, setBlock] = useState<Block | null>(null);

  useEffect(() => {
    const fetchBlock = async () => {
      const block = await blockApi.get(blockId);
      setBlock(block);
    };
    fetchBlock();
  }, [blockId]);

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
