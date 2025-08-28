import { ApiError } from "../errors";
import { backendClient } from "@/services/http";
import type {
  GetBlockParentsResponse,
  GetBlockChildrenResponse,
  GetBlockRelatedLinkResposne,
} from "./types/getBlockLinksResponse";
import type {
  CreateParentLinkRequest,
  CreateChildLinkRequest,
  CreateRelatedLinkRequest,
} from "./types/createBlockLinkRequest";

export const blockLinkApi = {
  async getParents(id: string): Promise<GetBlockParentsResponse> {
    try {
      const response = await backendClient.get(`/blocks/${id}/parents`);
      return response.data;
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async getChldren(id: string): Promise<GetBlockChildrenResponse> {
    try {
      const response = await backendClient.get(`/blocks/${id}/children`);
      return response.data;
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async getRelated(id: string): Promise<GetBlockRelatedLinkResposne> {
    try {
      const response = await backendClient.get(`/blocks/${id}/related-links`);
      return response.data;
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async createParentLink(
    blockId: string,
    parentBlockId: string
  ): Promise<void> {
    try {
      const request: CreateParentLinkRequest = { parentBlockId };
      await backendClient.post(`/blocks/${blockId}/parents`, request);
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async createChildLink(blockId: string, childBlockId: string): Promise<void> {
    try {
      const request: CreateChildLinkRequest = { childBlockId };
      await backendClient.post(`/blocks/${blockId}/children`, request);
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async createRelatedLink(
    blockId: string,
    relatedBlockId: string
  ): Promise<void> {
    try {
      const request: CreateRelatedLinkRequest = { relatedBlockId };
      await backendClient.post(`/blocks/${blockId}/related-links`, request);
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async deleteParentLink(
    blockId: string,
    parentBlockId: string
  ): Promise<void> {
    try {
      await backendClient.delete(`/blocks/${blockId}/parents/${parentBlockId}`);
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async deleteChildLink(blockId: string, childBlockId: string): Promise<void> {
    try {
      await backendClient.delete(`/blocks/${blockId}/children/${childBlockId}`);
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async deleteRelatedLink(
    blockId: string,
    relatedBlockId: string
  ): Promise<void> {
    try {
      await backendClient.delete(
        `/blocks/${blockId}/related-links/${relatedBlockId}`
      );
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },
};
