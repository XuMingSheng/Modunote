import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import rehypePrism from "rehype-prism-plus";
import "katex/dist/katex.min.css";
import "prismjs/themes/prism-tomorrow.css";

interface Props {
  markdown: string;
}

export function MarkdownRenderer({ markdown }: Props) {
  return (
    <div className="prose prose-slate max-w-none dark:prose-invert">
      <ReactMarkdown
        remarkPlugins={[
          // Remark plugins (Markdown processing)
          remarkGfm, // GitHub Flavored Markdown (tables, strikethrough, etc.)
          remarkMath, // Parse math expressions
        ]}
        rehypePlugins={[
          // Rehype plugins (HTML processing)
          rehypeKatex, // Render math expressions
          [rehypePrism, { ignoreMissing: true }], // Syntax highlighting
        ]}
      >
        {markdown}
      </ReactMarkdown>
    </div>
  );
}
