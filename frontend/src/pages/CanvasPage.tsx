import { type FC, useState } from "react";
import { CanvasSidebar } from "@/components/canvas/CanvasSidebar";
import { CanvasGrid } from "@/components/canvas/CanvasGrid";
import { type Canvas } from "@/api/types/canvas";
import { canvasApi } from "@/api/canvasApi";

export const CanvasPage: FC = () => {
  const [activeCanvas, setActiveCanvas] = useState<Canvas | null>(null);

  const handleSwitchCanvas = (id: string) => {
    const fetchCanvas = async () => {
      const canvas = await canvasApi.get(id);
      console.log(canvas);
      setActiveCanvas(canvas);
    };
    fetchCanvas();
  };

  return (
    <div className="flex h-screen relative">
      <CanvasSidebar onSwitch={handleSwitchCanvas} />
      <div className="flex-1">
        {activeCanvas ? (
          <CanvasGrid activeCanvas={activeCanvas} />
        ) : (
          <p className="text-gray-500">No canvas open. Create or import one.</p>
        )}
      </div>
    </div>
  );
};
