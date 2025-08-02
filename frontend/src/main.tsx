import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.jsx";
import { BlocksProvider } from "./context/BlocksContext.js";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BlocksProvider>
      <App />
    </BlocksProvider>
  </StrictMode>
);
