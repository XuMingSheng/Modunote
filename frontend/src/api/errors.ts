import { isAxiosError } from "axios";

export class ApiError extends Error {
  public statusCode?: number;
  public data?: unknown;

  constructor(statusCode?: number, message?: string, data?: unknown) {
    super(message);
    this.name = "ApiError";
    this.statusCode = statusCode;
    this.data = data;
  }

  static fromError(error: unknown): ApiError {
    if (isAxiosError(error)) {
      const status = error.response?.status;
      const data = error.response?.data;
      const message = error.response?.statusText || error.message;

      return new ApiError(status, message, data);
    }

    if (error instanceof Error) {
      return new ApiError(undefined, error.message, error);
    }

    return new ApiError(undefined, "An unknown API error occurred.", error);
  }
}
