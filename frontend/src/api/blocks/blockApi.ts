import { ApiError } from "../errors";
import { backendClient } from "@/services/http";
import { type GetBlockResponse } from "./types/getBlockResponse";
import { type CreateBlockRequest } from "./types/createBlockRequest";
import { type CreateBlockResponse } from "./types/createBlockResponse";
import { type UpdateBlockRequest } from "./types/updateBlockRequest";
import { type UpdateBlockResponse } from "./types/updateBlockResponse";

export const blockApi = {
  async get(id: string): Promise<GetBlockResponse> {
    try {
      const response = await backendClient.get(`/blocks/${id}`);
      return response.data;
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async create(
    request: CreateBlockRequest = {
      title: `New Block ${Date.now()}`,
      content: "",
    }
  ): Promise<CreateBlockResponse> {
    try {
      const response = await backendClient.post(`/blocks`, request);
      return response.data;
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async update(
    id: string,
    request: UpdateBlockRequest
  ): Promise<UpdateBlockResponse> {
    try {
      const response = await backendClient.put(`/blocks/${id}`, request);
      return response.data;
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async delete(id: string): Promise<void> {
    try {
      await backendClient.delete(`/blocks/${id}`);
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },
};
