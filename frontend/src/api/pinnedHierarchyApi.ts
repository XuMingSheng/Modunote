import { type PinnedItem } from "@/api/types/pinnedItem";
import { blockApi, type ClosureTableEntry } from "./blockApi";
import { canvasApi } from "./canvasApi";
import { ApiError } from "./errors";
import { closureTable } from "./blockApi";
import { pinnedBlockIds } from "./blockApi";
import { pinnedCanvasIds } from "./canvasApi";
import { type Block } from "./types/block";
import { type Canvas } from "./types/canvas";

function delay<T>(data: T, ms = 300): Promise<T> {
  return new Promise((resolve) => setTimeout(() => resolve(data), ms));
}

export const pinnedHierarchyApi = {
  async get(): Promise<PinnedItem[]> {
    try {
      const hierarchy = await buildPinnedHierarchy();
      return delay(hierarchy);
    } catch (error) {
      throw new ApiError(500, "pinnedHierarchyApi.getHierarchy");
    }
  },

  async pinBlock(id: string): Promise<void> {
    return blockApi.pin(id);
  },

  async unpinBlock(id: string): Promise<void> {
    return blockApi.unpin(id);
  },

  async pinCanvas(id: string): Promise<void> {
    return canvasApi.pin(id);
  },

  async unpinCanvas(id: string): Promise<void> {
    return canvasApi.unpin(id);
  },
};

const buildPinnedHierarchy = async (): Promise<PinnedItem[]> => {
  const hierarchy: PinnedItem[] = [];

  await buildBlockHeirarchy(hierarchy);

  for (const canvasId of pinnedCanvasIds) {
    const canvas = await canvasApi.get(canvasId);
    let inserted: boolean = false;
    for (const node of hierarchy) {
      if (node.type !== "block") {
        continue;
      }
      inserted ||= await insertCanvas(canvas, node);
    }
    if (!inserted) {
      hierarchy.push({
        id: canvas.id,
        type: "canvas",
        title: canvas.name,
        children: [],
        expanded: false,
      });
    }
  }

  sortHeirarchy(hierarchy);

  return hierarchy;
};

const sortHeirarchy = (hierarchy: PinnedItem[]) => {
  for (const node of hierarchy) {
    sortHeirarchy(node.children);
  }
  hierarchy.sort((a, b) => b.children.length - a.children.length);
};

const buildBlockHeirarchy = async (hierarchy: PinnedItem[]) => {
  const paths = await blockApi.getAllPathsOfNodes([...pinnedBlockIds]);

  const rootIds = new Set(pinnedBlockIds);

  for (const path of paths) {
    if (rootIds.has(path.descendant)) {
      rootIds.delete(path.descendant);
    }
  }

  const pathsByRoot = new Map<string, ClosureTableEntry[]>();

  for (const rootId of rootIds) {
    pathsByRoot.set(rootId, []);
  }

  for (const path of paths) {
    if (rootIds.has(path.ancestor)) {
      pathsByRoot.get(path.ancestor)!.push(path);
    }
  }

  for (const rootId of rootIds) {
    const block = await blockApi.get(rootId);

    const newNode = {
      id: block.id,
      type: "block",
      title: block.title,
      children: [],
      expanded: false,
    } as PinnedItem;

    for (const path of pathsByRoot.get(rootId)!) {
      await insertPath(path, 1, newNode);
    }

    hierarchy.push(newNode);
  }

  return delay(hierarchy);
};

const insertPath = async (
  path: ClosureTableEntry,
  idx: number,
  node: PinnedItem
) => {
  while (idx < path.path.length && !pinnedBlockIds.has(path.path[idx])) {
    idx++;
  }

  if (idx >= path.path.length) {
    return;
  }

  const childId = path.path[idx];
  let childIndex = node.children.findIndex((c) => c.id == childId);

  if (childIndex == -1) {
    const block = await blockApi.get(childId);

    const newNode = {
      id: block.id,
      type: "block",
      title: block.title,
      children: [],
      expanded: false,
    } as PinnedItem;

    node.children.push(newNode);
    childIndex = node.children.length - 1;
  }

  insertPath(path, idx + 1, node.children[childIndex]);
};

// const insertBlock = async (block: Block, node: PinnedItem) => {
//   const newNode = {
//     id: block.id,
//     type: "block",
//     title: block.title,
//     children: [],
//     expanded: false,
//   } as PinnedItem;

//   for (const child of node.children) {
//     if (await blockApi.isAncestor(block.id, child.id)) {
//       childrenIds.push(child.id);
//     } else {
//       inserted ||= insertBlock(block, child);
//     }
//   }

//   for (const childId of childrenIds) {
//     const index = node.children.findIndex((c) => c.id == childId);
//     newNode.children.push(node.children[index]);
//     node.children.splice(index, 1);
//   }

//   if (isAncestor(node.id, block.id)) {
//     node.children.push(newNode);
//   }
// };

const insertCanvas = async (
  canvas: Canvas,
  node: PinnedItem
): Promise<boolean> => {
  let inserted: boolean = false;

  for (const child of node.children) {
    if (child.type !== "block") {
      continue;
    }
    inserted ||= await insertCanvas(canvas, child);
  }

  if (!inserted) {
    for (const blockId of canvas.blockPlacements.map((b) => b.blockId)) {
      if (await blockApi.isAncestor(node.id, blockId)) {
        node.children.push({
          id: canvas.id,
          type: "canvas",
          title: canvas.name,
          children: [],
          expanded: false,
        });
        inserted = true;
        break;
      }
    }
  }

  return inserted;
};
