export interface SearchBlocksResponseItem {
  id: string;
  title: string;
  // matchedContent: string;
}

export interface SearchBlocksResponse {
  blocks: SearchBlocksResponseItem[];
}
