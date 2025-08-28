import { ApiError } from "./errors";
import { backendClient } from "@/services/http";
import { type SearchBlockResponse } from "./types/searchBlocksResponse";

export const searchApi = {
  async searchBlocks(query: string): Promise<SearchBlockResponse> {
    try {
      const response = await backendClient.post(`/search/blocks`, {
        query: query,
      });
      return response.data;
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },
};
