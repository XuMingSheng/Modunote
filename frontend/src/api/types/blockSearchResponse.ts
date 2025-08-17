export interface BlockSearchResponseItem {
  id: string;
  title: string;
  matchedContent: string;
}

export type BlockSearchResponse = BlockSearchResponseItem[];
