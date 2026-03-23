<script lang="ts">
	import CheckIcon from '$lib/components/CheckIcon.svelte';

	type NotifCategory = 'emision_dte' | 'conexion_tienda' | 'email';

	interface Notification {
		id: number;
		category: NotifCategory;
		fecha: string;
		mensaje: string;
		unread: boolean;
		actionLabel?: string;
		actionHref?: string;
	}

	const mockNotifications: Notification[] = [
		{
			id: 1,
			category: 'emision_dte',
			fecha: '11 feb 2026, 14:30',
			mensaje: '<strong>3 DTEs emitidos exitosamente.</strong> Boleta N° 1580, 1581, 1582 fueron subidas al SII.',
			unread: true,
			actionLabel: 'Ver',
			actionHref: '/',
		},
		{
			id: 2,
			category: 'conexion_tienda',
			fecha: '10 feb 2026, 09:15',
			mensaje: '<strong>Error de conexión con "Mi Tienda ML".</strong> Token de acceso expirado. Reconecta tu tienda para seguir recibiendo ventas.',
			unread: true,
			actionLabel: 'Reconectar',
			actionHref: '/tiendas',
		},
		{
			id: 3,
			category: 'emision_dte',
			fecha: '10 feb 2026, 08:45',
			mensaje: '<strong>Error al emitir Boleta.</strong> Venta de Pedro Gonzalez ($15.000) no pudo ser subida al SII. Motivo: SII no disponible.',
			unread: true,
			actionLabel: 'Reintentar',
			actionHref: '/dtes/8',
		},
		{
			id: 4,
			category: 'email',
			fecha: '09 feb 2026, 16:20',
			mensaje: 'PDF de Boleta N° 1578 enviado a carlos.perez@email.com',
			unread: false,
		},
		{
			id: 5,
			category: 'email',
			fecha: '09 feb 2026, 16:18',
			mensaje: '<strong>No se pudo enviar PDF</strong> de Factura N° 5040 a distribucion@empresa.cl. Motivo: dirección de email inválida.',
			unread: false,
			actionLabel: 'Ver DTE',
			actionHref: '/dtes/11',
		},
		{
			id: 6,
			category: 'emision_dte',
			fecha: '08 feb 2026, 11:00',
			mensaje: 'Boleta N° 1578 emitida exitosamente al SII.',
			unread: false,
		},
		{
			id: 7,
			category: 'email',
			fecha: '08 feb 2026, 11:01',
			mensaje: 'Copia de Boleta N° 1578 enviada a juan.vendedor@email.com',
			unread: false,
		},
	];

	type FilterKey = 'todas' | 'no_leidas' | 'emision_dte' | 'email' | 'conexion_tienda';

	let activeFilter = $state<FilterKey>('todas');
	let notifications = $state(mockNotifications.map(n => ({ ...n })));

	const filtered = $derived(
		activeFilter === 'todas' ? notifications
		: activeFilter === 'no_leidas' ? notifications.filter(n => n.unread)
		: notifications.filter(n => n.category === activeFilter)
	);

	const unreadCount = $derived(notifications.filter(n => n.unread).length);

	function markAllRead() {
		notifications = notifications.map(n => ({ ...n, unread: false }));
	}

	const categoryLabel: Record<NotifCategory, string> = {
		emision_dte: 'Emisión DTE',
		conexion_tienda: 'Conexión tienda',
		email: 'Email',
	};

	const filters: { key: FilterKey; label: string }[] = [
		{ key: 'todas', label: 'Todas' },
		{ key: 'no_leidas', label: 'No leídas' },
		{ key: 'emision_dte', label: 'Emisión DTE' },
		{ key: 'email', label: 'Emails' },
		{ key: 'conexion_tienda', label: 'Conexión tiendas' },
	];
</script>

<div class="page-content">

	<div class="page-hdr">
		<div class="page-hdr__left">
			<h1 class="page-title">Notificaciones</h1>
			<p class="page-subtitle">{unreadCount > 0 ? `${unreadCount} no leídas` : 'Todo al día'}</p>
		</div>
		{#if unreadCount > 0}
			<button onclick={markAllRead} class="btn-secondary">Marcar todas como leídas</button>
		{/if}
	</div>

	<!-- Filter pills -->
	<div class="filter-pills">
		{#each filters as f}
			<button
				class="filter-pill"
				class:filter-pill--active={activeFilter === f.key}
				onclick={() => activeFilter = f.key}
			>
				{f.label}
			</button>
		{/each}
	</div>

	<!-- Notification list -->
	<div class="notif-list">
		{#each filtered as notif (notif.id)}
		<div class="notif-row" class:notif-row--unread={notif.unread}>

			<div class="notif-left">
				<span class="notif-dot" class:notif-dot--visible={notif.unread}></span>
				<div class="notif-icon" data-category={notif.category}>
					{#if notif.category === 'emision_dte'}
						<CheckIcon />
					{:else if notif.category === 'conexion_tienda'}
						<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"/>
						</svg>
					{:else}
						<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
						</svg>
					{/if}
				</div>
			</div>

			<div class="notif-body">
				<div class="notif-meta">
					<span class="notif-category-pill" data-category={notif.category}>{categoryLabel[notif.category]}</span>
					<span class="notif-date">{notif.fecha}</span>
				</div>
				<!-- eslint-disable-next-line svelte/no-at-html-tags -->
				<p class="notif-msg" class:notif-msg--read={!notif.unread}>{@html notif.mensaje}</p>
			</div>

			<div class="notif-action">
				{#if notif.actionLabel && notif.actionHref}
					<a href={notif.actionHref} class="notif-action-link">{notif.actionLabel}</a>
				{/if}
			</div>

		</div>
		{/each}

		{#if filtered.length === 0}
			<div class="notif-empty">Sin notificaciones en esta categoría.</div>
		{/if}
	</div>

	<div class="list-footer">
		Mostrando {filtered.length} de {notifications.length} notificaciones
	</div>

</div>

<style>
	.page-content {
		padding: 24px;
		max-width: 900px;
		margin: 0 auto;
		width: 100%;
	}

	.page-hdr {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 16px;
		margin-bottom: 20px;
	}

	/* .page-title, .page-subtitle — global in app.css */

	/* ── Filter pills ── */
	.filter-pills {
		display: flex;
		align-items: center;
		gap: 8px;
		flex-wrap: wrap;
		margin-bottom: 20px;
	}

	.filter-pill {
		font-size: 12px;
		font-weight: 500;
		padding: 6px 14px;
		border-radius: 999px;
		border: none;
		cursor: pointer;
		font-family: inherit;
		background: color-mix(in srgb, var(--bg-page) 60%, transparent);
		color: var(--text-secondary);
		transition: background 0.15s, color 0.15s;
	}

	.filter-pill:hover {
		background: var(--bg-page);
		color: var(--text);
	}

	.filter-pill--active {
		background: var(--text-ink-dark);
		color: var(--bg-surface);
	}

	.filter-pill--active:hover {
		background: var(--text);
	}

	/* ── Notification list ── */
	.notif-list {
		background: var(--bg-surface);
		border: 2px solid var(--border);
		border-radius: 12px;
		overflow: hidden;
		margin-bottom: 16px;
	}

	.notif-row {
		display: flex;
		align-items: flex-start;
		gap: 16px;
		padding: 16px 20px;
		border-bottom: 1px solid var(--border-subtle);
		transition: background 0.15s;
		cursor: default;
	}

	.notif-row:last-child { border-bottom: none; }

	.notif-row:hover { background: color-mix(in srgb, var(--bg-page) 40%, transparent); }

	.notif-row--unread {
		background: color-mix(in srgb, var(--primary-subtle) 60%, transparent);
	}

	.notif-row--unread:hover {
		background: color-mix(in srgb, var(--primary-subtle) 80%, transparent);
	}

	/* Left column: dot + icon */
	.notif-left {
		display: flex;
		align-items: center;
		gap: 10px;
		flex-shrink: 0;
		padding-top: 2px;
	}

	.notif-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.notif-dot--visible { background: var(--primary); }

	.notif-icon {
		width: 32px;
		height: 32px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.notif-icon[data-category="emision_dte"] {
		background: color-mix(in srgb, #22c55e 12%, transparent);
		color: var(--status-emitido-text);
	}

	.notif-icon[data-category="conexion_tienda"] {
		background: color-mix(in srgb, var(--color-orange) 12%, transparent);
		color: var(--color-orange);
	}

	.notif-icon[data-category="email"] {
		background: color-mix(in srgb, #3b82f6 10%, transparent);
		color: #3b82f6;
	}

	/* Body */
	.notif-body { flex: 1; min-width: 0; }

	.notif-meta {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 4px;
	}

	.notif-category-pill {
		display: inline-flex;
		align-items: center;
		padding: 1px 7px;
		border-radius: 999px;
		font-size: 11px;
		font-weight: 500;
	}

	.notif-category-pill[data-category="emision_dte"] {
		background: var(--status-emitido-bg);
		color: var(--status-emitido-text);
	}

	.notif-category-pill[data-category="conexion_tienda"] {
		background: color-mix(in srgb, var(--color-orange) 12%, transparent);
		color: var(--color-orange);
	}

	.notif-category-pill[data-category="email"] {
		background: color-mix(in srgb, #3b82f6 10%, transparent);
		color: #3b82f6;
	}

	.notif-date {
		font-size: 12px;
		color: var(--text-faint);
	}

	.notif-msg {
		margin: 0;
		font-size: 14px;
		color: var(--text);
	}

	.notif-msg--read { color: var(--text-secondary); }

	/* Action link */
	.notif-action {
		flex-shrink: 0;
		padding-top: 2px;
		min-width: 48px;
		text-align: right;
	}

	.notif-action-link {
		font-size: 12px;
		font-weight: 500;
		color: var(--primary-text);
		text-decoration: none;
		white-space: nowrap;
	}

	.notif-action-link:hover { color: var(--primary-hover); }

	.notif-empty {
		padding: 32px;
		text-align: center;
		font-size: 14px;
		color: var(--text-faint);
	}

	.list-footer {
		text-align: center;
		font-size: 12px;
		color: var(--text-faint);
		padding: 8px 0 4px;
	}
</style>
