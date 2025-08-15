import { browser } from '$app/environment';
import type { ApiResponse, UrlRequest, UrlResponse } from '$lib/types';

// API base URL - fallback to localhost in development
const API_BASE = browser
  ? import.meta.env.VITE_API_URL || 'http://localhost:3000'
  : 'http://backend:3000';

/**
 * Shortens a URL via the API
 */
export async function shortenUrl(urlToShorten: string): Promise<ApiResponse<UrlResponse>> {
  try {
    const request: UrlRequest = { url: urlToShorten };

    const response = await fetch(`${API_BASE}/api/shorten`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    return await response.json();
  } catch (error) {
    console.error('API Error:', error);
    return { error: 'Failed to connect to the server' };
  }
}

/**
 * Fetches all shortened URLs
 */
export async function fetchUrls(): Promise<ApiResponse<UrlResponse[]>> {
  try {
    const response = await fetch(`${API_BASE}/api/urls`);

    if (!response.ok) {
      return { error: `Server error: ${response.status}` };
    }

    return await response.json();
  } catch (error) {
    console.error('API Error:', error);
    return { error: 'Failed to connect to the server' };
  }
}