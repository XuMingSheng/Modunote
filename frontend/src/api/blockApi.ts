import { ApiError } from "./errors";

let blocksDb: Block[] = [
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

let openBlockIds = new Set<string>(["b1", "b2", "b3"]);

export interface BlockLink {
  id: string;
  title: string;
}

export interface Block {
  id: string;
  title: string;
  content: string;

  parentBlocks: BlockLink[];
  childBlocks: BlockLink[];
  relatedBlocks: BlockLink[];
}

function delay<T>(data: T, ms = 300): Promise<T> {
  return new Promise((resolve) => setTimeout(() => resolve(data), ms));
}

export interface BlockCreateRequest {
  title: string;
  content?: string;
}

export interface BlockUpdateRequest {
  title?: string;
  content?: string;
  parentBlocks?: BlockLink[];
  childBlocks?: BlockLink[];
  relatedBlocks?: BlockLink[];
}

export interface BlockSearchResponseItem {
  id: string;
  title: string;
  matchedContent: string;
}
type BlockSearchResponse = BlockSearchResponseItem[];

export const blockApi = {
  async get(id: string): Promise<Block> {
    const block = blocksDb.find((b) => b.id === id);
    if (block) {
      return delay(block);
    }
    throw new ApiError(404, "blockApi.get");
  },

  async getOpen(): Promise<Block[]> {
    const openBlocks = blocksDb.filter((b) => openBlockIds.has(b.id));
    return delay(openBlocks);
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

  async create(request: BlockCreateRequest): Promise<Block> {
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
    return delay(newBlock);
  },

  async open(id: string): Promise<void> {
    openBlockIds.add(id);
    return delay(undefined);
  },

  async update(id: string, request: BlockUpdateRequest): Promise<Block> {
    const block = blocksDb.find((b) => b.id === id) ?? null;
    if (block) {
      if (request.parentBlocks) {
        for (const parentBlock of blocksDb) {
          const index = parentBlock.childBlocks.findIndex((b) => b.id === id);
          if (index !== -1) {
            parentBlock.childBlocks.splice(index, 1);
          }
        }

        for (const link of request.parentBlocks) {
          const parentBlock = blocksDb.find((b) => b.id === link.id);
          if (parentBlock) {
            parentBlock.childBlocks.push({ id: block.id, title: block.title });
          }
        }
      }

      if (request.childBlocks) {
        for (const childBlock of blocksDb) {
          const index = childBlock.parentBlocks.findIndex((b) => b.id === id);
          if (index !== -1) {
            childBlock.parentBlocks.splice(index, 1);
          }
        }

        for (const link of request.childBlocks) {
          const childBlock = blocksDb.find((b) => b.id === link.id);
          if (childBlock) {
            childBlock.parentBlocks.push({ id: block.id, title: block.title });
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

  async close(id: string): Promise<void> {
    openBlockIds.delete(id);
    return delay(undefined);
  },

  async delete(id: string): Promise<void> {
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

    openBlockIds.delete(id);
    return delay(undefined);
  },
};
