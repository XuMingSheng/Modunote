import { useRef } from "react";

import { AutoSave } from "@/components/shared/AutoSave";
import { MilkdownEditor } from "../shared/MiikdownEditor";
import { useAppStore } from "@/store/useAppStore";
import { blockApi } from "@/api/blocks/blockApi";
import { type UpdateBlockRequest } from "@/api/blocks/types/updateBlockRequest";

import "@milkdown/crepe/theme/common/style.css";
import "@milkdown/crepe/theme/frame.css";

const AUTOSAVE_INTERNVAL = 1500;

interface Block {
  title: string;
  content: string;
}

export const BlockEditor = () => {
  const loadedBlockId = useAppStore((state) => state.activeBlock?.id);
  const loadedTitle = useAppStore((state) => state.activeBlock?.title);
  const loadedContent = useAppStore((state) => state.activeBlock?.content);
  const loadOpenedBlocks = useAppStore((state) => state.loadOpenedBlocks);
  const setError = useAppStore((state) => state.setError);

  const activeBlockRef = useRef<Block | null>(null);
  const autoSaveRef = useRef<any>(null);

  const updateBlock = async (
    blockId: string,
    updateData: UpdateBlockRequest
  ) => {
    try {
      await blockApi.update(blockId, updateData);
      await loadOpenedBlocks();
    } catch (error) {
      console.error("Failed to update block:", error);
      const errorMsg = `Failed to update block: ${error instanceof Error ? error.message : "Unknown error"}`;
      setError(errorMsg);
      throw new Error(errorMsg);
    }
  };

  if (!loadedBlockId) {
    activeBlockRef.current = null;

    return (
      <p className="text-gray-500">No block open. Create or import one.</p>
    );
  }

  activeBlockRef.current = {
    title: loadedTitle!,
    content: loadedContent!,
  };
  const activeBlock = activeBlockRef.current;

  const handleChangeTitle = (title: string) => {
    autoSaveRef.current?.updateData({
      ...activeBlock,
      title: title,
    });
    activeBlock.title = title;
  };

  const handleChangeContent = (content: string) => {
    autoSaveRef.current?.updateData({
      ...activeBlock,
      content: content,
    });
    activeBlock.content = content;
  };

  return (
    <div className="flex flex-col h-full">
      {/* Title Section */}
      <div className="border-b p-4 bg-white flex items-center gap-2">
        <label className="block text-sm font-medium text-gray-500 mb-1">
          Title
        </label>
        <input
          key={loadedBlockId}
          defaultValue={loadedTitle}
          onChange={(e) => handleChangeTitle(e.target.value)}
          className="text-xl font-bold w-full"
          placeholder="Title..."
        />
        <AutoSave
          ref={autoSaveRef}
          activeId={loadedBlockId}
          data={activeBlock}
          interval={AUTOSAVE_INTERNVAL}
          onSave={() =>
            updateBlock(loadedBlockId, {
              title: activeBlock.title,
              content: activeBlock.content,
            })
          }
        />
      </div>

      {/* Content Section */}
      <div className="text-xs text-gray-400 flex-1 overflow-y-auto">
        <MilkdownEditor
          initialContent={loadedContent!}
          onUpdate={(updatedContent) => handleChangeContent(updatedContent)}
        />
      </div>
    </div>
  );
};
