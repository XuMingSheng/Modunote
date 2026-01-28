import { Milkdown, MilkdownProvider, useEditor } from "@milkdown/react";
import { Crepe } from "@milkdown/crepe";

interface CrepeEditorProps {
  id: string;
  content: string;
  onUpdate: (content: string) => void;
}

interface MilkdownEditorProps {
  id: string;
  content: string;
  onUpdate: (content: string) => void;
}

const CrepeEditor = ({ id, content, onUpdate }: CrepeEditorProps) => {
  useEditor(
    (root) => {
      const crepe = new Crepe({
        root,
        defaultValue: content,
      });

      crepe.on((api) => {
        api.markdownUpdated((ctx, markdown, prevMarkdown) => {
          onUpdate(markdown);
        });
      });

      return crepe;
    },
    [id, content],
  );

  return <Milkdown />;
};

export const MilkdownEditor = ({
  id,
  content,
  onUpdate,
}: MilkdownEditorProps) => {
  return (
    <MilkdownProvider>
      <CrepeEditor id={id} content={content} onUpdate={onUpdate} />
    </MilkdownProvider>
  );
};
