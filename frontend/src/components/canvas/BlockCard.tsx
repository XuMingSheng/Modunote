import { useState, useEffect, useRef } from "react";
import {
  Edit3,
  X,
  ChevronDown,
  ChevronRight,
  Move,
  Eye,
  EyeOff,
  Maximize2,
} from "lucide-react";
import { MarkdownRenderer } from "../shared/MarkdownRenderer";
import { type CanvasBlockPlacement } from "@/api/types/canvas";
import { type Block } from "@/api/types/block";
import { blockApi } from "@/api/blockApi";
import { cn } from "@/lib/utils";

interface BlockCardProps {
  placement: CanvasBlockPlacement;
  gridSize: number;
  zoomLevel: number;
  showEdges: boolean;
  onUpdate: (placement: CanvasBlockPlacement) => void;
  onRemove: (placementId: string) => void;
  snapToGrid: (value: number) => number;
}

export const BlockCard = ({
  placement,
  gridSize,
  zoomLevel,
  showEdges,
  onUpdate,
  onRemove,
  snapToGrid,
}: BlockCardProps) => {
  const [block, setBlock] = useState<Block | null>(null);
  const [isDragging, setIsDragging] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });
  const [initialPos, setInitialPos] = useState({ x: 0, y: 0 });
  const [collapsed, setCollapsed] = useState(placement.collapsed);
  const [showLocalEdges, setShowLocalEdges] = useState(true);

  const cardRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const fetchBlock = async () => {
      try {
        const blockData = await blockApi.get(placement.blockId);
        setBlock(blockData);
      } catch (error) {
        console.error("Failed to fetch block:", error);
      }
    };
    fetchBlock();
  }, [placement.blockId]);

  const handleMouseDown = (e: React.MouseEvent) => {
    if (
      e.target !== e.currentTarget &&
      !e.currentTarget.contains(e.target as Node)
    )
      return;

    setIsDragging(true);
    setDragStart({ x: e.clientX, y: e.clientY });
    setInitialPos({ x: placement.x, y: placement.y });

    e.preventDefault();
  };

  const handleMouseMove = (e: MouseEvent) => {
    if (!isDragging) return;

    const deltaX = (e.clientX - dragStart.x) / zoomLevel;
    const deltaY = (e.clientY - dragStart.y) / zoomLevel;

    const newX = snapToGrid(initialPos.x + deltaX);
    const newY = snapToGrid(initialPos.y + deltaY);

    onUpdate({
      ...placement,
      x: Math.max(gridSize, newX), // Minimum 1 grid unit from edge
      y: Math.max(gridSize, newY), // Minimum 1 grid unit from edge
    });
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  const handleResizeMouseDown = (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsResizing(true);
    setDragStart({ x: e.clientX, y: e.clientY });
    setInitialPos({ x: placement.width, y: placement.height });
  };

  const handleResizeMouseMove = (e: MouseEvent) => {
    if (!isResizing) return;

    const deltaX = (e.clientX - dragStart.x) / zoomLevel;
    const deltaY = (e.clientY - dragStart.y) / zoomLevel;

    const newWidth = snapToGrid(Math.max(gridSize * 4, initialPos.x + deltaX)); // Minimum 4 grid units wide
    const newHeight = snapToGrid(Math.max(gridSize * 3, initialPos.y + deltaY)); // Minimum 3 grid units tall

    onUpdate({
      ...placement,
      width: newWidth,
      height: newHeight,
    });
  };

  const handleResizeMouseUp = () => {
    setIsResizing(false);
  };

  useEffect(() => {
    if (isDragging) {
      document.addEventListener("mousemove", handleMouseMove);
      document.addEventListener("mouseup", handleMouseUp);
    }
    if (isResizing) {
      document.addEventListener("mousemove", handleResizeMouseMove);
      document.addEventListener("mouseup", handleResizeMouseUp);
    }

    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
      document.removeEventListener("mousemove", handleResizeMouseMove);
      document.removeEventListener("mouseup", handleResizeMouseUp);
    };
  }, [isDragging, isResizing, dragStart, initialPos]);

  const handleCollapse = () => {
    const newCollapsed = !collapsed;
    setCollapsed(newCollapsed);
    onUpdate({
      ...placement,
      collapsed: newCollapsed,
    });
  };

  const handleEdit = () => {
    // TODO: Navigate to workplace or open edit modal
    console.log("Edit block:", block?.id);
  };

  const handleRemove = () => {
    onRemove(placement.id);
  };

  const handleToggleEdges = () => {
    setShowLocalEdges(!showLocalEdges);
  };

  if (!block) {
    return (
      <div
        className="absolute bg-gray-200 border border-gray-300 rounded-lg animate-pulse"
        style={{
          left: placement.x,
          top: placement.y,
          width: placement.width,
          height: placement.height,
        }}
      />
    );
  }

  const cardHeight = collapsed ? 60 : placement.height;

  return (
    <div
      ref={cardRef}
      className={cn(
        "absolute bg-white border-2 rounded-lg shadow-md cursor-move select-none transition-shadow hover:shadow-lg",
        isDragging && "shadow-xl z-30",
        isResizing && "shadow-xl z-30"
      )}
      style={{
        left: placement.x,
        top: placement.y,
        width: placement.width,
        height: cardHeight,
        borderColor: isDragging || isResizing ? "#3b82f6" : "#e5e7eb",
      }}
      onMouseDown={handleMouseDown}
    >
      {/* Header */}
      <div className="flex items-center justify-between p-3 border-b border-gray-200 bg-gray-50 rounded-t-lg">
        <div className="flex items-center gap-2 flex-1 min-w-0">
          <button
            onClick={handleCollapse}
            className="p-1 hover:bg-gray-200 rounded"
          >
            {collapsed ? (
              <ChevronRight className="w-4 h-4" />
            ) : (
              <ChevronDown className="w-4 h-4" />
            )}
          </button>
          <h3 className="font-medium text-sm truncate">{block.title}</h3>
        </div>

        <div className="flex items-center gap-1">
          <button
            onClick={handleToggleEdges}
            className="p-1 hover:bg-gray-200 rounded"
            title={showLocalEdges ? "Hide edges" : "Show edges"}
          >
            {showLocalEdges ? (
              <Eye className="w-4 h-4" />
            ) : (
              <EyeOff className="w-4 h-4" />
            )}
          </button>
          <button
            onClick={handleEdit}
            className="p-1 hover:bg-gray-200 rounded"
            title="Edit in workplace"
          >
            <Edit3 className="w-4 h-4" />
          </button>
          <button
            onClick={handleRemove}
            className="p-1 hover:bg-red-100 hover:text-red-600 rounded"
            title="Remove from canvas"
          >
            <X className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Content */}
      {!collapsed && (
        <div className="p-3 overflow-auto h-full">
          <div
            className="prose prose-sm max-w-none text-sm leading-relaxed"
            style={{
              maxHeight: placement.height - 80, // Account for header
            }}
          >
            <MarkdownRenderer markdown={block.content || "No content"} />
          </div>
        </div>
      )}

      {/* Resize Handle */}
      {!collapsed && (
        <div
          className="absolute bottom-0 right-0 w-4 h-4 cursor-se-resize"
          onMouseDown={handleResizeMouseDown}
        >
          <Maximize2 className="w-3 h-3 absolute bottom-0.5 right-0.5 text-gray-400" />
        </div>
      )}

      {/* Drag Indicator */}
      {isDragging && (
        <div className="absolute -top-1 -left-1 w-2 h-2 bg-blue-500 rounded-full" />
      )}
    </div>
  );
};
