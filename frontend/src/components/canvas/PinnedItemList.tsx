import { useState, useEffect } from "react";
import { ChevronRight, FileText, Hash, PinOff, Trash2 } from "lucide-react";
import { type PinnedItem } from "../../api/types/pinnedItem";
import { pinnedHierarchyApi } from "@/api/pinnedHierarchyApi";
import { cn } from "@/lib/utils";
import { canvasApi } from "@/api/canvasApi";

interface PinnedItemListProps {
  onClickCanvas: (canvasId: string) => void;
}

export const PinnedItemList = ({ onClickCanvas }: PinnedItemListProps) => {
  const [pinnedItems, setPinnedItems] = useState<PinnedItem[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadPinnedItems();
  }, []);

  const loadPinnedItems = async () => {
    try {
      const hierarchyItems = await pinnedHierarchyApi.get();
      setPinnedItems(hierarchyItems);
    } catch (error) {
      console.error("Failed to load pinned items:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleUnpin = async (item: PinnedItem) => {
    try {
      if (item.type === "block") {
        await pinnedHierarchyApi.unpinBlock(item.id);
      } else {
        await pinnedHierarchyApi.unpinCanvas(item.id);
      }
      // Reload the pinned items
      loadPinnedItems();
    } catch (error) {
      console.error("Failed to unpin item:", error);
    }
  };

  const handleDeleteCanvas = async (item: PinnedItem) => {
    const confirmation = window.confirm(
      `Are you sure you want to DELETE canvas "${item.title}"? This action cannot be undone.`
    );

    if (confirmation) {
      try {
        await canvasApi.delete(item.id);
      } catch (error) {
        console.error("Failed to delete canvas:", error);
      }
    }
  };

  const toggleExpanded = (itemId: string) => {
    const updateItems = (items: PinnedItem[]): PinnedItem[] => {
      return items.map((item) => {
        if (item.id === itemId) {
          return { ...item, expanded: !item.expanded };
        }
        return { ...item, children: updateItems(item.children) };
      });
    };
    setPinnedItems(updateItems(pinnedItems));
  };

  const renderPinnedItem = (item: PinnedItem, depth = 0) => {
    const isCanvas = item.type === "canvas";
    const hasChildren = item.children.length > 0;

    return (
      <div key={item.id} className="select-none">
        <div
          className="flex items-center gap-2 px-2 py-1 hover:bg-gray-100 rounded group"
          style={{ paddingLeft: `${8 + depth * 16}px` }}
        >
          {/* Expand/Collapse Button */}
          {hasChildren && (
            <button
              onClick={() => toggleExpanded(item.id)}
              className="p-0.5 hover:bg-gray-200 rounded opacity-60 hover:opacity-100"
            >
              <ChevronRight
                className={cn(
                  "w-3 h-3 duration-300",
                  item.expanded && "rotate-90"
                )}
              />
            </button>
          )}

          {/* Icon */}
          <div className="flex-shrink-0">
            {isCanvas ? (
              <FileText className="w-4 h-4 text-blue-600" />
            ) : (
              <Hash className="w-4 h-4 text-gray-600" />
            )}
          </div>

          {/* Title */}
          <span
            className="flex-1 text-sm truncate cursor-pointer hover:text-blue-600"
            onClick={() => onClickCanvas(item.id)}
          >
            {item.title}
          </span>

          {/* Unpin button */}
          <button
            onClick={(e) => {
              e.stopPropagation();
              handleUnpin(item);
            }}
            className="p-0.5 hover:bg-gray-200 rounded opacity-0 group-hover:opacity-100"
            title="Unpin item"
          >
            <PinOff className="w-3 h-3 text-gray-500 hover:text-red-500" />
          </button>

          {/* Delete button */}
          {isCanvas && (
            <button
              className="p-0.5 hover:bg-gray-200 rounded opacity-0 group-hover:opacity-100"
              onClick={(e) => {
                e.stopPropagation();
                handleDeleteCanvas(item);
              }}
              aria-label={`Delete canvas ${item.title}`}
            >
              <Trash2 className="w-3 h-3 text-gray-500 hover:text-red-500" />
            </button>
          )}
        </div>

        {/* Children */}
        {item.expanded &&
          item.children.map((child) => renderPinnedItem(child, depth + 1))}
      </div>
    );
  };

  if (loading) {
    return (
      <div className="p-3">
        <p className="text-sm text-gray-500">Loading pinned items...</p>
      </div>
    );
  }

  return (
    <div className="p-3">
      {pinnedItems.length === 0 ? (
        <p className="text-sm text-gray-500 italic">
          No pinned items. Use the buttons below to pin blocks or canvases.
        </p>
      ) : (
        <div className="space-y-1">
          {pinnedItems.map((item) => renderPinnedItem(item))}
        </div>
      )}
    </div>
  );
};
