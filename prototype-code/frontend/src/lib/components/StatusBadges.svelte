<!-- Shared estado + PDF badge pair — used in home page table, detail panel, and DTE detail page -->
<script lang="ts">
	import { estadoKey, estadoLabel, type DteEstado, type PdfStatus } from '$lib/utils/dte';

	interface Props {
		estado: DteEstado;
		esManual?: boolean;
		pdf?: PdfStatus;
	}
	let { estado, esManual, pdf }: Props = $props();
	const dteBase = $derived({ estado, esManual });
</script>

<span class="badge" data-estado={estadoKey(dteBase)}>{estadoLabel(dteBase)}</span>
{#if pdf}
	<span class="badge badge--pdf" data-pdf={pdf}>PDF {pdf}</span>
{/if}
