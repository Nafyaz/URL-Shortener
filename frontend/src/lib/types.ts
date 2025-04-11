// Types for URL shortener API

export interface UrlRequest {
  url: string;
}

export interface UrlResponse {
  short_url: string;
  original_url: string;
}

// API error response
export interface ApiError {
  error: string;
}

// API response can be successful data or an error
export type ApiResponse<T> = T | ApiError;

// Type guard to check if response is an error
export function isApiError<T>(response: ApiResponse<T>): response is ApiError {
  return (response as ApiError).error !== undefined;
}