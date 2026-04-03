import { Milkdown, MilkdownProvider, useEditor } from "@milkdown/react";
import { codeBlockConfig } from "@milkdown/kit/component/code-block";
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

      crepe.editor.config((ctx) => {
        ctx.update(codeBlockConfig.key, (prev) => ({
          ...prev,
          previewOnlyByDefault: true,
        }));
      });

      crepe.on((api) => {
        api.markdownUpdated((_ctx, markdown, _prevMarkdown) => {
          onUpdate(markdown);
        });
      });

      return crepe;
    },
    [id],
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
