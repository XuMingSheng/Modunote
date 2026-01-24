export interface LinkedBlock {
  blockId: string;
  title: string;
}

export interface GetBlockParentsResponse {
  parentBlocks: LinkedBlock[];
}

export interface GetBlockChildrenResponse {
  childBlocks: LinkedBlock[];
}

export interface GetBlockRelatedLinkResposne {
  relatedBlocks: LinkedBlock[];
}
