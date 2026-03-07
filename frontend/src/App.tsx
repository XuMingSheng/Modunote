import { useState } from "react";
import { FunctionalSidebar } from "./components/FuncitonalSidebar";
import { BlockEditorPage } from "./pages/BlockEditorPage";
import { GlobalErrorPopup } from "./components/GlobalErrorPopup";
import { useAppStore } from "./store/useAppStore";

export type PageId = "block-editor-page" | "placeholder";

function AppContent() {
  const [pageId, setPageId] = useState<PageId>("block-editor-page");
  const error = useAppStore((state) => state.error);
  const setError = useAppStore((state) => state.setError);

  return (
    <div className="flex min-h-screen bg-gray-100">
      <FunctionalSidebar activePageId={pageId} onChange={setPageId} />
      <main className="flex-1 ml-14 transition-all duration-300">
        {pageId === "block-editor-page" && <BlockEditorPage />}
        {pageId === "placeholder" && (
          <div>Placeholder Page (To be implemented)</div>
        )}
      </main>
      {error && (
        <GlobalErrorPopup message={error} onDismiss={() => setError(null)} />
      )}
    </div>
  );
}

function App() {
  return <AppContent />;
}

export default App;
