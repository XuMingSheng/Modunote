import { ApiError } from "@/api/errors";
import { backendClient } from "@/services/http";
import { type SearchBlocksResponse } from "./types/searchBlocksResponse";

export const searchApi = {
  async searchBlocks(query: string): Promise<SearchBlocksResponse> {
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
