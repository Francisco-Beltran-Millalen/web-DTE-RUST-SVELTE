<script lang="ts">
	import Sidebar from '$lib/components/layout/Sidebar.svelte';

	let { children } = $props();
</script>

<div class="app-shell">
	<Sidebar />
	<main class="app-main">
		{@render children()}
	</main>
</div>

<style>
	.app-shell {
		display: flex;
		min-height: 100vh;
		background: var(--bg-page);
	}

	.app-main {
		flex: 1;
		margin-left: 64px;
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}
</style>
