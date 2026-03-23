<script lang="ts">
	import CheckIcon from '$lib/components/CheckIcon.svelte';
	import AlertCircleIcon from '$lib/components/AlertCircleIcon.svelte';
	import CloseIcon from '$lib/components/CloseIcon.svelte';

	type ShopStatus = 'activa' | 'desconectada' | 'error';

	interface Shop {
		id: number;
		nombre: string;
		plataforma: string;
		mlId: string;
		status: ShopStatus;
		conectada: string;
		ventas: number;
		desconectada?: string;
		ultimoError?: string;
		errorMsg?: string;
	}

	const shops: Shop[] = [
		{
			id: 1,
			nombre: 'Mi Tienda ML',
			plataforma: 'MercadoLibre',
			mlId: 'ML-987654321',
			status: 'activa',
			conectada: '15 ene 2026',
			ventas: 47,
		},
		{
			id: 2,
			nombre: 'Tienda Prueba',
			plataforma: 'MercadoLibre',
			mlId: 'ML-123456789',
			status: 'desconectada',
			conectada: '02 ene 2026',
			desconectada: '10 ene 2026',
			ventas: 3,
		},
		{
			id: 3,
			nombre: 'Tienda Mayorista',
			plataforma: 'MercadoLibre',
			mlId: 'ML-555666777',
			status: 'error',
			conectada: '05 ene 2026',
			ultimoError: '10 feb 2026',
			errorMsg: 'Token expirado. Reconecta para seguir recibiendo ventas.',
			ventas: 21,
		},
	];

	let feedbackType = $state<'success' | 'error' | null>(null);
	let modalShop = $state<Shop | null>(null);

	function openModal(shop: Shop) { modalShop = shop; }
	function closeModal() { modalShop = null; }
	function confirmDisconnect() { closeModal(); }
</script>

<!-- Connection feedback banner -->
{#if feedbackType === 'success'}
<div class="feedback-bar feedback-bar--success">
	<div class="feedback-bar__left">
		<CheckIcon />
		<span>Tienda conectada correctamente. Ya puedes recibir ventas.</span>
	</div>
	<button onclick={() => feedbackType = null} class="feedback-close" aria-label="Cerrar">
		<CloseIcon size={14} />
	</button>
</div>
{/if}

{#if feedbackType === 'error'}
<div class="feedback-bar feedback-bar--error">
	<div class="feedback-bar__left">
		<AlertCircleIcon />
		<span>No se pudo conectar la tienda. MercadoLibre no autorizó el acceso.</span>
	</div>
	<button onclick={() => feedbackType = null} class="feedback-close" aria-label="Cerrar">
		<CloseIcon size={14} />
	</button>
</div>
{/if}

<div class="page-content">

	<div class="page-hdr">
		<h1 class="page-title">Tiendas</h1>
		<p class="page-subtitle">Conecta tus tiendas para recibir ventas automáticamente.</p>
	</div>

	<!-- Connected shops -->
	<div class="shop-list">
		{#each shops as shop}
		<div class="shop-card" class:shop-card--error={shop.status === 'error'} class:shop-card--dimmed={shop.status === 'desconectada'}>
			<div class="shop-body">

				<!-- Platform icon -->
				<div class="shop-icon" class:shop-icon--ml={shop.status === 'activa'} class:shop-icon--error={shop.status === 'error'}>
					<svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor"><circle cx="12" cy="12" r="5"/></svg>
					{#if shop.status === 'error'}
						<span class="shop-icon-dot"></span>
					{/if}
				</div>

				<!-- Info -->
				<div class="shop-info">
					<div class="shop-name-row">
						<h3 class="shop-name">{shop.nombre}</h3>
						<span class="shop-status-badge" data-status={shop.status}>
							{#if shop.status === 'activa'}Activa
							{:else if shop.status === 'desconectada'}Desconectada
							{:else}Error de conexión{/if}
						</span>
					</div>

					{#if shop.status === 'error' && shop.errorMsg}
						<div class="shop-error-msg">{shop.errorMsg}</div>
					{/if}

					<div class="shop-meta">
						<div class="shop-meta-item">
							<span class="shop-meta-label">Plataforma:</span>
							<span class="shop-meta-value">{shop.plataforma}</span>
						</div>
						<div class="shop-meta-item">
							<span class="shop-meta-label">ID:</span>
							<span class="shop-meta-value font-mono">{shop.mlId}</span>
						</div>
						<div class="shop-meta-item">
							<span class="shop-meta-label">Conectada:</span>
							<span class="shop-meta-value">{shop.conectada}</span>
						</div>
						{#if shop.desconectada}
							<div class="shop-meta-item">
								<span class="shop-meta-label">Desconectada:</span>
								<span class="shop-meta-value">{shop.desconectada}</span>
							</div>
						{/if}
						{#if shop.ultimoError}
							<div class="shop-meta-item">
								<span class="shop-meta-label">Último error:</span>
								<span class="shop-meta-value">{shop.ultimoError}</span>
							</div>
						{/if}
						<div class="shop-meta-item">
							<span class="shop-meta-label">Ventas:</span>
							<span class="shop-meta-value font-medium">{shop.ventas}</span>
						</div>
					</div>
				</div>

				<!-- Actions -->
				<div class="shop-actions">
					{#if shop.status === 'activa'}
						<button onclick={() => openModal(shop)} class="btn-secondary">Desconectar</button>
					{:else if shop.status === 'desconectada'}
						<button class="btn-primary">Reconectar</button>
					{:else}
						<button class="btn-primary">Reconectar</button>
						<button onclick={() => openModal(shop)} class="btn-secondary">Desconectar</button>
					{/if}
				</div>

			</div>
		</div>
		{/each}
	</div>

	<!-- Connect new shop -->
	<div class="connect-section">
		<h2 class="section-heading">Conectar Nueva Tienda</h2>
		<div class="connect-grid">

			<div class="connect-card">
				<div class="connect-icon connect-icon--ml">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor"><circle cx="12" cy="12" r="5"/></svg>
				</div>
				<h3 class="connect-name">MercadoLibre</h3>
				<p class="connect-desc">Se abrirá MercadoLibre para autorizar acceso</p>
				<button class="btn-primary">Conectar con MercadoLibre</button>
			</div>

			<div class="connect-card connect-card--disabled">
				<div class="connect-icon connect-icon--shopify">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"/>
					</svg>
				</div>
				<h3 class="connect-name">Shopify</h3>
				<p class="connect-desc">No disponible en esta versión</p>
				<button class="btn-secondary" disabled>Próximamente</button>
			</div>

		</div>
	</div>

</div>

<!-- Disconnect modal -->
{#if modalShop}
<div class="modal-overlay" role="presentation">
	<button class="modal-backdrop" onclick={closeModal} aria-label="Cerrar diálogo"></button>
	<div class="modal" role="dialog" aria-modal="true" aria-labelledby="modal-heading">
		<h3 class="modal-title" id="modal-heading">Desconectar tienda</h3>
		<p class="modal-body">
			¿Estás seguro de desconectar <strong>"{modalShop.nombre}"</strong>? Dejarás de recibir ventas de esta tienda.
		</p>
		<div class="modal-actions">
			<button onclick={closeModal} class="btn-secondary">Cancelar</button>
			<button onclick={confirmDisconnect} class="btn-danger">Desconectar</button>
		</div>
	</div>
</div>
{/if}

<style>
	.page-content {
		padding: 24px;
		max-width: 900px;
		margin: 0 auto;
		width: 100%;
	}

	/* .page-hdr, .page-title, .page-subtitle — global in app.css */

	/* ── Feedback banners ── */
	.feedback-bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		padding: 12px 20px;
		font-size: 14px;
		border-bottom: 1px solid var(--border-subtle);
	}

	.feedback-bar--success {
		background: color-mix(in srgb, #22c55e 8%, transparent);
		color: color-mix(in srgb, #15803d 90%, var(--text));
	}

	.feedback-bar--error {
		background: color-mix(in srgb, var(--color-red) 8%, transparent);
		color: color-mix(in srgb, var(--color-red) 85%, var(--text));
	}

	.feedback-bar__left {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.feedback-close {
		background: none;
		border: none;
		cursor: pointer;
		padding: 2px;
		color: inherit;
		opacity: 0.6;
		flex-shrink: 0;
	}

	.feedback-close:hover { opacity: 1; }

	/* ── Shop list ── */
	.shop-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
		margin-bottom: 32px;
	}

	.shop-card {
		background: var(--bg-surface);
		border: 2px solid var(--border);
		border-radius: 12px;
		overflow: hidden;
	}

	.shop-card--error { border-color: color-mix(in srgb, var(--color-red) 50%, transparent); }
	.shop-card--dimmed { opacity: 0.75; }

	.shop-body {
		display: flex;
		align-items: flex-start;
		gap: 16px;
		padding: 16px 20px;
	}

	/* Platform icon */
	.shop-icon {
		width: 40px;
		height: 40px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		position: relative;
		background: color-mix(in srgb, var(--color-ml) 15%, transparent);
		color: var(--color-ml);
	}

	.shop-icon--error {
		background: color-mix(in srgb, var(--color-red) 10%, transparent);
		color: color-mix(in srgb, var(--color-red) 60%, transparent);
	}

	.shop-icon-dot {
		position: absolute;
		top: -2px;
		right: -2px;
		width: 12px;
		height: 12px;
		background: var(--color-red);
		border-radius: 50%;
		border: 2px solid var(--bg-surface);
	}

	/* Shop info */
	.shop-info { flex: 1; min-width: 0; }

	.shop-name-row {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 8px;
	}

	.shop-name {
		margin: 0;
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
	}

	.shop-status-badge {
		display: inline-flex;
		align-items: center;
		padding: 2px 8px;
		border-radius: 999px;
		font-size: 12px;
		font-weight: 500;
	}

	.shop-status-badge[data-status="activa"] {
		background: var(--status-emitido-bg);
		color: var(--status-emitido-text);
	}

	.shop-status-badge[data-status="desconectada"] {
		background: color-mix(in srgb, var(--text-faint) 20%, transparent);
		color: var(--text-muted);
	}

	.shop-status-badge[data-status="error"] {
		background: var(--status-fallido-bg);
		color: var(--status-fallido-text);
	}

	.shop-error-msg {
		font-size: 12px;
		color: var(--status-fallido-text);
		background: color-mix(in srgb, var(--color-red) 6%, transparent);
		border-radius: 8px;
		padding: 6px 10px;
		margin-bottom: 8px;
	}

	.shop-meta {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 4px 32px;
	}

	.shop-meta-item {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
	}

	.shop-meta-label { color: var(--text-muted); }
	.shop-meta-value { color: var(--text); }

	/* Shop actions */
	.shop-actions {
		display: flex;
		flex-direction: column;
		gap: 8px;
		flex-shrink: 0;
		align-self: flex-start;
	}

	/* ── Connect new shop ── */
	.connect-section { margin-bottom: 24px; }

	.connect-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
		margin-top: 12px;
	}

	.connect-card {
		background: var(--bg-surface);
		border: 2px solid var(--border);
		border-radius: 12px;
		padding: 20px;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
	}

	.connect-card--disabled { opacity: 0.5; }

	.connect-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 12px;
	}

	.connect-icon--ml {
		background: color-mix(in srgb, var(--color-ml) 15%, transparent);
		color: var(--color-ml);
	}

	.connect-icon--shopify {
		background: color-mix(in srgb, #22c55e 10%, transparent);
		color: color-mix(in srgb, #22c55e 70%, transparent);
	}

	.connect-name {
		margin: 0 0 4px;
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
	}

	.connect-desc {
		margin: 0 0 16px;
		font-size: 12px;
		color: var(--text-muted);
	}

	/* ── Disconnect modal ── */
	.modal-overlay {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.modal-backdrop {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.3);
		border: none;
		cursor: default;
		width: 100%;
		height: 100%;
	}

	.modal {
		position: relative;
		z-index: 1;
		background: var(--bg-surface);
		border: 2px solid var(--border);
		border-radius: 12px;
		padding: 24px;
		width: 100%;
		max-width: 360px;
	}

	.modal-title {
		margin: 0 0 8px;
		font-size: 15px;
		font-weight: 600;
		color: var(--text);
	}

	.modal-body {
		margin: 0 0 20px;
		font-size: 14px;
		color: var(--text-secondary);
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 8px;
	}
</style>
