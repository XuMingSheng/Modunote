import ReactMarkdown from "react-markdown";

interface Props {
  markdown: string;
}

export function MarkdownRenderer({ markdown }: Props) {
  return (
    <div className="prose prose-slate max-w-none dark:prose-invert">
      <ReactMarkdown>{markdown}</ReactMarkdown>
    </div>
  );
}
