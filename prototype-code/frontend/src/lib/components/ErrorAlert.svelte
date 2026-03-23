<script lang="ts">
	import AlertCircleIcon from './AlertCircleIcon.svelte';
	interface Props { message: string; }
	let { message }: Props = $props();
</script>

{#if message}
	<div class="form-error">
		<AlertCircleIcon class="error-icon" />
		<span>{message}</span>
	</div>
{/if}
