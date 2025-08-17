import { useState, useEffect } from "react";

import { useError } from "@/context/ErrorContext";
import { AutoSave } from "@/components/shared/AutoSave";
import { MilkdownEditor } from "../shared/MiikdownEditor";
import { blockApi } from "@/api/blockApi";
import { type Block } from "@/api/types/block";
import { type BlockUpdateRequest } from "@/api/types/blockUpdateRequest";

import "@milkdown/crepe/theme/common/style.css";
import "@milkdown/crepe/theme/frame.css";

interface BlockEditorProps {
  activeBlock: Block;
}

const AUTOSAVE_INTERNVAL = 1500;

export const BlockEditor = ({ activeBlock }: BlockEditorProps) => {
  const [title, setTitle] = useState(activeBlock.title);
  const [content, setContent] = useState(activeBlock.content);

  const { setError } = useError();

  useEffect(() => {
    setTitle(activeBlock.title);
    setContent(activeBlock.content);
  }, [activeBlock.id]);

  const updateBlock = async (updateData: BlockUpdateRequest) => {
    try {
      await blockApi.update(activeBlock.id, updateData);
      return true;
    } catch (error) {
      console.error("Failed to update block:", error);
      setError(
        `Failed to update block: ${error instanceof Error ? error.message : "Unknown error"}`
      );
      return false;
    }
  };

  if (!activeBlock) {
    return (
      <p className="text-gray-500">No block open. Create or import one.</p>
    );
  }

  return (
    <div className="flex flex-col h-full">
      {/* Title Section */}
      <div className="border-b p-4 bg-white flex items-center gap-2">
        <label className="block text-sm font-medium text-gray-500 mb-1">
          Title
        </label>
        <input
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          className="text-xl font-bold w-full"
          placeholder="Title..."
        />
        <AutoSave
          activeId={activeBlock.id}
          data={{ title, content }}
          interval={AUTOSAVE_INTERNVAL}
          onSave={async () => {
            return await updateBlock({ title, content });
          }}
        />
      </div>

      {/* Content Section */}
      <div className="text-xs text-gray-400 flex-1 overflow-y-auto">
        <MilkdownEditor
          initialContent={activeBlock.content}
          onUpdate={(updatedContent) => setContent(updatedContent)}
        />
      </div>
    </div>
  );
};
