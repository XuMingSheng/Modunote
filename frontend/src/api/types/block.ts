import { type BlockLink } from "./blockLink";

export interface Block {
  id: string;
  title: string;
  content: string;

  parentBlocks: BlockLink[];
  childBlocks: BlockLink[];
  relatedBlocks: BlockLink[];
}
