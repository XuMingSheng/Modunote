export interface PinnedItem {
  id: string;
  type: "block" | "canvas";
  title: string;
  parentId?: string;
  children: PinnedItem[];
  expanded: boolean;
}
