import { X } from "lucide-react";

interface GlobalErrorPopupProps {
  message: string;
  onDismiss: () => void;
}

export const GlobalErrorPopup = ({
  message,
  onDismiss,
}: GlobalErrorPopupProps) => {
  return (
    <div className="fixed top-4 right-4 z-50 max-w-md bg-red-50 border border-red-200 rounded-lg shadow-lg p-4">
      <div className="flex items-start gap-3">
        <div className="flex-1">
          <h3 className="text-red-800 font-medium mb-1">Error</h3>
          <p className="text-red-600 text-sm">{message}</p>
        </div>
        <button
          onClick={onDismiss}
          className="flex-shrink-0 p-1 hover:bg-red-100 rounded-full transition-colors"
          aria-label="Dismiss error"
        >
          <X className="w-4 h-4 text-red-600" />
        </button>
      </div>
    </div>
  );
};
