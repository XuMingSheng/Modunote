import { useState, useRef, useEffect } from "react";

interface AutoSaveProps {
  activeId: string;
  data: any;
  interval: number;
  onSave: () => Promise<boolean>;
}

type SaveStatus = "idle" | "saving" | "saved" | "error";

export const AutoSave = ({
  activeId,
  data,
  interval,
  onSave,
}: AutoSaveProps) => {
  const [status, setStatus] = useState<SaveStatus>("idle");
  const lastSavedState = useRef({
    activeId,
    data,
  });
  const saveTimeout = useRef<number | null>(null);

  useEffect(() => {
    // Check if the active ID has changed, which means we're on a new document
    if (activeId !== lastSavedState.current.activeId) {
      if (saveTimeout.current) {
        clearTimeout(saveTimeout.current);
      }

      lastSavedState.current.activeId = activeId;
      lastSavedState.current.data = data;
      setStatus("idle");
    }
    // Check if the data has changed from the last saved state
    else if (data !== lastSavedState.current.data) {
      if (saveTimeout.current) {
        clearTimeout(saveTimeout.current);
      }

      setStatus("saving");
      saveTimeout.current = setTimeout(saveData, interval);
    }

    return () => {
      if (saveTimeout.current) {
        clearTimeout(saveTimeout.current);
      }
    };
  }, [activeId, data]);

  const saveData = async () => {
    const success = await onSave();
    if (success) {
      lastSavedState.current.data = {
        activeId,
        data,
      };
      setStatus("saved");
    } else {
      setStatus("error");
    }
  };

  return (
    <p className="text-xs text-gray-400 mt-1">
      {status === "idle"
        ? "idle..."
        : status === "saving"
          ? "Saving..."
          : status === "saved"
            ? "Saved"
            : status === "error"
              ? "Failed to save"
              : null}
    </p>
  );
};
