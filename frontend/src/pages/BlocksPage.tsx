import { useEffect } from "react";
import { OpenedBlocksSidebar } from "@/components/block/OpenedBlockSidebar";
import { BlockEditor } from "@/components/block/BlockEditor";
import { LinkedBlocksSidebar } from "@/components/block/LinkedBlocksSidebar";
import { useAppStore } from "@/store/useAppStore";

export function BlocksPage() {
  const loadOpenedBlocks = useAppStore((state) => state.loadOpenedBlocks);

  useEffect(() => {
    loadOpenedBlocks();
  }, []);

  return (
    <div className="flex h-screen">
      {/* Sidebar for open blocks */}
      <OpenedBlocksSidebar />
      {/* Center editor */}
      <div className="flex-1 p-4 h-screen bg-white border-l border-gray-300">
        <BlockEditor />
      </div>
      {/* Sidebar for linked blocks */}
      <LinkedBlocksSidebar />
    </div>
  );
}
