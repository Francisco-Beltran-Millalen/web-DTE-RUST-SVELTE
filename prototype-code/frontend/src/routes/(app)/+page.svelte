<script lang="ts">
	import { goto } from '$app/navigation';
	import { calcNeto, calcIva, type DteEstado, type PdfStatus } from '$lib/utils/dte';
	import { formatCLP } from '$lib/utils/format';
	import PDFActions from '$lib/components/PDFActions.svelte';
	import StatusBadges from '$lib/components/StatusBadges.svelte';
	import CloseIcon from '$lib/components/CloseIcon.svelte';

	type DteTab = 'pendiente' | 'manual' | 'fallido' | 'emitido' | 'todos';

	interface Dte {
		id: number;
		tipo: 'Boleta' | 'Factura';
		folio: string | null;
		tienda: string;
		comprador: string;
		rut?: string;
		monto: number;
		fecha: string;
		fechaEmision?: string;
		estado: DteEstado;
		esManual?: boolean;
		pdf?: PdfStatus;
		email?: string;
		mlId: string;
		item: string;
		observaciones?: string;
	}

	// Tab counts — hardcoded to match Phase 3 mock design
	const tabCounts: Record<DteTab, number> = {
		pendiente: 5,
		manual: 2,
		fallido: 2,
		emitido: 43,
		todos: 52,
	};

	const mockDtes: Dte[] = [
		// Pendientes
		{ id: 1, tipo: 'Boleta', folio: null, tienda: 'Mi Tienda ML', comprador: 'Carlos Perez', monto: 45990, fecha: '11 feb 2026', estado: 'pendiente', mlId: 'MLB-2098467312', item: '1x Auriculares Bluetooth Sony', email: 'carlos@email.com' },
		{ id: 2, tipo: 'Factura', folio: null, tienda: 'Mi Tienda ML', comprador: 'Importadora Sur SpA', rut: '76.123.456-7', monto: 320000, fecha: '10 feb 2026', estado: 'pendiente', mlId: 'MLB-2098467200', item: '1x Rack servidor 2U Dell', email: 'compras@importadorasur.cl' },
		{ id: 3, tipo: 'Boleta', folio: null, tienda: 'Mi Tienda ML', comprador: 'Ana Torres', monto: 67500, fecha: '09 feb 2026', estado: 'pendiente', mlId: 'MLB-2098467100', item: '1x Teclado mecánico Keychron K2', email: 'ana.torres@gmail.com' },
		{ id: 4, tipo: 'Boleta', folio: null, tienda: 'Mi Tienda ML', comprador: 'Sofia Herrera', monto: 29990, fecha: '09 feb 2026', estado: 'pendiente', mlId: 'MLB-2098467050', item: '1x Mouse inalámbrico Logitech', email: 'sofia.h@gmail.com' },
		{ id: 5, tipo: 'Factura', folio: null, tienda: 'Mi Tienda ML', comprador: 'Comercial Lagos Ltda', rut: '76.444.555-8', monto: 185000, fecha: '08 feb 2026', estado: 'pendiente', mlId: 'MLB-2098466900', item: '5x Hub USB-C 7 puertos', email: 'compras@clagos.cl' },
		// Emision manual
		{ id: 6, tipo: 'Boleta', folio: null, tienda: 'Mi Tienda ML', comprador: 'Maria Lopez', monto: 89990, fecha: '10 feb 2026', estado: 'manual', mlId: 'MLB-2098467250', item: '1x Monitor 24" LG IPS', email: 'maria.lopez@gmail.com', observaciones: 'La venta fue procesada fuera de horario de automatización. Requiere emisión manual.' },
		{ id: 7, tipo: 'Factura', folio: null, tienda: 'Mi Tienda ML', comprador: 'Tech Solutions Ltda', rut: '76.999.111-2', monto: 156000, fecha: '08 feb 2026', estado: 'manual', mlId: 'MLB-2098466800', item: '2x SSD NVMe Samsung 1TB', email: 'admin@techsolutions.cl', observaciones: 'RUT no encontrado en base SII. Verificar datos antes de emitir.' },
		// Fallidos
		{ id: 8, tipo: 'Boleta', folio: null, tienda: 'Mi Tienda ML', comprador: 'Pedro Gonzalez', monto: 15000, fecha: '09 feb 2026', estado: 'fallido', mlId: 'MLB-2098467080', item: '1x Cable USB-C 2m', email: 'pedro.g@gmail.com', observaciones: 'FALLIDO: Timeout de conexión con SII. El servicio no respondió en el tiempo esperado.' },
		{ id: 9, tipo: 'Boleta', folio: null, tienda: 'Mi Tienda ML', comprador: 'Diego Navarro', monto: 52300, fecha: '07 feb 2026', estado: 'fallido', mlId: 'MLB-2098466700', item: '1x Funda laptop 15"', email: 'diego.navarro@outlook.com', observaciones: 'FALLIDO: Código de error SII 42 — CAF agotado. Renovar folios antes de reintentar.' },
		// Emitidos
		{ id: 10, tipo: 'Boleta', folio: 'N° 12001', tienda: 'Mi Tienda ML', comprador: 'Luis Ramirez', monto: 32500, fecha: '08 feb 2026', fechaEmision: '08 feb 2026, 14:32', estado: 'emitido', pdf: 'enviado', mlId: 'MLB-2098466980', item: '1x Webcam HD Logitech C920', email: 'luis.r@gmail.com' },
		{ id: 11, tipo: 'Factura', folio: 'N° 5042', tienda: 'Mi Tienda ML', comprador: 'Distribuidora Norte Ltda', rut: '77.888.999-0', monto: 540000, fecha: '07 feb 2026', fechaEmision: '07 feb 2026, 10:15', estado: 'emitido', pdf: 'pendiente', mlId: 'MLB-2098466750', item: '10x Switch 8 puertos TP-Link', email: 'bodega@distnorte.cl' },
		{ id: 12, tipo: 'Boleta', folio: 'N° 12000', tienda: 'Mi Tienda ML', comprador: 'Camila Soto', monto: 89990, fecha: '06 feb 2026', fechaEmision: '06 feb 2026, 16:45', estado: 'emitido', esManual: true, pdf: 'enviado', mlId: 'MLB-2098466600', item: '1x Monitor 24" LG IPS', email: 'camila.s@gmail.com' },
		{ id: 13, tipo: 'Boleta', folio: 'N° 11999', tienda: 'Mi Tienda ML', comprador: 'Jorge Munoz', monto: 25000, fecha: '05 feb 2026', fechaEmision: '05 feb 2026, 11:20', estado: 'emitido', pdf: 'enviado', mlId: 'MLB-2098466500', item: '1x Almohadilla de carga inalámbrica', email: 'jorge.m@gmail.com' },
		{ id: 14, tipo: 'Factura', folio: 'N° 5041', tienda: 'Mi Tienda ML', comprador: 'Servicios Andinos SpA', rut: '76.555.444-3', monto: 1200000, fecha: '03 feb 2026', fechaEmision: '03 feb 2026, 09:50', estado: 'emitido', pdf: 'pendiente', mlId: 'MLB-2098466400', item: '20x Router WiFi 6 TP-Link', email: 'it@serviciosandinos.cl' },
	];

	// --- State ---
	let activeTab = $state<DteTab>('pendiente');
	let selectedIds = $state<Set<number>>(new Set());
	let selectedDte = $state<Dte | null>(null);
	let masterChecked = $state(false);

	// --- Derived ---
	const filteredDtes = $derived(
		activeTab === 'todos' ? mockDtes : mockDtes.filter(d => d.estado === activeTab)
	);
	const selectedCount = $derived(selectedIds.size);

	// --- Handlers ---
	function switchTab(tab: DteTab) {
		activeTab = tab;
		selectedIds = new Set();
		masterChecked = false;
		selectedDte = null;
	}

	function toggleRow(id: number, checked: boolean) {
		const next = new Set(selectedIds);
		if (checked) next.add(id); else next.delete(id);
		selectedIds = next;
		masterChecked = filteredDtes.length > 0 && filteredDtes.every(d => next.has(d.id));
	}

	function toggleAll(checked: boolean) {
		masterChecked = checked;
		selectedIds = checked ? new Set(filteredDtes.map(d => d.id)) : new Set();
	}

	function clearSelection() {
		selectedIds = new Set();
		masterChecked = false;
	}

	function selectDte(dte: Dte) {
		selectedDte = dte;
	}

	function closeDetail() {
		selectedDte = null;
	}

</script>

<div class="page-content">

	<!-- TABLE CARD -->
	<div class="dte-card">

		<!-- Tab row -->
		<div class="tabs-row">
			<div class="tab-list">
				{#each ([
					{ key: 'pendiente', label: 'Pendientes', alert: true },
					{ key: 'manual',    label: 'Emisión manual', alert: true },
					{ key: 'fallido',   label: 'Fallidos', alert: true },
					{ key: 'emitido',   label: 'Emitidos', alert: false },
					{ key: 'todos',     label: 'Todos', alert: false },
				] as const) as tab}
					<button
						onclick={() => switchTab(tab.key)}
						class="tab-btn"
						class:active={activeTab === tab.key}
					>
						{tab.label}
						<span class={tab.alert ? 'tab-count-alert' : 'tab-count'}>
							{tabCounts[tab.key]}
						</span>
					</button>
				{/each}
			</div>
			<div class="tab-stats">
				<span class="stat-savings">~4.5h ahorradas</span>
				<span class="stat-revenue">$2.847.000 en ventas</span>
			</div>
		</div>

		<!-- Filters + bulk actions -->
		<div class="filters-row">
			<div class="filter-group">

				<!-- Tienda filter -->
				<div class="filter-select-wrap">
					<svg class="filter-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 100 4 2 2 0 000-4z"/>
					</svg>
					<select class="filter-select">
						<option>Tienda</option>
						<option>Mi Tienda ML</option>
					</select>
					<svg class="filter-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
						<path d="M6 9l6 6 6-6"/>
					</svg>
				</div>

				<!-- Tipo DTE filter -->
				<div class="filter-select-wrap">
					<svg class="filter-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
					</svg>
					<select class="filter-select">
						<option>Tipo DTE</option>
						<option>Boleta</option>
						<option>Factura</option>
					</select>
					<svg class="filter-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
						<path d="M6 9l6 6 6-6"/>
					</svg>
				</div>

				<!-- Ordenar filter -->
				<div class="filter-select-wrap">
					<svg class="filter-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12"/>
					</svg>
					<select class="filter-select">
						<option>Más reciente</option>
						<option>Más antigua</option>
						<option>Mayor monto</option>
						<option>Menor monto</option>
					</select>
					<svg class="filter-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
						<path d="M6 9l6 6 6-6"/>
					</svg>
				</div>

				<div class="filter-divider"></div>

				<!-- Date range -->
				<div class="date-range">
					<input type="date" value="2026-01-01" class="date-input">
					<span class="date-sep">—</span>
					<input type="date" value="2026-02-13" class="date-input">
				</div>
			</div>

			<div class="filter-spacer"></div>

			<!-- Bulk actions -->
			{#if selectedCount > 0}
				<div class="bulk-actions">
					<span class="bulk-count">
						{selectedCount === 1 ? '1 seleccionado' : `${selectedCount} seleccionados`}
					</span>
					{#if activeTab === 'pendiente'}
						<button class="btn-pill btn-pill--primary">Emitir a SII</button>
						<button class="btn-pill btn-pill--secondary">Marcar manual</button>
					{:else if activeTab === 'emitido'}
						<button class="btn-pill btn-pill--dark">
							<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<path d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
							</svg>
							Descargar PDFs
						</button>
					{:else if activeTab === 'manual'}
						<button class="btn-pill btn-pill--orange">Marcar como emitido</button>
					{:else if activeTab === 'fallido'}
						<button class="btn-pill btn-pill--primary">Reintentar emisión</button>
					{/if}
					<button onclick={clearSelection} class="bulk-clear" title="Deseleccionar">
						<CloseIcon size={14} />
					</button>
				</div>
			{/if}
		</div>

		<!-- Table -->
		<div class="table-wrapper">
			<table class="data-table">
				<thead>
					<tr>
						<th class="col-check">
							<input type="checkbox" checked={masterChecked}
								onchange={(e) => toggleAll((e.target as HTMLInputElement).checked)}
								class="table-checkbox" title="Seleccionar todos">
						</th>
						<th>Tipo</th>
						<th>Folio</th>
						<th class="col-tienda" title="Tienda">
							<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon-inline icon-faint">
								<path d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 100 4 2 2 0 000-4z"/>
							</svg>
						</th>
						<th>Comprador</th>
						<th class="col-amount">Monto</th>
						<th>Fecha</th>
						<th>Estado</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredDtes as dte (dte.id)}
						<tr
							onclick={() => selectDte(dte)}
							class:row-selected={selectedDte?.id === dte.id}
						>
							<td class="col-check" onclick={(e) => e.stopPropagation()}>
								<input type="checkbox"
									checked={selectedIds.has(dte.id)}
									onchange={(e) => toggleRow(dte.id, (e.target as HTMLInputElement).checked)}
									class="table-checkbox">
							</td>
							<td>{dte.tipo}</td>
							<td>
								{#if dte.folio}
									<span class="font-mono">{dte.folio}</span>
								{:else}
									<span class="text-faint">—</span>
								{/if}
							</td>
							<td class="col-tienda" title={dte.tienda}>
								<!-- MercadoLibre store indicator -->
								<svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="icon-inline icon-ml">
									<circle cx="12" cy="12" r="5"/>
								</svg>
							</td>
							<td>
								{dte.comprador}
								{#if dte.rut}<span class="text-faint font-small"> ({dte.rut})</span>{/if}
							</td>
							<td class="col-amount"><span class="font-medium">{formatCLP(dte.monto)}</span></td>
							<td class="text-muted">{dte.fecha}</td>
							<td>
								<StatusBadges estado={dte.estado} esManual={dte.esManual} pdf={dte.pdf} />
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<!-- Footer -->
		<div class="table-footer">
			Mostrando {filteredDtes.length} de {tabCounts[activeTab]} documentos
		</div>

	</div>

	<!-- DETAIL PANEL -->
	{#if selectedDte}
		<div class="detail-panel">

			<!-- Header -->
			<div class="panel-header">
				<h2 class="panel-title">Detalle DTE</h2>
				<button onclick={closeDetail} aria-label="Cerrar" class="btn-close">
					<CloseIcon />
				</button>
			</div>

			<!-- Content -->
			<div class="panel-content">

				<!-- Type + status -->
				<div class="panel-type-row">
					<div>
						<span class="panel-doc-type">
							{selectedDte.tipo === 'Boleta' ? 'Boleta Electrónica' : 'Factura Electrónica'}
						</span>
						{#if selectedDte.folio}
							<span class="panel-folio">{selectedDte.folio}</span>
						{/if}
					</div>
					<div class="panel-badges">
						<StatusBadges estado={selectedDte.estado} esManual={selectedDte.esManual} pdf={selectedDte.pdf} />
					</div>
				</div>

				<!-- Amounts -->
				<div class="amounts-panel">
					<div class="amount-row">
						<span class="amount-label">Neto</span>
						<span class="amount-value">{formatCLP(calcNeto(selectedDte.monto))}</span>
					</div>
					<div class="amount-row">
						<span class="amount-label">IVA (19%)</span>
						<span class="amount-value">{formatCLP(calcIva(selectedDte.monto))}</span>
					</div>
					<div class="amount-row amount-row--total">
						<span class="amount-label-total">Total</span>
						<span class="amount-value-total">{formatCLP(selectedDte.monto)}</span>
					</div>
				</div>

				<!-- Buyer -->
				<div class="panel-section">
					<h3 class="section-heading">Comprador</h3>
					<div class="detail-rows">
						<div class="detail-row">
							<span class="detail-label">Nombre</span>
							<span class="detail-value">{selectedDte.comprador}</span>
						</div>
						{#if selectedDte.rut}
							<div class="detail-row">
								<span class="detail-label">RUT</span>
								<span class="detail-value">{selectedDte.rut}</span>
							</div>
						{/if}
						{#if selectedDte.email}
							<div class="detail-row">
								<span class="detail-label">Email</span>
								<span class="detail-value">{selectedDte.email}</span>
							</div>
						{/if}
					</div>
				</div>

				<!-- Sale / Emission info -->
				<div class="panel-section">
					<h3 class="section-heading">
						{selectedDte.estado === 'emitido' ? 'Emisión' : 'Venta'}
					</h3>
					<div class="detail-rows">
						<div class="detail-row">
							<span class="detail-label">Tienda</span>
							<span class="detail-value">{selectedDte.tienda}</span>
						</div>
						<div class="detail-row">
							<span class="detail-label">Fecha venta</span>
							<span class="detail-value">{selectedDte.fecha}</span>
						</div>
						<div class="detail-row">
							<span class="detail-label">ID MercadoLibre</span>
							<span class="detail-value font-mono font-small">{selectedDte.mlId}</span>
						</div>
						{#if selectedDte.fechaEmision}
							<div class="detail-row">
								<span class="detail-label">Fecha emisión</span>
								<span class="detail-value">{selectedDte.fechaEmision}</span>
							</div>
						{/if}
					</div>
				</div>

				<!-- Observations -->
				{#if selectedDte.observaciones}
					<div class="panel-section">
						<h3 class="section-heading">Observaciones</h3>
						<div class="observations-box">{selectedDte.observaciones}</div>
					</div>
				{/if}

				<!-- Items -->
				<div class="panel-section">
					<h3 class="section-heading">Items</h3>
					<div class="items-box">
						<div class="detail-row">
							<span class="detail-value">{selectedDte.item}</span>
							<span class="detail-value font-medium">{formatCLP(selectedDte.monto)}</span>
						</div>
					</div>
				</div>

			</div>

			<!-- Panel actions -->
			<div class="panel-footer">
				{#if selectedDte.estado === 'pendiente'}
					<button class="btn-primary">Emitir a SII</button>
					<div class="btn-row">
						<button class="btn-secondary" onclick={() => goto(`/dtes/${selectedDte!.id}`)}>Ver formato SII</button>
						<button class="btn-secondary">Marcar manual</button>
					</div>
				{:else if selectedDte.estado === 'manual'}
					<button class="btn-orange">Marcar como emitido</button>
					<button class="btn-secondary" onclick={() => goto(`/dtes/${selectedDte!.id}`)}>Ver formato SII</button>
				{:else if selectedDte.estado === 'fallido'}
					<button class="btn-primary">Reintentar emisión</button>
					<button class="btn-secondary" onclick={() => goto(`/dtes/${selectedDte!.id}`)}>Ver formato SII</button>
				{:else if selectedDte.estado === 'emitido'}
					<div class="btn-row">
						<PDFActions />
					</div>
					<button class="btn-secondary" onclick={() => goto(`/dtes/${selectedDte!.id}`)}>Ver formato SII completo</button>
				{/if}
			</div>

		</div>
	{/if}

</div>

<style>
	/* ── Page Layout ── */
	.page-content {
		flex: 1;
		display: flex;
		padding: 24px;
		gap: 0;
		min-height: 0;
	}

	/* ── DTE Card ── */
	.dte-card {
		flex: 1;
		background: var(--bg-surface);
		border: 2px solid var(--border);
		border-radius: 12px;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		min-height: 0;
	}

	/* ── Tabs ── */
	.tabs-row {
		display: flex;
		align-items: flex-end;
		justify-content: space-between;
		padding: 16px 20px 0;
	}

	.tab-list {
		display: flex;
		align-items: center;
		gap: 4px;
	}

	.tab-btn {
		padding: 8px 14px;
		border-radius: 12px 12px 0 0;
		font-size: 14px;
		font-weight: 500;
		color: var(--text-muted);
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		cursor: pointer;
		transition: color 0.15s, background 0.15s;
		font-family: inherit;
	}

	.tab-btn:hover {
		color: var(--text-secondary);
		background: var(--bg-tab-hover);
	}

	.tab-btn.active {
		font-weight: 600;
		border-bottom-color: var(--primary);
		color: var(--primary-text);
		background: var(--bg-tab-active);
	}

	.tab-count-alert {
		font-size: 11px;
		font-weight: 700;
		color: color-mix(in srgb, var(--color-red) 70%, transparent);
		margin-left: 2px;
	}

	.tab-count {
		font-size: 11px;
		color: var(--text-faint);
		margin-left: 2px;
	}

	.tab-stats {
		display: flex;
		align-items: center;
		gap: 12px;
		padding-bottom: 10px;
		padding-right: 4px;
		font-size: 12px;
	}

	.stat-savings { color: var(--stat-savings); }
	.stat-revenue { color: var(--stat-revenue); }

	/* ── Filters ── */
	.filters-row {
		display: flex;
		align-items: center;
		height: 44px;
		padding: 0 20px;
		border-bottom: 1px solid var(--border-subtle);
	}

	.filter-group {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.filter-select-wrap {
		position: relative;
		display: inline-flex;
		align-items: center;
	}

	.filter-icon {
		position: absolute;
		left: 10px;
		width: 14px;
		height: 14px;
		color: var(--text-faint);
		pointer-events: none;
	}

	.filter-chevron {
		position: absolute;
		right: 7px;
		width: 10px;
		height: 10px;
		color: var(--text-muted);
		pointer-events: none;
	}

	.filter-select {
		font-size: 12px;
		color: var(--text-secondary);
		background: var(--bg-input);
		border: none;
		border-radius: 999px;
		padding: 6px 22px 6px 28px;
		cursor: pointer;
		appearance: none;
		font-family: inherit;
	}

	.filter-select:hover { background: var(--bg-page); }
	.filter-select:focus { outline: none; box-shadow: 0 0 0 2px var(--primary-ring); }

	.filter-divider {
		width: 1px;
		height: 20px;
		background: var(--border-subtle);
	}

	.date-range {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.date-sep {
		font-size: 12px;
		color: var(--text-faint);
	}

	.date-input {
		background: var(--bg-input);
		color: var(--text-secondary);
		border: none;
		border-radius: 999px;
		padding: 6px 10px;
		font-size: 12px;
		font-family: inherit;
	}

	.date-input:focus { outline: none; box-shadow: 0 0 0 2px var(--primary-ring); }

	.filter-spacer { flex: 1; }

	/* ── Bulk Actions ── */
	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.bulk-count {
		font-size: 12px;
		color: var(--text-muted);
		font-weight: 500;
	}

	/* btn-pill: base + variants defined globally in app.css */

	.bulk-clear {
		background: transparent;
		border: none;
		cursor: pointer;
		color: var(--text-faint);
		padding: 4px;
		display: flex;
		align-items: center;
	}
	.bulk-clear:hover { color: var(--text-muted); }

	/* ── Table ── */
	.table-wrapper {
		flex: 1;
		overflow: auto;
	}

	.data-table {
		width: 100%;
		font-size: 14px;
		border-collapse: collapse;
	}

	.data-table thead {
		position: sticky;
		top: 0;
		background: var(--bg-hover);
		z-index: 10;
		border-bottom: 1px solid var(--border);
	}

	.data-table thead th {
		padding: 12px 20px;
		font-weight: 600;
		color: var(--text-muted);
		text-align: left;
		white-space: nowrap;
	}

	.data-table tbody tr {
		cursor: pointer;
		transition: background 0.1s;
		border-bottom: 1px solid var(--border-subtle);
	}

	.data-table tbody tr:hover { background: var(--bg-hover); }
	.data-table tbody tr.row-selected { background: color-mix(in srgb, var(--primary-subtle) 50%, transparent); }

	.data-table tbody td {
		padding: 12px 20px;
		color: var(--text);
	}

	.col-check { width: 40px; }
	.col-tienda { width: 40px; text-align: center; }
	.col-amount { text-align: right; }

	.table-checkbox {
		cursor: pointer;
		accent-color: var(--primary);
	}

	/* ── Table Footer ── */
	.table-footer {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 12px 20px;
		border-top: 1px solid var(--border-subtle);
		font-size: 12px;
		color: var(--text-faint);
	}

	/* .badge, .badge--pdf, [data-estado], [data-pdf] — global in app.css */

	/* ── Detail Panel ── */
	.detail-panel {
		width: 400px;
		background: var(--bg-surface);
		border: 2px solid var(--border);
		border-radius: 12px;
		margin-left: 16px;
		flex-shrink: 0;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		border-bottom: 1px solid var(--border-subtle);
	}

	.panel-title {
		margin: 0;
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
	}

	.btn-close {
		width: 28px;
		height: 28px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		cursor: pointer;
		color: var(--text-muted);
		transition: background 0.15s, color 0.15s;
	}

	.btn-close:hover {
		background: var(--bg-page);
		color: var(--text-secondary);
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 20px;
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.panel-type-row {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
	}

	.panel-doc-type {
		font-size: 18px;
		font-weight: 700;
		color: var(--text);
	}

	.panel-folio {
		font-size: 12px;
		color: var(--text-muted);
		margin-left: 6px;
	}

	.panel-badges {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 4px;
	}

	/* ── Amounts ── */
	.amounts-panel {
		background: var(--bg-panel);
		border-radius: 8px;
		padding: 16px;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.amount-row {
		display: flex;
		justify-content: space-between;
		font-size: 14px;
	}

	.amount-label { color: var(--text-muted); }
	.amount-value { color: var(--text); font-weight: 500; }

	.amount-row--total {
		border-top: 1px solid var(--border);
		padding-top: 8px;
		font-size: 16px;
	}

	.amount-label-total { color: var(--text); font-weight: 600; }
	.amount-value-total { color: var(--text); font-weight: 700; }

	/* ── Panel Sections ── */
	.panel-section { display: flex; flex-direction: column; gap: 8px; }

	/* .section-heading — global in app.css */

	.detail-rows { display: flex; flex-direction: column; gap: 6px; }

	.detail-row {
		display: flex;
		justify-content: space-between;
		font-size: 14px;
	}

	.detail-label { color: var(--text-muted); }
	.detail-value { color: var(--text); }

	.observations-box {
		background: var(--color-amber-bg);
		border-radius: 8px;
		padding: 12px;
		font-size: 14px;
		color: var(--color-amber-text);
	}

	.items-box {
		background: var(--bg-panel);
		border-radius: 8px;
		padding: 12px;
		display: flex;
		flex-direction: column;
		gap: 8px;
		font-size: 14px;
	}

	/* ── Panel Footer / Action Buttons ── */
	.panel-footer {
		border-top: 1px solid var(--border-subtle);
		padding: 16px 20px;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	/* btn-primary/secondary/orange: colors defined globally in app.css */
	/* Panel footer buttons are full-width */
	.btn-primary, .btn-secondary, .btn-orange {
		width: 100%;
		padding: 10px 16px;
	}

	.btn-row {
		display: flex;
		gap: 8px;
	}

	/* PDFActions renders btn-dark + btn-secondary as child component — use :global() to reach them */
	.btn-row :global(.btn-dark),
	.btn-row :global(.btn-secondary) {
		flex: 1;
	}

	/* Typography helpers — global in app.css */
</style>
