import { ApiError } from "./errors";
import { type Block } from "./types/block";
import { type BlockCreateRequest } from "./types/blockCreateRequest";
import { type BlockUpdateRequest } from "./types/blockUpdateRequest";
import { type BlockSearchResponse } from "./types/blockSearchResponse";
import type { BlockGetOpenResponse } from "./types/blockGetOpenResponse";
import type { BlockGetLinksRepsonse } from "./types/blockGetLinksResponse";

export let blocksDb: Block[] = [
  {
    id: "b1",
    title: "Introduction to Fourier Series",
    content: `
# Fourier Series

A **Fourier series** is a way to represent a (periodic) function as a sum of sines and cosines:

$$
f(x) = a_0 + \\sum_{n=1}^{\\infty} [a_n \\cos(nx) + b_n \\sin(nx)]
$$

See also: [[Block Two]]

- For derivation, see [[Block Three]]
- For applications, see [[Block Four]]
    `.trim(),
    parentBlocks: [],
    childBlocks: [
      { id: "b2", title: "Relation: Orthogonality" },
      { id: "b3", title: "Proof: Fourier Coefficients" },
      { id: "b4", title: "Applications: Signal Processing" },
    ],
    relatedBlocks: [{ id: "b5", title: "Chebyshev Polynomials" }],
  },
  {
    id: "b2",
    title: "Relation: Orthogonality",
    content: `
## Orthogonality of Trigonometric Functions

The set $\\{1, \\sin(nx), \\cos(nx)\\}$ forms an **orthogonal basis** over $[-\\pi, \\pi]$:

$$
\\int_{-\\pi}^{\\pi} \\sin(nx) \\cos(mx) dx = 0
$$

See [[Introduction to Fourier Series]]
    `.trim(),
    parentBlocks: [{ id: "b1", title: "Introduction to Fourier Series" }],
    childBlocks: [],
    relatedBlocks: [{ id: "b5", title: "Chebyshev Polynomials" }],
  },
  {
    id: "b3",
    title: "Proof: Fourier Coefficients",
    content: `
## Derivation of Coefficients

To find $a_n$ and $b_n$:

\`\`\`math
a_n = \\frac{1}{\\pi} \\int_{-\\pi}^{\\pi} f(x) \\cos(nx)\\, dx
b_n = \\frac{1}{\\pi} \\int_{-\\pi}^{\\pi} f(x) \\sin(nx)\\, dx
\`\`\`

For LaTeX inside code blocks, use triple backticks with "math".
    `.trim(),
    parentBlocks: [{ id: "b1", title: "Introduction to Fourier Series" }],
    childBlocks: [],
    relatedBlocks: [],
  },
  {
    id: "b4",
    title: "Applications: Signal Processing",
    content: `
## Applications

Fourier series are fundamental in **signal processing**:

- Audio compression (e.g., MP3)
- Image analysis (see \`Fourier Transform\`)
- Solving PDEs

### Example: Decompose a Square Wave

![Square wave](https://upload.wikimedia.org/wikipedia/commons/7/7c/Fourier_series_and_transform.gif)

The Fourier series of a square wave:

$$
f(x) = \\frac{4}{\\pi} \\sum_{n=1,3,5,\\dots}^\\infty \\frac{\\sin(nx)}{n}
$$
    `.trim(),
    parentBlocks: [{ id: "b1", title: "Introduction to Fourier Series" }],
    childBlocks: [],
    relatedBlocks: [{ id: "b6", title: "Fourier Transform" }],
  },
  {
    id: "b5",
    title: "Chebyshev Polynomials",
    content: `
# Chebyshev Polynomials

Orthogonal polynomials useful in **approximation theory**.

For more, see [[Relation: Orthogonality]]
    `.trim(),
    parentBlocks: [],
    childBlocks: [],
    relatedBlocks: [{ id: "b2", title: "Relation: Orthogonality" }],
  },
  {
    id: "b6",
    title: "Fourier Transform",
    content: `
# Fourier Transform

Generalizes Fourier series to non-periodic functions.

$$
\\hat{f}(\\xi) = \\int_{-\\infty}^{\\infty} f(x)\\, e^{-2\\pi i x \\xi} dx
$$

- See also [[Applications: Signal Processing]]
    `.trim(),
    parentBlocks: [],
    childBlocks: [],
    relatedBlocks: [{ id: "b4", title: "Applications: Signal Processing" }],
  },
  {
    id: "b7",
    title: "Parseval's Theorem",
    content: `
# Parseval's Theorem

Parseval's theorem states:

$$
\\frac{1}{\\pi} \\int_{-\\pi}^{\\pi} |f(x)|^2 dx = a_0^2 + \\sum_{n=1}^{\\infty} (a_n^2 + b_n^2)
$$

Useful for energy computations in signal processing.

See also: [[Introduction to Fourier Series]], [[Applications: Signal Processing]]
    `.trim(),
    parentBlocks: [{ id: "b1", title: "Introduction to Fourier Series" }],
    childBlocks: [],
    relatedBlocks: [{ id: "b4", title: "Applications: Signal Processing" }],
  },
  {
    id: "b8",
    title: "Convolution Theorem",
    content: `
# Convolution Theorem

Convolution in time domain equals multiplication in frequency domain:

$$
(f * g)(t) \\xrightarrow{\\mathcal{F}} F(\\omega) G(\\omega)
$$

Where $\\mathcal{F}$ is the Fourier transform.

See [[Fourier Transform]]
    `.trim(),
    parentBlocks: [{ id: "b6", title: "Fourier Transform" }],
    childBlocks: [],
    relatedBlocks: [],
  },
  {
    id: "b9",
    title: "Dirichlet Conditions",
    content: `
# Dirichlet Conditions

A function $f(x)$ can be expanded in a Fourier series if:

- It is absolutely integrable over $[a, a+2\\pi]$
- Has a finite number of maxima and minima
- Has a finite number of discontinuities

For more details, see [[Block Three]]
    `.trim(),
    parentBlocks: [],
    childBlocks: [{ id: "b3", title: "Proof: Fourier Coefficients" }],
    relatedBlocks: [],
  },
  {
    id: "b10",
    title: "Laplace Transform",
    content: `
# Laplace Transform

$$
F(s) = \\int_0^{\\infty} f(t) e^{-st} dt
$$

Used for solving linear ODEs with initial conditions.

Compare with [[Fourier Transform]]
    `.trim(),
    parentBlocks: [],
    childBlocks: [],
    relatedBlocks: [{ id: "b6", title: "Fourier Transform" }],
  },
  {
    id: "b11",
    title: "Gibbs Phenomenon",
    content: `
# Gibbs Phenomenon

Truncated Fourier series exhibit overshoot near discontinuities:

$$
\\text{Overshoot} \\approx 9\\%
$$

See [[Block Four]] for visualization.
    `.trim(),
    parentBlocks: [{ id: "b4", title: "Applications: Signal Processing" }],
    childBlocks: [],
    relatedBlocks: [],
  },
  {
    id: "b12",
    title: "Discrete Fourier Transform (DFT)",
    content: `
# Discrete Fourier Transform

$$
X_k = \\sum_{n=0}^{N-1} x_n e^{-2\\pi i k n / N}
$$

- Used in digital signal processing
- Fast Fourier Transform (FFT) computes this efficiently

See [[Block Four]], [[Fourier Transform]]
    `.trim(),
    parentBlocks: [
      { id: "b4", title: "Applications: Signal Processing" },
      { id: "b6", title: "Fourier Transform" },
    ],
    childBlocks: [],
    relatedBlocks: [],
  },
  {
    id: "b13",
    title: "Heaviside Step Function",
    content: `
# Heaviside Step Function

$$
u(t) = 
\\begin{cases}
0 & t < 0 \\\\
1 & t \\geq 0
\\end{cases}
$$

Used in Laplace and Fourier analysis.
    `.trim(),
    parentBlocks: [],
    childBlocks: [],
    relatedBlocks: [{ id: "b10", title: "Laplace Transform" }],
  },
  {
    id: "b14",
    title: "Hilbert Transform",
    content: `
# Hilbert Transform

$$
\\mathcal{H}\\{f\\}(t) = \\frac{1}{\\pi} \\text{P.V.} \\int_{-\\infty}^{\\infty} \\frac{f(\\tau)}{t-\\tau} d\\tau
$$

Used for analytic signals and envelope detection.

See [[Fourier Transform]]
    `.trim(),
    parentBlocks: [{ id: "b6", title: "Fourier Transform" }],
    childBlocks: [],
    relatedBlocks: [],
  },
  {
    id: "b15",
    title: "Bessel Functions",
    content: `
# Bessel Functions

Solutions to Bessel's differential equation:

$$
x^2 y'' + x y' + (x^2 - \\nu^2) y = 0
$$

Arise in Fourier-Bessel series and physics.

Related: [[Chebyshev Polynomials]]
    `.trim(),
    parentBlocks: [],
    childBlocks: [],
    relatedBlocks: [{ id: "b5", title: "Chebyshev Polynomials" }],
  },
  {
    id: "b16",
    title: "Wave Equation Solution",
    content: `
# Wave Equation Solution

The general solution to the 1D wave equation:

$$
u(x, t) = F(x - ct) + G(x + ct)
$$

Can be expressed as a Fourier series for certain boundary conditions.

See [[Introduction to Fourier Series]]
    `.trim(),
    parentBlocks: [{ id: "b1", title: "Introduction to Fourier Series" }],
    childBlocks: [],
    relatedBlocks: [],
  },
];

export let openBlockIds = new Set<string>(["b1", "b2", "b3"]);
export let pinnedBlockIds = new Set<string>([
  "b1",
  "b4",
  "b9",
  "b11",
  "b6",
  "b12",
  "b14",
  "b16",
  "b3",
  "b8",
]);

// Mock closure table for efficient hierarchy queries
// Each entry represents a specific path: ancestor -> descendant with path as identifier
export interface ClosureTableEntry {
  ancestor: string;
  descendant: string;
  pathLength: number;
  path: string[]; // Full path from ancestor to descendant (including both endpoints)
  pathId: string; // Unique identifier for this path (e.g., "b1->b4->b11")
}

export let closureTable: ClosureTableEntry[] = [
  // Self-references (pathLength 0)
  {
    ancestor: "b1",
    descendant: "b1",
    pathLength: 0,
    path: ["b1"],
    pathId: "b1",
  },
  {
    ancestor: "b2",
    descendant: "b2",
    pathLength: 0,
    path: ["b2"],
    pathId: "b2",
  },
  {
    ancestor: "b3",
    descendant: "b3",
    pathLength: 0,
    path: ["b3"],
    pathId: "b3",
  },
  {
    ancestor: "b4",
    descendant: "b4",
    pathLength: 0,
    path: ["b4"],
    pathId: "b4",
  },
  {
    ancestor: "b5",
    descendant: "b5",
    pathLength: 0,
    path: ["b5"],
    pathId: "b5",
  },
  {
    ancestor: "b6",
    descendant: "b6",
    pathLength: 0,
    path: ["b6"],
    pathId: "b6",
  },
  {
    ancestor: "b7",
    descendant: "b7",
    pathLength: 0,
    path: ["b7"],
    pathId: "b7",
  },
  {
    ancestor: "b8",
    descendant: "b8",
    pathLength: 0,
    path: ["b8"],
    pathId: "b8",
  },
  {
    ancestor: "b9",
    descendant: "b9",
    pathLength: 0,
    path: ["b9"],
    pathId: "b9",
  },
  {
    ancestor: "b10",
    descendant: "b10",
    pathLength: 0,
    path: ["b10"],
    pathId: "b10",
  },
  {
    ancestor: "b11",
    descendant: "b11",
    pathLength: 0,
    path: ["b11"],
    pathId: "b11",
  },
  {
    ancestor: "b12",
    descendant: "b12",
    pathLength: 0,
    path: ["b12"],
    pathId: "b12",
  },
  {
    ancestor: "b13",
    descendant: "b13",
    pathLength: 0,
    path: ["b13"],
    pathId: "b13",
  },
  {
    ancestor: "b14",
    descendant: "b14",
    pathLength: 0,
    path: ["b14"],
    pathId: "b14",
  },
  {
    ancestor: "b15",
    descendant: "b15",
    pathLength: 0,
    path: ["b15"],
    pathId: "b15",
  },
  {
    ancestor: "b16",
    descendant: "b16",
    pathLength: 0,
    path: ["b16"],
    pathId: "b16",
  },

  // Direct parent-child relationships (pathLength 1)
  {
    ancestor: "b1",
    descendant: "b2",
    pathLength: 1,
    path: ["b1", "b2"],
    pathId: "b1->b2",
  },
  {
    ancestor: "b1",
    descendant: "b3",
    pathLength: 1,
    path: ["b1", "b3"],
    pathId: "b1->b3",
  },
  {
    ancestor: "b1",
    descendant: "b4",
    pathLength: 1,
    path: ["b1", "b4"],
    pathId: "b1->b4",
  },
  {
    ancestor: "b1",
    descendant: "b7",
    pathLength: 1,
    path: ["b1", "b7"],
    pathId: "b1->b7",
  },
  {
    ancestor: "b1",
    descendant: "b16",
    pathLength: 1,
    path: ["b1", "b16"],
    pathId: "b1->b16",
  },
  {
    ancestor: "b4",
    descendant: "b11",
    pathLength: 1,
    path: ["b4", "b11"],
    pathId: "b4->b11",
  },
  {
    ancestor: "b4",
    descendant: "b12",
    pathLength: 1,
    path: ["b4", "b12"],
    pathId: "b4->b12",
  },
  {
    ancestor: "b6",
    descendant: "b8",
    pathLength: 1,
    path: ["b6", "b8"],
    pathId: "b6->b8",
  },
  {
    ancestor: "b6",
    descendant: "b12",
    pathLength: 1,
    path: ["b6", "b12"],
    pathId: "b6->b12",
  },
  {
    ancestor: "b6",
    descendant: "b14",
    pathLength: 1,
    path: ["b6", "b14"],
    pathId: "b6->b14",
  },
  {
    ancestor: "b9",
    descendant: "b3",
    pathLength: 1,
    path: ["b9", "b3"],
    pathId: "b9->b3",
  },

  // Indirect relationships - Multiple paths possible
  {
    ancestor: "b1",
    descendant: "b11",
    pathLength: 2,
    path: ["b1", "b4", "b11"],
    pathId: "b1->b4->b11",
  },
  {
    ancestor: "b1",
    descendant: "b12",
    pathLength: 2,
    path: ["b1", "b4", "b12"],
    pathId: "b1->b4->b12",
  },
  // Note: b12 has multiple parents (b4 and b6), so there would be another path: b6->b12
  // This creates multiple paths between b1 and b12 if b1 is also connected to b6
];

function delay<T>(data: T, ms = 300): Promise<T> {
  return new Promise((resolve) => setTimeout(() => resolve(data), ms));
}

// Helper functions for closure table maintenance
const addToClosureTable = (ancestorId: string, descendantId: string) => {
  const directPath = [ancestorId, descendantId];
  const directPathId = `${ancestorId}->${descendantId}`;

  // Add direct relationship if it doesn't exist
  if (!closureTable.find((entry) => entry.pathId === directPathId)) {
    closureTable.push({
      ancestor: ancestorId,
      descendant: descendantId,
      pathLength: 1,
      path: directPath,
      pathId: directPathId,
    });
  }

  // Add transitive relationships through all existing paths
  // All paths ending at ancestor can be extended to descendant
  const pathsToAncestor = closureTable.filter(
    (entry) => entry.descendant === ancestorId
  );

  for (const ancestorPath of pathsToAncestor) {
    const newPath = [...ancestorPath.path, descendantId];
    const newPathId = newPath.join("->");
    const newPathLength = ancestorPath.pathLength + 1;

    // Only add if this exact path doesn't already exist
    if (!closureTable.find((entry) => entry.pathId === newPathId)) {
      closureTable.push({
        ancestor: ancestorPath.ancestor,
        descendant: descendantId,
        pathLength: newPathLength,
        path: newPath,
        pathId: newPathId,
      });
    }
  }

  // All paths starting from descendant can be extended from ancestor
  const pathsFromDescendant = closureTable.filter(
    (entry) => entry.ancestor === descendantId
  );

  for (const descendantPath of pathsFromDescendant) {
    const newPath = [ancestorId, ...descendantPath.path];
    const newPathId = newPath.join("->");
    const newPathLength = descendantPath.pathLength + 1;

    // Only add if this exact path doesn't already exist
    if (!closureTable.find((entry) => entry.pathId === newPathId)) {
      closureTable.push({
        ancestor: ancestorId,
        descendant: descendantPath.descendant,
        pathLength: newPathLength,
        path: newPath,
        pathId: newPathId,
      });
    }
  }

  // Combine all ancestor paths with all descendant paths
  for (const ancestorPath of pathsToAncestor) {
    for (const descendantPath of pathsFromDescendant) {
      const newPath = [...ancestorPath.path, ...descendantPath.path.slice(1)]; // Remove duplicate middle node
      const newPathId = newPath.join("->");
      const newPathLength =
        ancestorPath.pathLength + 1 + descendantPath.pathLength;

      // Only add if this exact path doesn't already exist
      if (!closureTable.find((entry) => entry.pathId === newPathId)) {
        closureTable.push({
          ancestor: ancestorPath.ancestor,
          descendant: descendantPath.descendant,
          pathLength: newPathLength,
          path: newPath,
          pathId: newPathId,
        });
      }
    }
  }
};

const removeFromClosureTable = (ancestorId: string, descendantId: string) => {
  // Remove all paths that contain this direct edge
  const directEdge = `${ancestorId}->${descendantId}`;

  closureTable = closureTable.filter((entry) => {
    // Remove direct relationship
    if (entry.pathId === directEdge) {
      return false;
    }

    // Remove any path that contains this edge (pathId contains the edge pattern)
    if (entry.pathId.includes(directEdge)) {
      return false;
    }

    // Also check if the path array contains the consecutive nodes
    for (let i = 0; i < entry.path.length - 1; i++) {
      if (entry.path[i] === ancestorId && entry.path[i + 1] === descendantId) {
        return false;
      }
    }

    return true;
  });

  // Rebuild transitive closure to ensure consistency
  rebuildTransitiveClosure();
};

const rebuildTransitiveClosure = () => {
  // Keep only self-references and direct relationships (pathLength <= 1)
  const directRelations = closureTable.filter((entry) => entry.pathLength <= 1);
  closureTable = [...directRelations];

  // Get all direct edges to rebuild from
  const directEdges = closureTable.filter((entry) => entry.pathLength === 1);

  // Rebuild transitive relationships by combining paths
  let changed = true;
  while (changed) {
    changed = false;
    const currentSize = closureTable.length;

    // For each existing path, try to extend it with each direct edge
    const currentPaths = [...closureTable];

    for (const path1 of currentPaths) {
      for (const path2 of currentPaths) {
        // Can combine path1 -> path2 if path1 ends where path2 starts
        if (
          path1.descendant === path2.ancestor &&
          path1.ancestor !== path2.descendant
        ) {
          const newPath = [...path1.path.slice(0, -1), ...path2.path];
          const newPathId = newPath.join("->");
          const newPathLength = path1.pathLength + path2.pathLength;

          // Only add if this exact path doesn't already exist
          if (!closureTable.find((entry) => entry.pathId === newPathId)) {
            closureTable.push({
              ancestor: path1.ancestor,
              descendant: path2.descendant,
              pathLength: newPathLength,
              path: newPath,
              pathId: newPathId,
            });
          }
        }
      }
    }

    if (closureTable.length > currentSize) {
      changed = true;
    }
  }
};

export const blockApi = {
  async get(id: string): Promise<Block> {
    const block = blocksDb.find((b) => b.id === id);
    if (block) {
      return delay(block);
    }
    throw new ApiError(404, "blockApi.get");
  },

  async getLinks(id: string): Promise<BlockGetLinksRepsonse> {
    const block = blocksDb.find((b) => b.id === id);
    if (block) {
      return delay({
        id: block.id,
        title: block.title,
        parentBlocks: block.parentBlocks,
        childBlocks: block.childBlocks,
        relatedBlocks: block.relatedBlocks,
      });
    }
    throw new ApiError(404, "blockApi.get");
  },

  async search(query: string): Promise<BlockSearchResponse> {
    const q = query.toLowerCase();
    const results = blocksDb
      .filter(
        (b) =>
          b.title.toLowerCase().includes(q) ||
          b.content.toLowerCase().includes(q)
      )
      .map((b) => ({
        id: b.id,
        title: b.title,
        matchedContent: b.content.slice(0, 100), // Return first 100 chars of content
      }));
    return delay(results);
  },

  async create(
    request: BlockCreateRequest = {
      title: `New Block ${Date.now()}`,
    }
  ): Promise<Block> {
    const newBlock: Block = {
      id: `b${Date.now()}`,
      title: request.title,
      content: request.content ?? "",
      parentBlocks: [],
      childBlocks: [],
      relatedBlocks: [],
    };
    blocksDb.push(newBlock);
    openBlockIds.add(newBlock.id);

    // Add self-reference to closure table
    closureTable.push({
      ancestor: newBlock.id,
      descendant: newBlock.id,
      pathLength: 0,
      path: [newBlock.id],
      pathId: newBlock.id,
    });

    return delay(newBlock);
  },

  async update(id: string, request: BlockUpdateRequest): Promise<Block> {
    const block = blocksDb.find((b) => b.id === id) ?? null;
    if (block) {
      if (request.parentBlocks) {
        // Remove old parent relationships from closure table
        for (const oldParent of block.parentBlocks) {
          removeFromClosureTable(oldParent.id, id);
        }

        // Remove from block references
        for (const parentBlock of blocksDb) {
          const index = parentBlock.childBlocks.findIndex((b) => b.id === id);
          if (index !== -1) {
            parentBlock.childBlocks.splice(index, 1);
          }
        }

        // Add new parent relationships
        for (const link of request.parentBlocks) {
          const parentBlock = blocksDb.find((b) => b.id === link.id);
          if (parentBlock) {
            parentBlock.childBlocks.push({ id: block.id, title: block.title });
            // Add to closure table
            addToClosureTable(link.id, id);
          }
        }
      }

      if (request.childBlocks) {
        // Remove old child relationships from closure table
        for (const oldChild of block.childBlocks) {
          removeFromClosureTable(id, oldChild.id);
        }

        // Remove from block references
        for (const childBlock of blocksDb) {
          const index = childBlock.parentBlocks.findIndex((b) => b.id === id);
          if (index !== -1) {
            childBlock.parentBlocks.splice(index, 1);
          }
        }

        // Add new child relationships
        for (const link of request.childBlocks) {
          const childBlock = blocksDb.find((b) => b.id === link.id);
          if (childBlock) {
            childBlock.parentBlocks.push({ id: block.id, title: block.title });
            // Add to closure table
            addToClosureTable(id, link.id);
          }
        }
      }

      if (request.relatedBlocks) {
        for (const relatedBlock of blocksDb) {
          const index = relatedBlock.relatedBlocks.findIndex(
            (b) => b.id === id
          );
          if (index !== -1) {
            relatedBlock.relatedBlocks.splice(index, 1);
          }
        }

        for (const link of request.relatedBlocks) {
          const relatedBlock = blocksDb.find((b) => b.id === link.id);
          if (relatedBlock) {
            relatedBlock.relatedBlocks.push({
              id: block.id,
              title: block.title,
            });
          }
        }
      }

      block.title = request.title ?? block.title;
      block.content = request.content ?? block.content;
      block.parentBlocks = request.parentBlocks ?? block.parentBlocks;
      block.childBlocks = request.childBlocks ?? block.childBlocks;
      block.relatedBlocks = request.relatedBlocks ?? block.relatedBlocks;
      return delay(block);
    }
    throw new ApiError(404, "blockApi.update");
  },

  async delete(id: string): Promise<void> {
    // Remove all closure table entries involving this block
    closureTable = closureTable.filter(
      (entry) => entry.ancestor !== id && entry.descendant !== id
    );

    blocksDb = blocksDb.filter((b) => b.id !== id);
    for (const block of blocksDb) {
      let index = block.parentBlocks.findIndex((b) => b.id === id);
      if (index !== -1) {
        block.parentBlocks.splice(index, 1);
      }
      index = block.childBlocks.findIndex((b) => b.id === id);
      if (index !== -1) {
        block.childBlocks.splice(index, 1);
      }
      index = block.relatedBlocks.findIndex((b) => b.id === id);
      if (index !== -1) {
        block.relatedBlocks.splice(index, 1);
      }
    }

    // Rebuild transitive closure to handle any broken paths
    rebuildTransitiveClosure();

    openBlockIds.delete(id);
    pinnedBlockIds.delete(id);
    return delay(undefined);
  },

  async getOpen(): Promise<BlockGetOpenResponse> {
    const openBlocks = blocksDb
      .filter((b) => openBlockIds.has(b.id))
      .map((b) => ({
        id: b.id,
        title: b.title,
      }));
    return delay(openBlocks);
  },

  async open(id: string): Promise<void> {
    openBlockIds.add(id);
    return delay(undefined);
  },

  async close(id: string): Promise<void> {
    openBlockIds.delete(id);
    return delay(undefined);
  },

  async pin(id: string): Promise<void> {
    pinnedBlockIds.add(id);
    return delay(undefined);
  },

  async unpin(id: string): Promise<void> {
    pinnedBlockIds.delete(id);
    return delay(undefined);
  },

  // Closure table query methods
  async isAncestor(ancestorId: string, descendantId: string): Promise<boolean> {
    const exists = closureTable.some(
      (entry) =>
        entry.ancestor === ancestorId &&
        entry.descendant === descendantId &&
        entry.pathLength > 0
    );
    console.log("isAncestor", ancestorId, descendantId);
    return delay(exists, 0);
  },

  async getAncestors(id: string): Promise<string[]> {
    const ancestors = closureTable
      .filter((entry) => entry.descendant === id && entry.pathLength > 0)
      .map((entry) => entry.ancestor);
    return delay(ancestors);
  },

  async getDescendants(id: string): Promise<string[]> {
    const descendants = closureTable
      .filter((entry) => entry.ancestor === id && entry.pathLength > 0)
      .map((entry) => entry.descendant);
    return delay(descendants);
  },

  async getPathLength(
    ancestorId: string,
    descendantId: string
  ): Promise<number | null> {
    const entry = closureTable.find(
      (entry) =>
        entry.ancestor === ancestorId && entry.descendant === descendantId
    );
    return delay(entry ? entry.pathLength : null);
  },

  async getClosureTable(): Promise<ClosureTableEntry[]> {
    return delay([...closureTable]);
  },

  // New path-aware query methods
  async getAllPaths(
    ancestorId: string,
    descendantId: string
  ): Promise<ClosureTableEntry[]> {
    const paths = closureTable.filter(
      (entry) =>
        entry.ancestor === ancestorId &&
        entry.descendant === descendantId &&
        entry.pathLength > 0
    );
    return delay(paths);
  },

  async getAllPathsOfNodes(nodeIds: string[]): Promise<ClosureTableEntry[]> {
    const idSet = new Set(nodeIds);
    const paths = closureTable.filter(
      (entry) =>
        idSet.has(entry.ancestor) &&
        idSet.has(entry.descendant) &&
        entry.pathLength > 0
    );
    return delay(paths);
  },

  async getShortestPath(
    ancestorId: string,
    descendantId: string
  ): Promise<ClosureTableEntry | null> {
    const paths = closureTable.filter(
      (entry) =>
        entry.ancestor === ancestorId &&
        entry.descendant === descendantId &&
        entry.pathLength > 0
    );

    if (paths.length === 0) return delay(null);

    const shortestPath = paths.reduce((shortest, current) =>
      current.pathLength < shortest.pathLength ? current : shortest
    );
    return delay(shortestPath);
  },

  async getPathsByLength(
    ancestorId: string,
    descendantId: string,
    pathLength: number
  ): Promise<ClosureTableEntry[]> {
    const paths = closureTable.filter(
      (entry) =>
        entry.ancestor === ancestorId &&
        entry.descendant === descendantId &&
        entry.pathLength === pathLength
    );
    return delay(paths);
  },
};
