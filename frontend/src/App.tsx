import { useState } from "react";
import { FunctionalSidebar } from "./components/FuncitonalSidebar";
import { BlocksPage } from "./pages/BlocksPage";
import { CanvasPage } from "./pages/CanvasPage";
import { ErrorProvider, useError } from "./context/ErrorContext";
import { GlobalErrorPopup } from "./components/GlobalErrorPopup";

export type PageId = "blocks" | "canvases" | "graph";

function AppContent() {
  const [pageId, setPageId] = useState<PageId>("blocks");
  const { error, setError } = useError();

  return (
    <div className="flex min-h-screen bg-gray-100">
      <FunctionalSidebar activePageId={pageId} onChange={setPageId} />
      <main className="flex-1 ml-14 transition-all duration-300">
        {pageId === "blocks" && <BlocksPage />}
        {pageId === "canvases" && <CanvasPage />}
        {pageId === "graph" && <div>Graph Page (To be implemented)</div>}
      </main>
      {error && (
        <GlobalErrorPopup message={error} onDismiss={() => setError(null)} />
      )}
    </div>
  );
}

function App() {
  return (
    <ErrorProvider>
      <AppContent />
    </ErrorProvider>
  );
}

export default App;
