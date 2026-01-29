import { useEffect, useState } from "react";
import { type CanvasBlockPlacement } from "@/api/types/canvas";
import { type Block } from "@/api/blocks/types/getBlockResponse";
import { blockApi } from "@/api/blocks/blockApi";

interface EdgeRendererProps {
  placements: CanvasBlockPlacement[];
  showEdges: boolean;
  gridSize: number;
  zoomLevel: number;
}

interface BlockData {
  [blockId: string]: Block;
}

interface Connection {
  from: string;
  to: string;
  type: "parent" | "child" | "related";
}

export const EdgeRenderer = ({
  placements,
  showEdges,
  gridSize,
  zoomLevel,
}: EdgeRendererProps) => {
  const [blocks, setBlocks] = useState<BlockData>({});
  const [connections, setConnections] = useState<Connection[]>([]);

  useEffect(() => {
    const fetchBlocks = async () => {
      const blockData: BlockData = {};
      const newConnections: Connection[] = [];

      for (const placement of placements) {
        try {
          const block = await blockApi.get(placement.blockId);
          blockData[placement.blockId] = block;

          // Add parent-child connections
          for (const parent of block.parentBlocks) {
            if (placements.some((p) => p.blockId === parent.id)) {
              newConnections.push({
                from: parent.id,
                to: placement.blockId,
                type: "parent",
              });
            }
          }

          // Add related connections
          for (const related of block.relatedBlocks) {
            if (placements.some((p) => p.blockId === related.id)) {
              newConnections.push({
                from: placement.blockId,
                to: related.id,
                type: "related",
              });
            }
          }
        } catch (error) {
          console.error(`Failed to fetch block ${placement.blockId}:`, error);
        }
      }

      setBlocks(blockData);
      setConnections(newConnections);
    };

    if (showEdges && placements.length > 0) {
      fetchBlocks();
    }
  }, [placements, showEdges]);

  const getPlacementCenter = (blockId: string) => {
    const placement = placements.find((p) => p.blockId === blockId);
    if (!placement) return { x: 0, y: 0 };

    return {
      x: placement.x + placement.width / 2,
      y: placement.y + (placement.collapsed ? 30 : placement.height / 2),
    };
  };

  const renderConnection = (connection: Connection, index: number) => {
    const fromPos = getPlacementCenter(connection.from);
    const toPos = getPlacementCenter(connection.to);

    const strokeColor =
      connection.type === "parent"
        ? "#3b82f6"
        : connection.type === "related"
          ? "#10b981"
          : "#6b7280";
    const strokeWidth = connection.type === "parent" ? 2 : 1;
    const isDashed = connection.type === "related";

    // Calculate control points for curved line
    const deltaX = toPos.x - fromPos.x;
    const deltaY = toPos.y - fromPos.y;
    const controlOffset = Math.min(Math.abs(deltaX), Math.abs(deltaY)) * 0.5;

    const controlX1 = fromPos.x + (deltaX > 0 ? controlOffset : -controlOffset);
    const controlY1 = fromPos.y;
    const controlX2 = toPos.x - (deltaX > 0 ? controlOffset : -controlOffset);
    const controlY2 = toPos.y;

    const pathData = `M ${fromPos.x} ${fromPos.y} C ${controlX1} ${controlY1}, ${controlX2} ${controlY2}, ${toPos.x} ${toPos.y}`;

    return (
      <g
        key={`${connection.from}-${connection.to}-${connection.type}-${index}`}
      >
        <path
          d={pathData}
          stroke={strokeColor}
          strokeWidth={strokeWidth}
          fill="none"
          strokeDasharray={isDashed ? "5,5" : "none"}
          opacity={0.7}
        />

        {/* Arrow head for parent-child relationships */}
        {connection.type === "parent" && (
          <polygon
            points={`${toPos.x - 6},${toPos.y - 4} ${toPos.x},${toPos.y} ${toPos.x - 6},${toPos.y + 4}`}
            fill={strokeColor}
            opacity={0.7}
          />
        )}
      </g>
    );
  };

  if (!showEdges || connections.length === 0) {
    return null;
  }

  return (
    <svg
      className="absolute inset-0 pointer-events-none z-10"
      style={{ width: "100%", height: "100%" }}
    >
      {connections.map(renderConnection)}
    </svg>
  );
};
