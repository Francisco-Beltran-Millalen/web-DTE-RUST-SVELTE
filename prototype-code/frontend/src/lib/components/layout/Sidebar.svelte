<script lang="ts">
	import { page } from '$app/stores';
	import ThemeToggle from '$lib/components/ThemeToggle.svelte';
	import DocIcon from '$lib/components/DocIcon.svelte';

	function isActive(path: string): boolean {
		return $page.url.pathname === path;
	}
</script>

<aside class="sidebar">

	<!-- Logo -->
	<a href="/" class="nav-logo" title="DTE Auto-Upload">
		<DocIcon />
	</a>

	<!-- Inicio -->
	<a href="/" class="nav-item" class:active={isActive('/')} title="Inicio">
		<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
			<path d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-4 0a1 1 0 01-1-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 01-1 1h-2z"/>
		</svg>
	</a>

	<div class="nav-separator"></div>

	<!-- Tiendas -->
	<a href="/tiendas" class="nav-item" class:active={isActive('/tiendas')} title="Tiendas">
		<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
			<path d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 100 4 2 2 0 000-4z"/>
		</svg>
	</a>

	<!-- Credenciales SII -->
	<a href="/sii-credentials" class="nav-item" class:active={isActive('/sii-credentials')} title="Credenciales SII">
		<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
			<path d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"/>
		</svg>
	</a>

	<div class="nav-spacer"></div>

	<!-- Theme toggle -->
	<ThemeToggle />

	<!-- Notificaciones -->
	<a href="/notifications" class="nav-item nav-relative" class:active={isActive('/notifications')} title="Notificaciones">
		<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
			<path d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/>
		</svg>
		<span class="notification-dot"></span>
	</a>

	<!-- Perfil -->
	<a href="/profile" class="nav-item" class:active={isActive('/profile')} title="Perfil">
		<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
			<path d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
		</svg>
	</a>

	<!-- Salir -->
	<a href="/login" class="nav-item nav-logout" title="Salir">
		<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
			<path d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/>
		</svg>
	</a>

</aside>

<style>
	.sidebar {
		width: 64px;
		background: var(--bg-surface);
		border-right: 2px solid var(--border);
		position: fixed;
		top: 0;
		left: 0;
		height: 100vh;
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 20px 0;
		gap: 8px;
		z-index: 20;
	}

	.nav-logo {
		width: 40px;
		height: 40px;
		border-radius: 12px;
		background: var(--primary);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 16px;
		color: white;
		text-decoration: none;
		flex-shrink: 0;
	}

	.nav-item {
		width: 40px;
		height: 40px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-muted);
		text-decoration: none;
		transition: background 0.15s, color 0.15s;
		flex-shrink: 0;
	}

	.nav-item:hover {
		background: var(--bg-page);
		color: var(--text-secondary);
	}

	.nav-item.active {
		background: var(--primary-subtle);
		color: var(--primary-text);
	}

	.nav-separator {
		width: 32px;
		height: 1px;
		background: var(--border);
		margin: 4px 0;
		flex-shrink: 0;
	}

	.nav-spacer {
		flex: 1;
	}

	.nav-relative {
		position: relative;
	}

	.notification-dot {
		position: absolute;
		top: 4px;
		right: 4px;
		width: 10px;
		height: 10px;
		background: var(--color-red);
		border-radius: 50%;
		border: 2px solid var(--bg-surface);
	}

	.nav-logout {
		color: var(--text-faint);
	}

	.nav-logout:hover {
		color: var(--text-muted);
		background: var(--bg-page);
	}
</style>
