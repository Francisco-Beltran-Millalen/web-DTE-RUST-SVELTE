<script lang="ts">
	import { page } from '$app/stores';
	import { calcNeto, calcIva, type DteEstado, type PdfStatus } from '$lib/utils/dte';
	import { formatNumber, formatCLP } from '$lib/utils/format';
	import PDFActions from '$lib/components/PDFActions.svelte';
	import StatusBadges from '$lib/components/StatusBadges.svelte';
	import AlertCircleIcon from '$lib/components/AlertCircleIcon.svelte';

	interface LineItem {
		cod: string;
		detalle: string;
		cant: number;
		unid: string;
		precioUni: number;
		descuento: number;
		indicador: string;
		total: number;
	}

	interface DteDetail {
		id: number;
		tipo: 'Boleta' | 'Factura';
		folio: string | null;
		estado: DteEstado;
		esManual?: boolean;
		pdf?: PdfStatus;
		fechaVenta: string;
		fechaEmision?: string;
		errorMsg?: string;
		// Seller
		empresa: string;
		rut: string;
		giro: string;
		direccion: string;
		siiUnidad: string;
		// Buyer
		compradorRut: string;
		compradorNombre: string;
		compradorGiro: string;
		compradorCiudad: string;
		compradorComuna: string;
		compradorDireccion: string;
		// Items
		items: LineItem[];
		// Payment
		formaPago: { forma: string; fecha: string; monto: number; glosa: string } | null;
		observaciones?: string;
	}

	// Mock data — keyed by id
	const mockDtes: Record<string, DteDetail> = {
		'10': {
			id: 10, tipo: 'Boleta', folio: 'N° 12001', estado: 'emitido', pdf: 'enviado',
			fechaVenta: '08 feb 2026', fechaEmision: '08 feb 2026, 14:32',
			empresa: 'Survalley SpA', rut: '76.259.812-4',
			giro: 'Servicios Integrales de Informática',
			direccion: 'Providencia 1881 #96545566, Providencia',
			siiUnidad: 'S.I.I. Unidad Providencia',
			compradorRut: '66.666.666-6', compradorNombre: 'Luis Ramirez',
			compradorGiro: 'Servicios', compradorCiudad: 'Santiago',
			compradorComuna: 'Santiago', compradorDireccion: 'Dirección no especificada',
			items: [{ cod: 'SKU-WC', detalle: 'Webcam HD Logitech C920', cant: 1, unid: 'UN', precioUni: 27311, descuento: 0, indicador: 'Afecto', total: 27311 }],
			formaPago: { forma: 'Mercado Pago', fecha: '08-02-2026', monto: 32500, glosa: 'Pago en línea' },
		},
		'11': {
			id: 11, tipo: 'Factura', folio: 'N° 5042', estado: 'emitido', pdf: 'pendiente',
			fechaVenta: '07 feb 2026', fechaEmision: '07 feb 2026, 10:15',
			empresa: 'Survalley SpA', rut: '76.259.812-4',
			giro: 'Servicios Integrales de Informática',
			direccion: 'Providencia 1881 #96545566, Providencia',
			siiUnidad: 'S.I.I. Unidad Providencia',
			compradorRut: '77.888.999-0', compradorNombre: 'Distribuidora Norte Ltda',
			compradorGiro: 'Distribución', compradorCiudad: 'Santiago',
			compradorComuna: 'Providencia', compradorDireccion: 'Av. Providencia 1500',
			items: [{ cod: 'SKU-SW8', detalle: 'Switch 8 puertos TP-Link', cant: 10, unid: 'UN', precioUni: 45378, descuento: 0, indicador: 'Afecto', total: 453780 }],
			formaPago: { forma: 'Transferencia', fecha: '07-02-2026', monto: 540000, glosa: 'Pago a 30 días' },
		},
		'1': {
			id: 1, tipo: 'Boleta', folio: null, estado: 'pendiente',
			fechaVenta: '11 feb 2026',
			empresa: 'Survalley SpA', rut: '76.259.812-4',
			giro: 'Servicios Integrales de Informática',
			direccion: 'Providencia 1881 #96545566, Providencia',
			siiUnidad: 'S.I.I. Unidad Providencia',
			compradorRut: '66.666.666-6', compradorNombre: 'Carlos Perez',
			compradorGiro: 'Servicios', compradorCiudad: 'Santiago',
			compradorComuna: 'Las Condes', compradorDireccion: 'Dirección no especificada',
			items: [{ cod: 'SKU-ABS', detalle: 'Auriculares Bluetooth Sony', cant: 1, unid: 'UN', precioUni: 38647, descuento: 0, indicador: 'Afecto', total: 38647 }],
			formaPago: { forma: 'Mercado Pago', fecha: '11-02-2026', monto: 45990, glosa: 'Pago en línea' },
		},
		'8': {
			id: 8, tipo: 'Boleta', folio: null, estado: 'fallido',
			fechaVenta: '09 feb 2026',
			errorMsg: 'Timeout de conexión con SII. El servicio no respondió en el tiempo esperado.',
			empresa: 'Survalley SpA', rut: '76.259.812-4',
			giro: 'Servicios Integrales de Informática',
			direccion: 'Providencia 1881 #96545566, Providencia',
			siiUnidad: 'S.I.I. Unidad Providencia',
			compradorRut: '66.666.666-6', compradorNombre: 'Pedro Gonzalez',
			compradorGiro: 'Servicios', compradorCiudad: 'Santiago',
			compradorComuna: 'Santiago', compradorDireccion: 'Dirección no especificada',
			items: [{ cod: 'SKU-UC2', detalle: 'Cable USB-C 2m', cant: 1, unid: 'UN', precioUni: 12605, descuento: 0, indicador: 'Afecto', total: 12605 }],
			formaPago: { forma: 'Mercado Pago', fecha: '09-02-2026', monto: 15000, glosa: 'Pago en línea' },
		},
		'6': {
			id: 6, tipo: 'Boleta', folio: null, estado: 'manual',
			fechaVenta: '10 feb 2026',
			observaciones: 'La venta fue procesada fuera de horario de automatización. Requiere emisión manual.',
			empresa: 'Survalley SpA', rut: '76.259.812-4',
			giro: 'Servicios Integrales de Informática',
			direccion: 'Providencia 1881 #96545566, Providencia',
			siiUnidad: 'S.I.I. Unidad Providencia',
			compradorRut: '66.666.666-6', compradorNombre: 'Maria Lopez',
			compradorGiro: 'Servicios', compradorCiudad: 'Santiago',
			compradorComuna: 'Ñuñoa', compradorDireccion: 'Dirección no especificada',
			items: [{ cod: 'SKU-MO24', detalle: 'Monitor 24" LG IPS', cant: 1, unid: 'UN', precioUni: 75622, descuento: 0, indicador: 'Afecto', total: 75622 }],
			formaPago: { forma: 'Mercado Pago', fecha: '10-02-2026', monto: 89990, glosa: 'Pago en línea' },
		},
	};

	const id = $derived($page.params.id as string);
	const dte = $derived(mockDtes[id] ?? null);

	function totalItems(items: LineItem[]): number {
		return items.reduce((s, i) => s + i.total, 0);
	}

</script>

{#if !dte}
<div class="not-found">
	<p>DTE no encontrado.</p>
	<a href="/" class="back-link">← Volver a Inicio</a>
</div>
{:else}
<div class="page-content">

	<!-- Back nav + title -->
	<div class="page-header">
		<a href="/" class="back-link">
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M15 19l-7-7 7-7"/>
			</svg>
			Volver a Inicio
		</a>
		<div class="title-row">
			<div class="title-left">
				<h1 class="page-title">{dte.tipo === 'Boleta' ? 'Boleta Electrónica' : 'Factura Electrónica'}</h1>
				{#if dte.folio}
					<span class="folio-label">{dte.folio}</span>
				{/if}
			</div>
			<div class="title-badges">
				<StatusBadges estado={dte.estado} esManual={dte.esManual} pdf={dte.pdf} />
			</div>
		</div>
	</div>

	<!-- State-aware action bar -->
	{#if dte.estado === 'pendiente'}
	<div class="action-bar">
		<span class="action-bar__text">Este DTE está listo para ser emitido al SII.</span>
		<div class="action-bar__btns">
			<button class="btn-primary">Emitir a SII</button>
			<button class="btn-secondary">Marcar manual</button>
		</div>
	</div>

	{:else if dte.estado === 'manual'}
	<div class="action-bar action-bar--warning">
		<div class="action-bar__left">
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon-warning">
				<path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4.5c-.77-.833-2.694-.833-3.464 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
			</svg>
			<span class="action-bar__text">{dte.observaciones ?? 'Datos del comprador incompletos. Emita manualmente desde el portal SII.'}</span>
		</div>
		<button class="btn-orange">Marcar como emitido</button>
	</div>

	{:else if dte.estado === 'fallido'}
	<div class="action-bar action-bar--error">
		<div class="action-bar__left">
			<AlertCircleIcon class="error-icon" />
			<span class="action-bar__text">{dte.errorMsg ?? 'Error de conexión con SII.'}</span>
		</div>
		<button class="btn-primary">Reintentar emisión</button>
	</div>

	{:else if dte.estado === 'emitido'}
	<div class="action-bar">
		<span class="action-bar__text">
			Emitido el <strong>{dte.fechaEmision}</strong>
		</span>
		<div class="action-bar__btns">
			<PDFActions />
		</div>
	</div>
	{/if}

	<!-- SII Document preview -->
	<div class="doc-card">

		<!-- Document header -->
		<div class="doc-header">
			<div class="doc-seller">
				<h2 class="doc-seller__name">{dte.empresa}</h2>
				<p class="doc-seller__giro">{dte.giro}</p>
				<p class="doc-seller__dir">{dte.direccion}</p>
			</div>
			<div class="doc-type-box">
				<p class="doc-type-box__rut">R.U.T: {dte.rut}</p>
				<p class="doc-type-box__tipo">{dte.tipo === 'Boleta' ? 'BOLETA ELECTRÓNICA' : 'FACTURA ELECTRÓNICA'}</p>
				{#if dte.folio}
					<p class="doc-type-box__folio">{dte.folio}</p>
				{/if}
			</div>
		</div>
		<div class="doc-sii-meta">
			<p>{dte.siiUnidad}</p>
			<p>Fecha de emisión: {dte.fechaVenta}</p>
		</div>

		<!-- Buyer info -->
		<div class="doc-section">
			<table class="buyer-table">
				<tbody>
					<tr>
						<td class="buyer-table__label">RUT:</td>
						<td class="buyer-table__value buyer-table__value--mono">{dte.compradorRut}</td>
						<td class="buyer-table__label">RAZÓN SOCIAL:</td>
						<td class="buyer-table__value">{dte.compradorNombre}</td>
					</tr>
					<tr>
						<td class="buyer-table__label">GIRO:</td>
						<td class="buyer-table__value">{dte.compradorGiro}</td>
						<td class="buyer-table__label">CIUDAD:</td>
						<td class="buyer-table__value">{dte.compradorCiudad}</td>
					</tr>
					<tr>
						<td class="buyer-table__label">COMUNA:</td>
						<td class="buyer-table__value">{dte.compradorComuna}</td>
						<td class="buyer-table__label">DIRECCIÓN:</td>
						<td class="buyer-table__value">{dte.compradorDireccion}</td>
					</tr>
				</tbody>
			</table>
		</div>

		<!-- Line items -->
		<div class="doc-section doc-section--bordered">
			<table class="items-table">
				<thead>
					<tr>
						<th>Cod.</th>
						<th>Detalle</th>
						<th class="text-center">Cant.</th>
						<th class="text-center">Unid.</th>
						<th class="text-right">Precio Uni.</th>
						<th class="text-center">Desc.</th>
						<th>Ind.</th>
						<th class="text-right">Total</th>
					</tr>
				</thead>
				<tbody>
					{#each dte.items as item}
					<tr>
						<td class="mono">{item.cod}</td>
						<td>{item.detalle}</td>
						<td class="text-center">{item.cant}</td>
						<td class="text-center">{item.unid}</td>
						<td class="text-right font-mono">{formatCLP(item.precioUni)}</td>
						<td class="text-center">{item.descuento}%</td>
						<td>{item.indicador}</td>
						<td class="text-right font-mono font-medium">{formatCLP(item.total)}</td>
					</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<!-- Totals -->
		<div class="doc-section doc-totals">
			<div class="totals-box">
				<div class="totals-row">
					<span class="totals-row__label">Neto $</span>
					<span class="totals-row__value font-mono">{formatNumber(calcNeto(totalItems(dte.items)))}</span>
				</div>
				<div class="totals-row">
					<span class="totals-row__label">IVA (19%) $</span>
					<span class="totals-row__value font-mono">{formatNumber(calcIva(totalItems(dte.items)))}</span>
				</div>
				<div class="totals-row totals-row--total">
					<span class="totals-row__label">Total $</span>
					<span class="totals-row__value font-mono font-bold">{formatNumber(totalItems(dte.items))}</span>
				</div>
			</div>
		</div>

		<!-- References + Observations -->
		<div class="doc-section doc-refs">
			<div class="refs-col">
				<h3 class="section-heading">Referencias</h3>
				<div class="inner-panel">
					<table class="refs-table">
						<thead>
							<tr>
								<th>Tipo Doc.</th>
								<th>Folio</th>
								<th>Fecha</th>
								<th>Motivo</th>
							</tr>
						</thead>
						<tbody>
							<tr><td colspan="4" class="empty-cell">Documento no tiene referencias</td></tr>
						</tbody>
					</table>
				</div>
			</div>
			<div class="obs-col">
				<h3 class="section-heading">Observación</h3>
				<div class="inner-panel">
					{#if dte.observaciones}
						<p class="obs-text">{dte.observaciones}</p>
					{:else}
						<p class="obs-empty">Sin observaciones</p>
					{/if}
				</div>
			</div>
		</div>

		<!-- Payment + Timbre -->
		<div class="doc-section doc-refs">
			<div class="refs-col">
				<h3 class="section-heading">Forma de Pago</h3>
				<div class="inner-panel">
					{#if dte.formaPago}
					<table class="refs-table">
						<thead>
							<tr>
								<th>Forma</th>
								<th>Fecha</th>
								<th class="text-right">Monto</th>
								<th>Glosa</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td>{dte.formaPago.forma}</td>
								<td>{dte.formaPago.fecha}</td>
								<td class="text-right font-mono">{formatCLP(dte.formaPago.monto)}</td>
								<td>{dte.formaPago.glosa}</td>
							</tr>
						</tbody>
					</table>
					{:else}
						<p class="obs-empty">Sin información de pago</p>
					{/if}
				</div>
			</div>
			<div class="obs-col">
				<h3 class="section-heading">Timbre Electrónico</h3>
				<div class="inner-panel timbre-panel">
					<div class="timbre-qr">
						<svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
							<path stroke-linecap="round" stroke-linejoin="round" d="M3.75 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 013.75 9.375v-4.5zM3.75 14.625c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5a1.125 1.125 0 01-1.125-1.125v-4.5zM13.5 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 0113.5 9.375v-4.5z"/>
							<path stroke-linecap="round" stroke-linejoin="round" d="M13.5 14.625v2.25m0 2.25h2.25m2.25 0h-2.25m0 0v-2.25m0 0h2.25"/>
						</svg>
					</div>
					<p class="timbre-hint">Verifique documento: www.sii.cl</p>
				</div>
			</div>
		</div>

	</div>

	<!-- Generated documents -->
	<div class="docs-card">
		<div class="docs-card__header">
			<h3 class="docs-card__title">Documentos Generados</h3>
		</div>
		<table class="docs-table">
			<thead>
				<tr>
					<th>Tipo</th>
					<th>Fecha Generación</th>
					<th>Acción</th>
				</tr>
			</thead>
			<tbody>
				{#if dte.estado === 'emitido'}
				<tr>
					<td>PDF</td>
					<td>{dte.fechaEmision}</td>
					<td>
						<button class="btn-link">Descargar</button>
					</td>
				</tr>
				<tr>
					<td>XML</td>
					<td>{dte.fechaEmision}</td>
					<td>
						<button class="btn-link">Descargar</button>
					</td>
				</tr>
				{:else}
				<tr>
					<td colspan="3" class="empty-cell">Sin documentos generados</td>
				</tr>
				{/if}
			</tbody>
		</table>
	</div>

</div>
{/if}

<style>
	.page-content {
		padding: 24px;
		max-width: 960px;
		margin: 0 auto;
		width: 100%;
	}

	.not-found {
		padding: 48px 24px;
		text-align: center;
		color: var(--text-muted);
	}

	/* ── Header ── */
	.page-header {
		margin-bottom: 20px;
	}

	/* .back-link — global in app.css */

	.title-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.title-left {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	/* .page-title — global in app.css */

	.folio-label {
		font-size: 13px;
		font-family: font-monospace;
		color: var(--text-muted);
	}

	.title-badges {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	/* .badge, .badge--pdf, [data-estado], [data-pdf] — global in app.css */

	/* ── Action bar ── */
	.action-bar {
		background: var(--bg-surface);
		border: 2px solid var(--border);
		border-radius: 12px;
		padding: 12px 20px;
		margin-bottom: 20px;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 16px;
	}

	.action-bar--warning { border-color: color-mix(in srgb, var(--color-orange) 40%, transparent); }
	.action-bar--error   { border-color: color-mix(in srgb, var(--color-red) 40%, transparent); }

	.action-bar__left {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.action-bar__text {
		font-size: 14px;
		color: var(--text-secondary);
	}

	.action-bar__btns {
		display: flex;
		align-items: center;
		gap: 8px;
		flex-shrink: 0;
	}

	.icon-warning { color: var(--color-orange); flex-shrink: 0; }
	/* .icon-error replaced by global .error-icon in app.css */

	/* ── Buttons ── */
	/* Base styles + colors defined globally in app.css */
	.btn-orange { flex-shrink: 0; }

	/* ── Document card ── */
	.doc-card {
		background: var(--bg-surface);
		border: 2px solid var(--border);
		border-radius: 12px;
		overflow: hidden;
		margin-bottom: 20px;
	}

	.doc-header {
		display: flex;
		gap: 24px;
		padding: 24px 24px 20px;
	}

	.doc-seller { flex: 1; }

	.doc-seller__name {
		margin: 0;
		font-size: 15px;
		font-weight: 700;
		color: var(--text);
	}

	.doc-seller__giro,
	.doc-seller__dir {
		margin: 3px 0 0;
		font-size: 13px;
		color: var(--text-muted);
	}

	.doc-type-box {
		width: 240px;
		flex-shrink: 0;
		border: 2px solid var(--sii-red);
		border-radius: 8px;
		padding: 16px;
		text-align: center;
	}

	.doc-type-box__rut,
	.doc-type-box__tipo,
	.doc-type-box__folio {
		margin: 0 0 4px;
		font-size: 13px;
		font-weight: 700;
		color: var(--sii-red-text);
	}

	.doc-sii-meta {
		text-align: right;
		padding: 0 24px 16px;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.doc-sii-meta p {
		margin: 0;
		font-size: 11px;
		color: var(--text-muted);
	}

	.doc-section {
		border-top: 1px solid var(--border-subtle);
		padding: 0;
	}

	.doc-section--bordered {
		border-top: 1px solid var(--border);
	}

	/* Buyer table */
	.buyer-table {
		width: 100%;
		font-size: 13px;
		border-collapse: collapse;
	}

	.buyer-table td { padding: 8px 20px; }

	.buyer-table__label {
		font-weight: 600;
		color: var(--text-muted);
		width: 112px;
	}

	.buyer-table__value { color: var(--text); }
	.buyer-table__value--mono { font-family: font-monospace; font-size: 12px; }

	/* Items table */
	.items-table {
		width: 100%;
		font-size: 13px;
		border-collapse: collapse;
	}

	.items-table th,
	.docs-table th {
		padding: 10px 20px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--text-muted);
		background: color-mix(in srgb, var(--bg-page) 60%, transparent);
		text-align: left;
	}

	.items-table td {
		padding: 10px 20px;
		color: var(--text);
		border-top: 1px solid var(--border-subtle);
	}

	/* .text-center, .text-right, .font-bold, .font-mono, .font-medium — global in app.css */

	/* Totals */
	.doc-totals {
		padding: 16px 20px;
	}

	.totals-box {
		width: 220px;
		margin-left: auto;
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.totals-row {
		display: flex;
		justify-content: space-between;
		font-size: 13px;
	}

	.totals-row__label { color: var(--text-muted); }
	.totals-row__value { color: var(--text); }

	.totals-row--total {
		border-top: 1px solid var(--border);
		padding-top: 6px;
		font-weight: 600;
	}

	/* Refs + obs */
	.doc-refs {
		display: flex;
		gap: 20px;
		padding: 16px 20px;
	}

	.refs-col { flex: 1; }

	.obs-col {
		width: 240px;
		flex-shrink: 0;
	}

	/* .section-heading — global in app.css */

	.inner-panel {
		background: color-mix(in srgb, var(--bg-page) 40%, transparent);
		border-radius: 8px;
		padding: 12px;
	}

	.refs-table {
		width: 100%;
		font-size: 12px;
		border-collapse: collapse;
	}

	.refs-table th {
		padding-bottom: 6px;
		font-weight: 600;
		color: var(--text-muted);
		text-align: left;
	}

	.refs-table td { padding: 4px 0; color: var(--text); }

	.empty-cell,
	.obs-empty {
		font-size: 12px;
		color: var(--text-faint);
		text-align: center;
		padding: 8px 0;
	}

	.empty-cell { padding: 8px 0 !important; }
	.obs-empty  { margin: 0; }

	.obs-text {
		margin: 0;
		font-size: 12px;
		color: var(--text-secondary);
		line-height: 1.6;
	}

	/* Timbre */
	.timbre-panel {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
	}

	.timbre-qr {
		width: 96px;
		height: 96px;
		background: color-mix(in srgb, var(--border) 40%, transparent);
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-faint);
	}

	.timbre-hint {
		margin: 0;
		font-size: 10px;
		color: var(--text-faint);
	}

	/* ── Generated docs card ── */
	.docs-card {
		background: var(--bg-surface);
		border: 2px solid var(--border);
		border-radius: 12px;
		overflow: hidden;
		margin-bottom: 24px;
	}

	.docs-card__header {
		padding: 12px 20px;
		border-bottom: 1px solid var(--border-subtle);
	}

	.docs-card__title {
		margin: 0;
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
	}

	.docs-table {
		width: 100%;
		font-size: 13px;
		border-collapse: collapse;
	}

	.docs-table th { border-bottom: 1px solid var(--border-subtle); }

	.docs-table td {
		padding: 12px 20px;
		color: var(--text);
		border-top: 1px solid var(--border-subtle);
	}
</style>
