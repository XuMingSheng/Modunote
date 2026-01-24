export interface CreateDirectionalLinkResponse {
  id: string;
  blockFromId: string;
  blockToId: string;
}

export interface CreateRelatedLinkResponse {
  id: string;
  blockIds: string[];
}
