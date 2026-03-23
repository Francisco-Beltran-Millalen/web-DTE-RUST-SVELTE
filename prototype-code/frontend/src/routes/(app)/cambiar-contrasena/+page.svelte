<script lang="ts">
	import CheckIcon from '$lib/components/CheckIcon.svelte';
	import AlertCircleIcon from '$lib/components/AlertCircleIcon.svelte';

	// Option 1: I know my password
	let localOpen = $state(true);
	let currentPw = $state('');
	let newPw = $state('');
	let confirmPw = $state('');
	let localResult = $state<'idle' | 'success' | 'error'>('idle');

	function submitLocal() {
		if (!newPw || newPw !== confirmPw) {
			localResult = 'error';
			return;
		}
		localResult = 'success';
	}

	// Option 2: I forgot my password
	let emailOpen = $state(false);
	let emailSent = $state(false);

	function sendResetEmail() {
		emailSent = true;
		setTimeout(() => { emailSent = false; }, 5000);
	}
</script>

<div class="page-content">

	<a href="/profile" class="back-link">
		<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
			<path d="M15 19l-7-7 7-7"/>
		</svg>
		Volver a Perfil
	</a>

	<div class="page-hdr">
		<h1 class="page-title">Cambiar Contraseña</h1>
		<p class="page-subtitle">Elige cómo quieres actualizar tu contraseña.</p>
	</div>

	<!-- Option 1: I know my password -->
	<div class="accordion">
		<button class="accordion-header" onclick={() => localOpen = !localOpen}>
			<div class="accordion-icon accordion-icon--primary">
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<path d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
				</svg>
			</div>
			<div class="accordion-label">
				<span class="accordion-title">Recuerdo mi contraseña</span>
				<p class="accordion-subtitle">Ingresa tu contraseña actual y elige una nueva</p>
			</div>
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="accordion-chevron" class:rotated={!localOpen}>
				<path d="M19 9l-7 7-7-7"/>
			</svg>
		</button>

		{#if localOpen}
		<div class="accordion-body">
			<div class="form-fields">
				<div class="form-group">
					<label class="form-label" for="local-current">Contraseña actual</label>
					<input id="local-current" type="password" class="form-input" bind:value={currentPw} placeholder="Tu contraseña actual">
				</div>
				<div class="form-group">
					<label class="form-label" for="local-new">Nueva contraseña</label>
					<input id="local-new" type="password" class="form-input" bind:value={newPw} placeholder="Mínimo 8 caracteres">
				</div>
				<div class="form-group">
					<label class="form-label" for="local-confirm">Confirmar contraseña</label>
					<input id="local-confirm" type="password" class="form-input" bind:value={confirmPw} placeholder="Repite la nueva contraseña">
				</div>
			</div>

			<div class="accordion-footer">
				<button onclick={submitLocal} class="btn-primary">Cambiar contraseña</button>
			</div>

			{#if localResult === 'success'}
			<div class="accordion-feedback accordion-feedback--success">
				<CheckIcon size={14} />
				<span>Contraseña actualizada correctamente.</span>
			</div>
			{:else if localResult === 'error'}
			<div class="accordion-feedback accordion-feedback--error">
				<AlertCircleIcon size={14} />
				<span>Las contraseñas no coinciden.</span>
			</div>
			{/if}
		</div>
		{/if}
	</div>

	<!-- Option 2: I forgot my password -->
	<div class="accordion" style="margin-top: 12px;">
		<button class="accordion-header" onclick={() => emailOpen = !emailOpen}>
			<div class="accordion-icon accordion-icon--blue">
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<path d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
				</svg>
			</div>
			<div class="accordion-label">
				<span class="accordion-title">No recuerdo mi contraseña</span>
				<p class="accordion-subtitle">Te enviaremos un enlace de restablecimiento por email</p>
			</div>
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="accordion-chevron" class:rotated={!emailOpen}>
				<path d="M19 9l-7 7-7-7"/>
			</svg>
		</button>

		{#if emailOpen}
		<div class="accordion-body">
			<p class="email-hint">
				Enviaremos un enlace para restablecer tu contraseña a <strong>juan.vendedor@email.com</strong>
			</p>

			<div class="accordion-footer">
				<button onclick={sendResetEmail} class="btn-primary">Enviar enlace de restablecimiento</button>
			</div>

			{#if emailSent}
			<div class="accordion-feedback accordion-feedback--success">
				<CheckIcon size={14} />
				<span>Enlace enviado. Revisa tu bandeja de entrada.</span>
			</div>
			{/if}
		</div>
		{/if}
	</div>

</div>

<style>
	.page-content {
		padding: 24px;
		max-width: 640px;
		margin: 0 auto;
		width: 100%;
	}

	/* .back-link, .page-hdr, .page-title, .page-subtitle — global in app.css */

	/* ── Accordion ── */
	.accordion {
		background: var(--bg-surface);
		border: 2px solid var(--border);
		border-radius: 12px;
		overflow: hidden;
	}

	.accordion-header {
		width: 100%;
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px 20px;
		background: none;
		border: none;
		cursor: pointer;
		text-align: left;
		font-family: inherit;
		transition: background 0.15s;
	}

	.accordion-header:hover { background: color-mix(in srgb, var(--bg-page) 30%, transparent); }

	.accordion-icon {
		width: 32px;
		height: 32px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.accordion-icon--primary {
		background: var(--primary-subtle);
		color: var(--primary-text);
	}

	.accordion-icon--blue {
		background: color-mix(in srgb, #3b82f6 10%, transparent);
		color: #3b82f6;
	}

	.accordion-label { flex: 1; }

	.accordion-title {
		display: block;
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
	}

	.accordion-subtitle {
		margin: 2px 0 0;
		font-size: 12px;
		color: var(--text-muted);
	}

	.accordion-chevron {
		color: var(--text-faint);
		flex-shrink: 0;
		transition: transform 0.2s;
	}

	.accordion-chevron.rotated { transform: rotate(-90deg); }

	.accordion-body {
		border-top: 1px solid var(--border-subtle);
		padding: 20px;
	}

	.accordion-footer {
		border-top: 1px solid var(--border-subtle);
		padding-top: 12px;
		margin-top: 4px;
	}

	.accordion-feedback {
		border-top: 1px solid var(--border-subtle);
		padding: 10px 0 0;
		margin-top: 12px;
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
	}

	.accordion-feedback--success { color: var(--status-emitido-text); }
	.accordion-feedback--error   { color: var(--status-fallido-text); }

	.email-hint {
		margin: 0 0 16px;
		font-size: 14px;
		color: var(--text-secondary);
	}
</style>
