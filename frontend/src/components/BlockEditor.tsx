import { type FC, useState, useEffect } from "react";
import { Milkdown, MilkdownProvider, useEditor } from "@milkdown/react";
import { Crepe } from "@milkdown/crepe";

import { useAutoSave } from "@/hooks/useAutoSave";
import { type Block } from "@/api/blockApi";
import { useBlocks } from "@/context/BlocksContext";

import "@milkdown/crepe/theme/common/style.css";
import "@milkdown/crepe/theme/frame.css";

interface CrepeEditorProps {
  blockId: string;
  initialContent: string;
  onUpdate: (content: string) => void;
}

interface BlockEditorProps {
  block: Block;
}

const CrepeEditor: FC<CrepeEditorProps> = ({
  blockId,
  initialContent,
  onUpdate,
}: CrepeEditorProps) => {
  useEditor(
    (root) => {
      const crepe = new Crepe({
        root,
        defaultValue: initialContent,
      });

      crepe.on((api) => {
        api.markdownUpdated((ctx, markdown, prevMarkdown) => {
          onUpdate(markdown);
        });
      });

      return crepe;
    },
    [blockId, initialContent]
  );

  return <Milkdown />;
};

export const BlockEditor: FC<BlockEditorProps> = ({ block }) => {
  const { updateBlock } = useBlocks();
  const [title, setTitle] = useState(block.title);
  const [content, setContent] = useState(block.content);

  // Reset state when block changes
  useEffect(() => {
    setTitle(block.title);
    setContent(block.content);
  }, [block.id]);

  // Triggers autosave whenever content changes
  const { saveStatus } = useAutoSave({
    blockId: block.id,
    content: content,
  });

  const handleTitleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newTitle = e.target.value;
    setTitle(newTitle);
    updateBlock(block.id, { title: newTitle });
  };

  return (
    <div className="flex flex-col h-full">
      {/* Title Section */}
      <div className="border-b p-4 bg-white flex items-center gap-2">
        <label className="block text-sm font-medium text-gray-500 mb-1">
          Title
        </label>
        <input
          value={title}
          onChange={handleTitleChange}
          className="text-xl font-bold w-full"
          placeholder="Title..."
        />
        <p className="text-xs text-gray-400 mt-1">
          {saveStatus === "idle"
            ? "idle..."
            : saveStatus === "saving"
              ? "Saving..."
              : saveStatus === "saved"
                ? "Saved"
                : saveStatus === "error"
                  ? "Failed to save"
                  : null}
        </p>
      </div>

      {/* Content Section */}
      <div className="text-xs text-gray-400 flex-1 overflow-y-auto">
        <MilkdownProvider>
          <CrepeEditor
            blockId={block.id}
            initialContent={block.content}
            onUpdate={(markdown) => setContent(markdown)}
          />
        </MilkdownProvider>
      </div>
    </div>
  );
};
