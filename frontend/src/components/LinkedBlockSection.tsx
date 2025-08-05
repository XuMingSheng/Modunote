import { type FC, useState, useEffect } from "react";
import { Plus, FileSearch, X } from "lucide-react";
import { useBlocks } from "@/context/BlocksContext";
import { type Block, type BlockSearchResponseItem } from "@/api/blockApi";
import { BlockSearchModal } from "./BlockSearchModal";

interface LinkedBlockSectionProps {
  title: string;
  blocksType: "parentBlocks" | "childBlocks" | "relatedBlocks";
  activeBlock: Block;
  onMouseEnderBlock: (id: string) => void;
  onMouseLeaveBlock: (id: string) => void;
}

function getLinkedBlocks<T extends keyof Block>(
  block: Block,
  blocksType: T
): Block[T] {
  return block[blocksType];
}

export const LinkedBlockSection: FC<LinkedBlockSectionProps> = ({
  title,
  blocksType,
  activeBlock,
  onMouseEnderBlock,
  onMouseLeaveBlock,
}) => {
  const { openBlock, createNewBlock, updateBlock } = useBlocks();
  const [searchOpen, setSearchOpen] = useState(false);

  const blocks = getLinkedBlocks(activeBlock, blocksType);

  async function handleAdd() {
    const newBlock = await createNewBlock();
    updateBlock(activeBlock.id, {
      [blocksType]: [...blocks, { id: newBlock.id, title: newBlock.title }],
    });
  }

  function handleSearchSelect(block: BlockSearchResponseItem) {
    if (blocks.findIndex((b) => b.id == block.id) !== -1) {
      return;
    }
    updateBlock(activeBlock.id, {
      [blocksType]: [...blocks, { id: block.id, title: block.title }],
    });
  }

  function handleDelete(blockId: string) {
    updateBlock(activeBlock.id, {
      [blocksType]: blocks.filter((b) => b.id !== blockId),
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
        {blocks.map((block) => (
          <div
            key={block.id}
            className="flex items-center justify-between px-3 py-2 cursor-pointer border-b border-gray-200 hover:bg-gray-100"
            onMouseEnter={() => onMouseEnderBlock(block.id)}
            onMouseLeave={() => onMouseLeaveBlock(block.id)}
          >
            <span
              className="min-w-0 flex-1 truncate text-sm"
              onClick={() => openBlock(block.id)}
            >
              {block.title}
            </span>
            <div className="flex items-center gap-1">
              <button
                className="ml-2 text-red-500 hover:text-red-700"
                onClick={(e) => {
                  e.stopPropagation();
                  handleDelete(block.id);
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
