import type { BlockLink } from "./blockLink";

export interface BlockGetLinksRepsonse {
  id: string;
  title: string;

  parentBlocks: BlockLink[];
  childBlocks: BlockLink[];
  relatedBlocks: BlockLink[];
}
