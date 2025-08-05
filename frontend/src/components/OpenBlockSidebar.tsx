import { useState } from "react";
import { Search, Plus, X, Trash2 } from "lucide-react";
import { cn } from "../lib/utils";
import { useBlocks } from "../context/BlocksContext";
import { BlockSearchModal } from "./BlockSearchModal";
import type { BlockSearchResponseItem } from "@/api/blockApi";

export function OpenBlocksSidebar() {
  const {
    openBlocks,
    activeBlockId,
    setActiveBlockId,
    openBlock,
    closeBlock,
    createNewBlock,
    deleteBlock,
  } = useBlocks();
  const [expanded, setExpanded] = useState(false);
  const [searchOpen, setSearchOpen] = useState(false);

  function handleDelete(blockId: string, blockTitle: string) {
    const confirmation = window.confirm(
      `Are you sure you want to DELETE block "${blockTitle}"? This action cannot be undone.`
    );
    if (confirmation) {
      deleteBlock(blockId);
    }
  }

  function handleSearchSelect(block: BlockSearchResponseItem) {
    openBlock(block.id);
    setSearchOpen(false);
  }

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
        <div className="flex-1 overflow-y-auto">
          {openBlocks.map((block) => (
            <div
              key={block.id}
              className={cn(
                "flex items-center justify-between px-3 py-2 cursor-pointer border-b border-gray-200 hover:bg-gray-100",
                block.id === activeBlockId ? "bg-gray-200 font-semibold" : ""
              )}
              onClick={() => setActiveBlockId(block.id)}
            >
              {expanded ? (
                <>
                  <div className="flex items-center flex-1 min-w-0">
                    <span className="truncate">{block.title}</span>
                  </div>
                  <div className="flex items-center gap-1">
                    <button
                      className="ml-2 text-red-500 hover:text-red-700"
                      onClick={(e) => {
                        e.stopPropagation();
                        closeBlock(block.id);
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
              ) : (
                <span className="truncate">{block.title[0]}</span>
              )}
            </div>
          ))}
        </div>
        {/* Create New Block Button */}
        <button
          className={cn(
            "flex items-center justify-center h-12 border-b border-gray-300 hover:bg-gray-100 transition-colors",
            expanded ? "justify-start px-4 gap-2" : "justify-center"
          )}
          onClick={createNewBlock}
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
}
