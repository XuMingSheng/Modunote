import { useEffect, useRef, useState } from "react";

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

  const [activeView, setActiveView] = useState<"visual" | "raw">("visual");
  const [contentDraft, setContentDraft] = useState(loadedContent ?? "");
  const [visualReloadToken, setVisualReloadToken] = useState(0);
  const [contentReadyId, setContentReadyId] = useState<string | null>(null);

  const activeBlockRef = useRef<Block | null>(null);
  const autoSaveRef = useRef<any>(null);

  const updateBlock = async (
    blockId: string,
    updateData: UpdateBlockRequest,
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

  useEffect(() => {
    setContentDraft(loadedContent ?? "");
    setContentReadyId(loadedBlockId ?? null);
  }, [loadedBlockId, loadedContent]);

  if (!loadedBlockId) {
    activeBlockRef.current = null;

    return (
      <p className="text-gray-500">No block open. Create or import one.</p>
    );
  }

  activeBlockRef.current = {
    title: loadedTitle!,
    content: contentDraft,
  };
  const activeBlock = activeBlockRef.current;

  const handleChangeTitle = (title: string) => {
    activeBlock.title = title;
    autoSaveRef.current?.updateData({
      ...activeBlock,
    });
  };

  const handleChangeContent = (content: string) => {
    activeBlock.content = content;
    autoSaveRef.current?.updateData({
      ...activeBlock,
    });
    setContentDraft(content);
  };

  const handleSetView = (view: "visual" | "raw") => {
    if (view === activeView) return;
    if (view === "visual") {
      setVisualReloadToken((token) => token + 1);
    }
    setActiveView(view);
  };

  return (
    <div className="flex flex-col h-full">
      {/* Title Section */}
      <div className="border-b p-4 bg-white flex items-center gap-3">
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
        <div className="flex items-center gap-1 rounded-md border border-gray-200 bg-gray-50 p-1 text-xs">
          <button
            type="button"
            onClick={() => handleSetView("visual")}
            className={`px-2 py-1 rounded ${
              activeView === "visual"
                ? "bg-white text-gray-900 shadow-sm"
                : "text-gray-500 hover:text-gray-700"
            }`}
          >
            Visual
          </button>
          <button
            type="button"
            onClick={() => handleSetView("raw")}
            className={`px-2 py-1 rounded ${
              activeView === "raw"
                ? "bg-white text-gray-900 shadow-sm"
                : "text-gray-500 hover:text-gray-700"
            }`}
          >
            Markdown
          </button>
        </div>
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
        {contentReadyId !== loadedBlockId ? (
          <div className="p-4 text-sm text-gray-400">Loading...</div>
        ) : activeView === "visual" ? (
          <MilkdownEditor
            key={`${loadedBlockId}-${visualReloadToken}`}
            id={loadedBlockId}
            content={contentDraft}
            onUpdate={(updatedContent) => {
              handleChangeContent(updatedContent);
            }}
          />
        ) : (
          <textarea
            key={loadedBlockId}
            value={contentDraft}
            onChange={(e) => handleChangeContent(e.target.value)}
            className="w-full h-full p-4 font-mono text-sm text-gray-900 outline-none resize-none"
            spellCheck={false}
            placeholder="Write Markdown..."
          />
        )}
      </div>
    </div>
  );
};
