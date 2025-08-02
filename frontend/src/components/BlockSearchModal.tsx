import { useState, useEffect } from "react";
import { X } from "lucide-react";
import { useBlocks, type Block } from "../context/BlocksContext";
import { blockApi } from "../api/blockApi";

export function BlockSearchModal({
  isOpen,
  onClose,
}: {
  isOpen: boolean;
  onClose: () => void;
}) {
  const { openBlock } = useBlocks();
  const [searchTerm, setSearchTerm] = useState("");
  const [results, setResults] = useState<Block[]>([]);

  useEffect(() => {
    if (!isOpen) return;

    async function fetchResults() {
      if (searchTerm.trim() === "") {
        setResults([]);
        return;
      }

      const res = await blockApi.search(searchTerm);
      setResults(res);
    }

    fetchResults();
  }, [searchTerm, isOpen]);

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
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
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
              onClick={() => {
                openBlock(block.id);
                onClose();
              }}
              role="button"
              tabIndex={0}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  openBlock(block.id);
                  onClose();
                }
              }}
            >
              <div className="font-semibold">{block.title}</div>
              <div className="text-xs text-gray-600 truncate">
                {block.content}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
