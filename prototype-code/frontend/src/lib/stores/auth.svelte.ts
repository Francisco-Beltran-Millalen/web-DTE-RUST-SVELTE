/**
 * Auth store — Svelte 5 runes-based reactive auth state.
 *
 * Usage:
 *   import { authStore } from '$lib/stores/auth.svelte';
 *   if (authStore.isAuthenticated) { ... }
 *   authStore.setUser(user, token);
 *   authStore.clear();
 */

export interface AuthUser {
	id: number;
	name: string;
	email: string;
}

interface AuthState {
	user: AuthUser | null;
	isAuthenticated: boolean;
}

function createAuthStore() {
	let state = $state<AuthState>({
		user: null,
		isAuthenticated: false
	});

	return {
		get user() {
			return state.user;
		},
		get isAuthenticated() {
			return state.isAuthenticated;
		},
		setUser(user: AuthUser, accessToken: string) {
			// TODO: store token via client.ts setAccessToken (Stage 4-2)
			state.user = user;
			state.isAuthenticated = true;
		},
		clear() {
			// TODO: clear token via client.ts clearAccessToken (Stage 4-2)
			state.user = null;
			state.isAuthenticated = false;
		}
	};
}

export const authStore = createAuthStore();
