import { Milkdown, MilkdownProvider, useEditor } from "@milkdown/react";
import { Crepe } from "@milkdown/crepe";

interface CrepeEditorProps {
  initialContent: string;
  onUpdate: (content: string) => void;
}

interface MilkdownEditorProps {
  initialContent: string;
  onUpdate: (content: string) => void;
}

const CrepeEditor = ({ initialContent, onUpdate }: CrepeEditorProps) => {
  useEditor(
    (root) => {
      const crepe = new Crepe({
        root,
        defaultValue: initialContent,
      });

      crepe.on((api) => {
        api.markdownUpdated((ctx, markdown, prevMarkdown) => {
          onUpdate(markdown);
        });
      });

      return crepe;
    },
    [initialContent]
  );

  return <Milkdown />;
};

export const MilkdownEditor = ({
  initialContent,
  onUpdate,
}: MilkdownEditorProps) => {
  return (
    <MilkdownProvider>
      <CrepeEditor initialContent={initialContent} onUpdate={onUpdate} />
    </MilkdownProvider>
  );
};
