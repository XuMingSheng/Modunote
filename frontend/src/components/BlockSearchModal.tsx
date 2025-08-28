import { useState, useEffect } from "react";
import { X } from "lucide-react";

import { useAppStore } from "@/store/useAppStore";
import { searchApi } from "@/api/searchApi";
import { type SearchBlockResponseItem } from "@/api/types/searchBlocksResponse";

interface BlockSearchModalProps {
  isOpen: boolean;
  onSelect: (block: SearchBlockResponseItem) => void;
  onClose: () => void;
}

export const BlockSearchModal = ({
  isOpen,
  onSelect,
  onClose,
}: BlockSearchModalProps) => {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchBlockResponseItem[]>([]);

  const setError = useAppStore((state) => state.setError);

  useEffect(() => {
    if (!isOpen) return;
    searchBlocks(query);
  }, [query, isOpen]);

  const searchBlocks = async (query: string) => {
    try {
      const response = await searchApi.searchBlocks(query);
      setResults(response.blocks);
    } catch (error) {
      console.error("Failed to search blocks:", error);
      setError(
        `Failed to search blocks: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    }
  };

  const handleSelect = (block: SearchBlockResponseItem) => {
    onSelect(block);
    onClose();
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/30 z-40 flex items-center justify-center">
      <div className="bg-white rounded shadow-lg w-96 max-h-[70vh] flex flex-col">
        <div className="flex items-center border-b border-gray-300 p-2">
          <input
            autoFocus
            type="text"
            placeholder="Search blocks..."
            className="flex-1 px-3 py-2 border rounded"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
          <button
            className="ml-2 p-1 text-gray-600 hover:text-gray-800"
            onClick={onClose}
            aria-label="Close search modal"
          >
            <X className="w-6 h-6" />
          </button>
        </div>
        <div className="overflow-y-auto p-2">
          {results.length === 0 && (
            <p className="text-gray-500 text-sm">No blocks found.</p>
          )}
          {results.map((block) => (
            <div
              key={block.id}
              className="cursor-pointer p-2 rounded hover:bg-gray-200"
              onClick={() => handleSelect(block)}
              role="button"
              tabIndex={0}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  handleSelect(block);
                }
              }}
            >
              <div className="font-semibold">{block.title}</div>
              <div className="text-xs text-gray-600 truncate">
                {block.title}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
