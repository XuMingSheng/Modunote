export class ApiError extends Error {
  public statusCode: number;
  public data?: unknown;

  constructor(statusCode: number, message: string, data?: unknown) {
    super(message);
    this.name = "ApiError";
    this.statusCode = statusCode;
    this.data = data;
  }
}
