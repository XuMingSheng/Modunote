import { useState, useEffect, useRef } from "react";
import { type Canvas, type CanvasBlockPlacement } from "@/api/types/canvas";
import { BlockCard } from "./BlockCard";
import { EdgeRenderer } from "./EdgeRenderer";

interface CanvasGridProps {
  activeCanvas: Canvas;
}

const GRID_SIZE = 40; // Grid cell size in pixels
const GRID_COLOR = "#f0f0f0";

export const CanvasGrid = ({ activeCanvas }: CanvasGridProps) => {
  const canvasRef = useRef<HTMLDivElement>(null);
  const [showGrid, setShowGrid] = useState(true);
  const [showEdges, setShowEdges] = useState(true);
  const [placements, setPlacements] = useState<CanvasBlockPlacement[]>([]);
  const [zoomLevel, setZoomLevel] = useState(1);

  const MIN_ZOOM = 0.25;
  const MAX_ZOOM = 3;
  const ZOOM_STEP = 0.25;

  useEffect(() => {
    // Convert grid units to pixels and add padding offset
    const CANVAS_PADDING = GRID_SIZE * 2; // 2 grid units of padding
    const pixelPlacements = activeCanvas.blockPlacements.map((placement) => ({
      ...placement,
      x: placement.x * GRID_SIZE + CANVAS_PADDING,
      y: placement.y * GRID_SIZE + CANVAS_PADDING,
      width: Math.max(placement.width * GRID_SIZE, GRID_SIZE * 4), // Minimum 4 grid units wide
      height: Math.max(placement.height * GRID_SIZE, GRID_SIZE * 3), // Minimum 3 grid units tall
    }));
    setPlacements(pixelPlacements);
  }, [activeCanvas]);

  const handlePlacementUpdate = (updatedPlacement: CanvasBlockPlacement) => {
    setPlacements((prev) =>
      prev.map((p) => (p.id === updatedPlacement.id ? updatedPlacement : p))
    );

    // TODO: Convert back to grid units and save to API
    // const gridPlacement = {
    //   ...updatedPlacement,
    //   x: updatedPlacement.x / GRID_SIZE,
    //   y: updatedPlacement.y / GRID_SIZE,
    //   width: updatedPlacement.width / GRID_SIZE,
    //   height: updatedPlacement.height / GRID_SIZE,
    // };
    // canvasApi.updatePlacement(gridPlacement);
  };

  const handleRemoveFromCanvas = (placementId: string) => {
    setPlacements((prev) => prev.filter((p) => p.id !== placementId));
  };

  const snapToGrid = (value: number) => {
    const scaledGridSize = GRID_SIZE * zoomLevel;
    return Math.round(value / scaledGridSize) * scaledGridSize;
  };

  const getGridStyle = () => {
    if (!showGrid) return {};

    const scaledGridSize = GRID_SIZE * zoomLevel;
    return {
      backgroundImage: `
        linear-gradient(to right, ${GRID_COLOR} 1px, transparent 1px),
        linear-gradient(to bottom, ${GRID_COLOR} 1px, transparent 1px)
      `,
      backgroundSize: `${scaledGridSize}px ${scaledGridSize}px`,
    };
  };

  const handleZoomIn = () => {
    setZoomLevel((prev) => Math.min(MAX_ZOOM, prev + ZOOM_STEP));
  };

  const handleZoomOut = () => {
    setZoomLevel((prev) => Math.max(MIN_ZOOM, prev - ZOOM_STEP));
  };

  const handleZoomReset = () => {
    setZoomLevel(1);
  };

  const handleWheel = (e: React.WheelEvent) => {
    if (e.ctrlKey || e.metaKey) {
      e.preventDefault();
      const delta = e.deltaY > 0 ? -ZOOM_STEP : ZOOM_STEP;
      setZoomLevel((prev) =>
        Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, prev + delta))
      );
    }
  };

  return (
    <div
      className="relative w-full h-full overflow-auto bg-white"
      onWheel={handleWheel}
    >
      {/* Canvas Controls */}
      <div className="absolute top-4 right-4 z-20 bg-white rounded-lg shadow-md border p-3 space-y-3">
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={showGrid}
            onChange={(e) => setShowGrid(e.target.checked)}
            className="rounded"
          />
          Show Grid
        </label>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={showEdges}
            onChange={(e) => setShowEdges(e.target.checked)}
            className="rounded"
          />
          Show Edges
        </label>

        {/* Zoom Controls */}
        <div className="pt-2 border-t border-gray-200">
          <div className="text-xs font-medium text-gray-700 mb-2">Zoom:</div>
          <div className="flex items-center gap-2">
            <button
              onClick={handleZoomOut}
              disabled={zoomLevel <= MIN_ZOOM}
              className="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 disabled:opacity-50 disabled:cursor-not-allowed rounded"
            >
              −
            </button>
            <span className="text-xs font-mono min-w-[3rem] text-center">
              {Math.round(zoomLevel * 100)}%
            </span>
            <button
              onClick={handleZoomIn}
              disabled={zoomLevel >= MAX_ZOOM}
              className="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 disabled:opacity-50 disabled:cursor-not-allowed rounded"
            >
              +
            </button>
            <button
              onClick={handleZoomReset}
              className="px-2 py-1 text-xs bg-blue-100 hover:bg-blue-200 rounded"
            >
              Reset
            </button>
          </div>
          <div className="text-xs text-gray-500 mt-1">
            Ctrl/Cmd + Scroll to zoom
          </div>
        </div>

        {/* Edge Legend */}
        {showEdges && (
          <div className="pt-2 border-t border-gray-200">
            <div className="text-xs font-medium text-gray-700 mb-2">
              Edge Types:
            </div>
            <div className="space-y-1 text-xs">
              <div className="flex items-center gap-2">
                <div className="w-4 h-0.5 bg-blue-500"></div>
                <span>Parent-Child</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-0.5 bg-green-500 border-dashed border-t-2 border-green-500"></div>
                <span>Related</span>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Canvas Title */}
      <div className="absolute top-4 left-4 z-20 bg-white rounded-lg shadow-md border px-4 py-2">
        <h1 className="text-lg font-semibold text-gray-800">
          {activeCanvas.name}
        </h1>
      </div>

      {/* Grid Container */}
      <div
        ref={canvasRef}
        className="relative origin-top-left"
        style={{
          ...getGridStyle(),
          transform: `scale(${zoomLevel})`,
        }}
      >
        {/* Block Cards */}
        {placements.map((placement) => (
          <BlockCard
            key={placement.id}
            placement={placement}
            gridSize={GRID_SIZE}
            zoomLevel={zoomLevel}
            showEdges={showEdges}
            onUpdate={handlePlacementUpdate}
            onRemove={handleRemoveFromCanvas}
            snapToGrid={snapToGrid}
          />
        ))}

        {/* Connection Lines/Edges */}
        <EdgeRenderer
          placements={placements}
          showEdges={showEdges}
          gridSize={GRID_SIZE}
          zoomLevel={zoomLevel}
        />
      </div>
    </div>
  );
};
