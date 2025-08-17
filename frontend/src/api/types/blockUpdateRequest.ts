import { type BlockLink } from "./blockLink";

export interface BlockUpdateRequest {
  title?: string;
  content?: string;
  parentBlocks?: BlockLink[];
  childBlocks?: BlockLink[];
  relatedBlocks?: BlockLink[];
}
