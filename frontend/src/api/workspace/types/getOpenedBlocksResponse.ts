export interface OpenedBlock {
  blockId: string;
  title: string;
  openedAt: string;
}

export interface GetOpenedBlocksResponse {
  openedBlocks: OpenedBlock[];
}
