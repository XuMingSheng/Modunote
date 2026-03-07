import { ApiError } from "../errors";
import { backendClient } from "@/services/http";
import { type ImportResponse } from "./types/importResponse";

export const exportImportApi = {
  async exportAll(): Promise<Blob> {
    try {
      const response = await backendClient.get("/export", {
        responseType: "blob",
      });
      return response.data as Blob;
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },

  async importZip(file: File): Promise<ImportResponse> {
    try {
      const form = new FormData();
      form.append("file", file);
      const response = await backendClient.post("/import", form, {
        headers: { "Content-Type": undefined },
      });
      return response.data as ImportResponse;
    } catch (error) {
      throw ApiError.fromError(error);
    }
  },
};
