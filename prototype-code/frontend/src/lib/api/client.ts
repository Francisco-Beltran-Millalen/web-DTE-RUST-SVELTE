/**
 * Fetch wrapper — handles JWT Authorization header and 401 retry.
 *
 * Usage:
 *   import { apiFetch } from '$lib/api/client';
 *   const data = await apiFetch('/health');
 *
 * The access token is read from localStorage on every request.
 * On 401, a token refresh is attempted once, then the request is retried.
 * If the refresh fails, the auth store is cleared (user logged out).
 */

const API_BASE = import.meta.env.PUBLIC_API_BASE_URL ?? 'http://localhost:3000';

const ACCESS_TOKEN_KEY = 'dte_access_token';

export function getAccessToken(): string | null {
	return localStorage.getItem(ACCESS_TOKEN_KEY);
}

export function setAccessToken(token: string): void {
	localStorage.setItem(ACCESS_TOKEN_KEY, token);
}

export function clearAccessToken(): void {
	localStorage.removeItem(ACCESS_TOKEN_KEY);
}

async function refreshAccessToken(): Promise<string | null> {
	// TODO: implement token refresh (Stage 4-2, use case 2: sign-in)
	// Will POST /auth/refresh with the HttpOnly cookie containing the refresh token.
	return null;
}

export async function apiFetch(path: string, init: RequestInit = {}): Promise<Response> {
	const token = getAccessToken();
	const headers = new Headers(init.headers);

	if (token) {
		headers.set('Authorization', `Bearer ${token}`);
	}
	headers.set('Content-Type', 'application/json');

	const response = await fetch(`${API_BASE}${path}`, { ...init, headers });

	if (response.status === 401) {
		const newToken = await refreshAccessToken();
		if (newToken) {
			setAccessToken(newToken);
			headers.set('Authorization', `Bearer ${newToken}`);
			return fetch(`${API_BASE}${path}`, { ...init, headers });
		}
		// Refresh failed — caller handles the 401
	}

	return response;
}
