import { useState } from "react";
import { ChevronRight, Plus, Search, FileText } from "lucide-react";
import { PinnedItemList } from "./PinnedItemList";
import { cn } from "@/lib/utils";

interface CanvasSidebarProps {
  onSwitch: (canvasId: string) => void;
}

export const CanvasSidebar = ({ onSwitch }: CanvasSidebarProps) => {
  const [expanded, setExpanded] = useState(false);

  return (
    <aside
      className={cn(
        "left-14 top-0 h-screen bg-white border-r border-gray-300 flex flex-col transition-width duration-300",
        expanded ? "w-80" : "w-12"
      )}
    >
      {/* Toggle Button */}
      <button
        onClick={() => setExpanded(!expanded)}
        className="flex items-center justify-center h-12 border-b border-gray-200 hover:bg-gray-100 transition-colors"
        title={expanded ? "Collapse sidebar" : "Expand sidebar"}
      >
        <ChevronRight
          className={cn(
            "w-5 h-5 text-gray-600 transition-transform duration-300",
            expanded && "rotate-180"
          )}
        />
      </button>

      {/* Pinned Items List */}
      <div className="flex-1 overflow-y-auto">
        {expanded && <PinnedItemList onClickCanvas={onSwitch} />}
      </div>

      {/* Action Buttons */}
      <div className="border-t border-gray-200">
        {/* New Canvas Button */}
        <button
          className={cn(
            "flex items-center h-12 w-full border-b border-gray-200 hover:bg-gray-100 transition-colors",
            expanded ? "justify-start px-4 gap-3" : "justify-center"
          )}
          onClick={() => {
            /* TODO: Create new canvas */
          }}
          title="Create new canvas"
        >
          <Plus className="w-5 h-5" />
          {expanded && <span>New Canvas</span>}
        </button>

        {/* Pin Buttons Row */}
        <div className="flex h-12">
          {/* Pin Blocks Button */}
          <button
            className={cn(
              "flex items-center flex-1 border-r border-gray-200 hover:bg-gray-100 transition-colors",
              expanded ? "justify-start px-3 gap-2" : "justify-center"
            )}
            onClick={() => {
              /* TODO: Open block search modal */
            }}
            title="Pin blocks"
          >
            <Search className="w-4 h-4" />
            {expanded && <span className="text-sm">Pin Blocks</span>}
          </button>

          {/* Pin Canvas Button */}
          <button
            className={cn(
              "flex items-center flex-1 hover:bg-gray-100 transition-colors",
              expanded ? "justify-start px-3 gap-2" : "justify-center"
            )}
            onClick={() => {
              /* TODO: Open canvas search modal */
            }}
            title="Pin canvas"
          >
            <FileText className="w-4 h-4" />
            {expanded && <span className="text-sm">Pin Canvas</span>}
          </button>
        </div>
      </div>
    </aside>
  );
};
