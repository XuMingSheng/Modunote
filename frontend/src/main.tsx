import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.jsx";
import "./index.css";
import { BlocksProvider } from "./context/BlocksContext.js";

const root = createRoot(document.getElementById("root")!);

root.render(
  <StrictMode>
    <BlocksProvider>
      <App />
    </BlocksProvider>
  </StrictMode>
);
