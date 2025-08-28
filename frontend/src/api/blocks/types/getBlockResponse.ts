export interface BlockLink {
  blockId: string;
  title: string;
}

export interface GetBlockResponse {
  id: string;
  title: string;
  content: string;

  parentBlocks: BlockLink[];
  childBlocks: BlockLink[];
  relatedBlocks: BlockLink[];
}
