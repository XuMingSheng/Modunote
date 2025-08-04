import { useBlocks } from "../context/BlocksContext";
import { OpenBlocksSidebar } from "../components/OpenBlockSidebar";
import { BlockEditor } from "@/components/BlockEditor";

export function Workplace() {
  const { openBlocks, activeBlockId } = useBlocks();

  const activeBlock = openBlocks.find((b) => b.id === activeBlockId) || null;

  return (
    <div className="flex h-screen">
      {/* Sidebar for open blocks */}
      <OpenBlocksSidebar />

      {/* Center editor */}
      <div className="flex-1 p-4 bg-white border-l border-gray-300 h-screen">
        {activeBlock ? (
          <BlockEditor block={activeBlock} />
        ) : (
          <p className="text-gray-500">No block open. Create or import one.</p>
        )}
      </div>
    </div>
  );
}
