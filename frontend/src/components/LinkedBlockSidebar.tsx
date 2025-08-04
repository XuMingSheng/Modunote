import { type FC, useState, useEffect } from "react";
import { ChevronLeft } from "lucide-react";
import { useBlocks } from "@/context/BlocksContext";
import { type Block, type BlockLink } from "@/api/blockApi";

interface LinkedBlockSectionProps {
  title: string;
  blocks: BlockLink[];
}

interface LinkedBlockSidebarProps {
  block: Block;
}

const LinkedBlockSection: FC<LinkedBlockSectionProps> = ({ title, blocks }) => {
  const { openBlock } = useBlocks();

  if (!blocks || blocks.length == 0) {
    return null;
  }

  return (
    <div>
      <div className="text-xs font-semibold text-gray-500 mb-2 uppercase tracking-wide">
        {title}
      </div>
      <ul className="space-y-1">
        {blocks.map((block) => (
          <li
            key={block.id}
            className="flex items-center justify-between px-3 py-2 cursor-pointer border-b border-gray-200 hover:bg-gray-100"
            onClick={() => openBlock(block.id)}
          >
            <div className="flex items-center flex-1 min-w-0">
              <span className="truncate">{block.title}</span>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
};

export const LinkedBlockSidebar: FC<LinkedBlockSidebarProps> = ({ block }) => {
  const [expanded, setExpanded] = useState(false);

  return (
    <div
      className={`transition-all duration-200 h-full bg-gray-50 border-l border-gray-300 ${
        expanded ? "w-72" : "w-8"
      } relative flex flex-col`}
      onMouseEnter={() => setExpanded(true)}
      onMouseLeave={() => setExpanded(false)}
    >
      {/* Slim bar when collapsed */}
      {!expanded && (
        <div className="flex items-center justify-center h-full">
          <ChevronLeft className="text-gray-400" />{" "}
        </div>
      )}

      {/* Expanded sidebar */}
      {expanded && (
        <div className="p-4 space-y-6 overflow-y-auto flex-1">
          <LinkedBlockSection title="Parents" blocks={block.parentBlocks} />
          <LinkedBlockSection title="Children" blocks={block.childBlocks} />
          <LinkedBlockSection title="Related" blocks={block.relatedBlocks} />
        </div>
      )}
    </div>
  );
};
