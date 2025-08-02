import { useEffect, useRef, useState } from "react";
import { type BlockSaveStatus, type Block } from "@/context/BlocksContext";
import { useBlocks } from "@/context/BlocksContext";

const AUTO_SAVE_INTERVAL = 2000;

interface Props {
  blockId: string;
  content: string;
}

export function useAutoSave({ blockId, content }: Props) {
  const [saveStatus, setSaveStatus] = useState<BlockSaveStatus>("idle");
  const { updateBlock } = useBlocks();
  const lastSavedContent = useRef<string>(content);
  const saveTimeout = useRef<number | null>(null);

  useEffect(() => {
    if (content !== lastSavedContent.current) {
      setSaveStatus("saving");

      if (saveTimeout.current) {
        clearTimeout(saveTimeout.current);
      }

      saveTimeout.current = setTimeout(async () => {
        try {
          updateBlock(blockId, { content });
          lastSavedContent.current = content;
          setSaveStatus("saved");
        } catch (error) {
          setSaveStatus("error");
          console.error("Auto-save failed:", error);
        }
      }, AUTO_SAVE_INTERVAL);
    }

    return () => {
      if (saveTimeout.current) {
        clearTimeout(saveTimeout.current);
      }
    };
  }, [content]);

  return { saveStatus };
}
