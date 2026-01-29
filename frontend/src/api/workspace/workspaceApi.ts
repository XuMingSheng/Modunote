import { ApiError } from "@/api/errors";
import { backendClient } from "@/services/http";
import type { GetOpenedBlocksResponse } from "./types/getOpenedBlocksResponse";
import type { OpenBlockRequest } from "./types/openBlockRequest";
import type { OpenBlockResponse } from "./types/openBlockResponse";

export const workspaceApi = {
  async getOpenedBlocks(): Promise<GetOpenedBlocksResponse> {
    try {
      const response = await backendClient.get("/workspace/opened-blocks");
      return response.data;
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async openBlock(request: OpenBlockRequest): Promise<OpenBlockResponse> {
    try {
      const response = await backendClient.post(
        "/workspace/opened-blocks",
        request,
      );
      return response.data;
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async closeBlock(id: string) {
    try {
      await backendClient.delete(`workspace/opened-blocks/${id}`);
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },
};
