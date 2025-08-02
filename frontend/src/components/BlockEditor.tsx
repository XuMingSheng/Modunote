import { useState, useEffect } from "react";
import { useAutoSave } from "@/hooks/useAutoSave";
import { MarkdownRenderer } from "@/components/MarkdownRenderer";
import { type Block, useBlocks } from "@/context/BlocksContext";

interface Props {
  block: Block;
}

export function BlockEditor({ block }: Props) {
  const { updateBlock } = useBlocks();
  const [title, setTitle] = useState(block.title);
  const [content, setContent] = useState(block.content);
  const [isEditing, setIsEditing] = useState(false);

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
      <div className="flex-1 overflow-auto p-4">
        {isEditing ? (
          <textarea
            value={content}
            onChange={(e) => {
              setContent(e.target.value);
            }}
            onBlur={() => setIsEditing(false)}
            className="w-full h-full font-mono p-2 resize-none"
            placeholder="Start writing..."
          />
        ) : (
          <div
            onClick={() => setIsEditing(true)}
            className="cursor-text h-full overflow-auto"
          >
            <MarkdownRenderer markdown={content} />
          </div>
        )}
      </div>
    </div>
  );
}
