import { useRef, useState, useEffect } from "react";
import { Download, Upload } from "lucide-react";
import { exportImportApi } from "@/api/export-import/exportImportApi";
import { type ImportResponse } from "@/api/export-import/types/importResponse";
import { useAppStore } from "@/store/useAppStore";

function formatImportSummary(summary: ImportResponse): string {
  return (
    `Blocks: ${summary.blocksInserted} inserted, ${summary.blocksUpdated} updated, ${summary.blocksSkipped} skipped\n` +
    `Directional links: ${summary.dirLinksInserted} inserted, ${summary.dirLinksSkipped} skipped\n` +
    `Related links: ${summary.relatedLinksInserted} inserted, ${summary.relatedLinksSkipped} skipped`
  );
}

export const ExportImportToolbar = () => {
  const setError = useAppStore((state) => state.setError);
  const loadOpenedBlocks = useAppStore((state) => state.loadOpenedBlocks);

  const [exporting, setExporting] = useState(false);
  const [importing, setImporting] = useState(false);
  const [importSummary, setImportSummary] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (!importSummary) return;
    const timer = setTimeout(() => setImportSummary(null), 4000);
    return () => clearTimeout(timer);
  }, [importSummary]);

  const handleExport = async () => {
    setExporting(true);
    try {
      const blob = await exportImportApi.exportAll();
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `modunote-export-${new Date().toISOString()}.zip`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (error) {
      setError(
        `Export failed: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    } finally {
      setExporting(false);
    }
  };

  const handleFileSelected = async (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    const file = e.target.files?.[0];
    if (!file) return;
    e.target.value = "";

    setImporting(true);
    setImportSummary(null);
    try {
      const summary = await exportImportApi.importZip(file);
      setImportSummary(formatImportSummary(summary));
      await loadOpenedBlocks();
    } catch (error) {
      setError(
        `Import failed: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    } finally {
      setImporting(false);
    }
  };

  return (
    <>
      {importSummary && (
        <div className="fixed bottom-4 left-1/2 -translate-x-1/2 z-50 bg-gray-900 text-white text-xs rounded-lg px-4 py-3 shadow-lg whitespace-pre-line">
          {importSummary}
        </div>
      )}
      <div className="flex items-center justify-center gap-1 w-full border-t border-gray-300 py-2">
        <button
          onClick={handleExport}
          disabled={exporting}
          className="flex items-center justify-center w-10 h-10 rounded-md hover:bg-gray-100 disabled:opacity-50 transition-colors"
          title={exporting ? "Exporting…" : "Export All"}
        >
          <Download className="w-5 h-5" />
        </button>
        <button
          onClick={() => fileInputRef.current?.click()}
          disabled={importing}
          className="flex items-center justify-center w-10 h-10 rounded-md hover:bg-gray-100 disabled:opacity-50 transition-colors"
          title={importing ? "Importing…" : "Import from .zip"}
        >
          <Upload className="w-5 h-5" />
        </button>
        <input
          ref={fileInputRef}
          type="file"
          accept=".zip"
          className="hidden"
          onChange={handleFileSelected}
        />
      </div>
    </>
  );
};
