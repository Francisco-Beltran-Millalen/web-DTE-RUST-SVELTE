<script lang="ts">
	import DocIcon from '$lib/components/DocIcon.svelte';
	let { children } = $props();
</script>

<div class="auth-shell">

	<!-- Logo (shared across all auth views) -->
	<div class="auth-logo">
		<div class="auth-logo-icon">
			<DocIcon />
		</div>
		<h1>DTE Auto-Upload</h1>
		<p>Emite tus DTEs de forma automática y gratuita</p>
	</div>

	{@render children()}

</div>

<style>
	.auth-shell {
		min-height: 100vh;
		background: var(--bg-page);
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem 1rem;
	}

	.auth-logo {
		display: flex;
		flex-direction: column;
		align-items: center;
		margin-bottom: 2rem;
		text-align: center;
	}

	.auth-logo-icon {
		width: 40px;
		height: 40px;
		border-radius: 12px;
		background: var(--primary);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 12px;
		color: white;
	}

	.auth-logo h1 {
		margin: 0;
		font-size: 20px;
		font-weight: 700;
		color: var(--text);
	}

	.auth-logo p {
		margin: 4px 0 0;
		font-size: 14px;
		color: var(--text-muted);
	}
</style>
