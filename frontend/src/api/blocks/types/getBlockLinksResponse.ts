export interface BlockLink {
  blockId: string;
  title: string;
}

export interface GetBlockParentsResponse {
  parentBlocks: BlockLink[];
}

export interface GetBlockChildrenResponse {
  childBlocks: BlockLink[];
}

export interface GetBlockRelatedLinkResposne {
  relatedBlocks: BlockLink[];
}
