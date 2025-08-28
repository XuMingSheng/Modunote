import {
  useState,
  useRef,
  useEffect,
  type RefAttributes,
  useImperativeHandle,
} from "react";
import { isDeepEqual, clone } from "remeda";

interface AutoSaveProps extends RefAttributes<unknown> {
  activeId: string;
  data: any;
  interval: number;
  onSave: () => Promise<void>;
}

type SaveStatus = "idle" | "saving" | "saved" | "error";

export const AutoSave = ({
  ref,
  activeId,
  data,
  interval,
  onSave,
}: AutoSaveProps) => {
  const [status, setStatus] = useState<SaveStatus>("idle");
  const [latestData, setLatestData] = useState(data);
  const lastSavedState = useRef({
    activeId,
    data: clone(data),
  });
  const saveTimeout = useRef<number | null>(null);

  useImperativeHandle(ref, () => ({
    updateData: (latestData: typeof data) => {
      setLatestData(latestData);
    },
  }));

  useEffect(() => {
    // console.log(
    //   "passed activeId:",
    //   activeId,
    //   "savedId:",
    //   lastSavedState.current.activeId
    // );
    // console.log(
    //   "prop data:",
    //   data,
    //   "latest data:",
    //   latestData,
    //   "savedData:",
    //   lastSavedState.current.data
    // );

    // Check if the active ID has changed, which means we're on a new document
    if (activeId !== lastSavedState.current.activeId) {
      if (saveTimeout.current) {
        clearTimeout(saveTimeout.current);
      }

      lastSavedState.current.activeId = activeId;
      lastSavedState.current.data = clone(data);
      setLatestData(data);
      setStatus("idle");
    }
    // Check if the data has changed from the last saved state
    else if (!isDeepEqual(latestData, lastSavedState.current.data)) {
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
  }, [activeId, latestData]);

  const saveData = async () => {
    try {
      await onSave();
      lastSavedState.current.data = clone(latestData);
      setStatus("saved");
    } catch (_) {
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
