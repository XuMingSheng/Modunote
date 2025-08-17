import { type ComponentType } from "react";
import { LayoutDashboard, GitBranch, FileText } from "lucide-react";
import { type PageId } from "@/App";

import { cn } from "../lib/utils";

interface Page {
  id: PageId;
  icon: ComponentType<{ className?: string }>;
  label: string;
}

const PAGES: Page[] = [
  { id: "workplace", icon: LayoutDashboard, label: "Workplace" },
  { id: "canvases", icon: FileText, label: "Canvases" },
  { id: "graph", icon: GitBranch, label: "Graph" },
];

interface FunctionalSidbarProps {
  activePageId: string;
  onChange: (id: PageId) => void;
}

export const FunctionalSidebar = ({
  activePageId,
  onChange,
}: FunctionalSidbarProps) => {
  return (
    <aside className="group fixed left-0 top-0 h-screen w-14 z-40 flex flex-col bg-gray-900 text-gray-100">
      <div className="flex-1 flex flex-col items-center py-4 space-y-3">
        {PAGES.map(({ id, icon: Icon, label }) => (
          <button
            key={id}
            onClick={() => onChange(id)}
            className={cn(
              "relative flex items-center justify-center w-10 h-10 rounded-md hover:bg-gray-800 transition-colors",
              activePageId === id ? "bg-gray-700" : ""
            )}
            title={label}
          >
            <Icon className="w-6 h-6" />
          </button>
        ))}
      </div>
      <div className="p-3 border-t border-gray-700 text-center text-xs text-gray-500">
        v0.1.0
      </div>
    </aside>
  );
};
