import { useState, useEffect, useRef } from "react";
import { ChevronLeft } from "lucide-react";

import { useAppStore } from "@/store/useAppStore";
import { LinkedBlockSection } from "./LinkedBlockSection";
import { BlockPreviewTooltip } from "./BlockPreviewTooltip";

export const LinkedBlocksSidebar = () => {
  const activeBlockId = useAppStore((state) => state.activeBlock?.id);

  const [expanded, setExpanded] = useState(false);
  const [hovered, setHovered] = useState(false);
  const [previewHovered, setPreviewHovered] = useState(false);
  const [previewBlockId, setPreviewBlockId] = useState<string | null>(null);
  const [hoveredBlockId, setHoveredBlockId] = useState<string | null>(null);
  const previewHoverTimeout = useRef<number | null>(null);

  useEffect(() => {
    setPreviewBlockId(null);
  }, [activeBlockId]);

  useEffect(() => {
    if (hovered) {
      setExpanded(true);
    } else if (!previewBlockId) {
      setExpanded(false);
    }
  }, [hovered, previewBlockId]);

  useEffect(() => {
    if (previewHoverTimeout.current) {
      clearTimeout(previewHoverTimeout.current);
    }

    if (hoveredBlockId) {
      setPreviewBlockId(hoveredBlockId);
    } else if (!previewHovered) {
      // Use a short timeout so that moving mouse from row to tooltip doesn’t flicker
      previewHoverTimeout.current = setTimeout(() => {
        setPreviewBlockId(null);
      }, 500);

      return () => {
        if (previewHoverTimeout.current) {
          clearTimeout(previewHoverTimeout.current);
        }
      };
    }
  }, [hoveredBlockId, previewHovered]);

  if (!activeBlockId) {
    return null;
  }

  return (
    <div
      className={`transition-all duration-200 h-full bg-gray-50 border-l border-gray-300 ${
        expanded ? "w-72" : "w-8"
      } relative flex flex-col`}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
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
          <LinkedBlockSection
            title="Parents"
            linkType="parents"
            onChangeHoverBlock={(id) => setHoveredBlockId(id)}
          />
          <LinkedBlockSection
            title="Children"
            linkType="children"
            onChangeHoverBlock={(id) => setHoveredBlockId(id)}
          />
          <LinkedBlockSection
            title="Related"
            linkType="related"
            onChangeHoverBlock={(id) => setHoveredBlockId(id)}
          />
        </div>
      )}
      {/* Preview */}
      {previewBlockId && (
        <div
          className="absolute right-full"
          onMouseEnter={() => setPreviewHovered(true)}
          onMouseLeave={() => setPreviewHovered(false)}
        >
          {<BlockPreviewTooltip blockId={previewBlockId} />}
        </div>
      )}
    </div>
  );
};
