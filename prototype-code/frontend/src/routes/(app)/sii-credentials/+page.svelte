<script lang="ts">
	import CheckIcon from '$lib/components/CheckIcon.svelte';
	import AlertCircleIcon from '$lib/components/AlertCircleIcon.svelte';

	type CredentialState = 'stored' | 'empty';
	type ValidationState = 'idle' | 'success' | 'error';

	let credentialState = $state<CredentialState>('stored');
	let showUpdateForm = $state(false);
	let validationState = $state<ValidationState>('idle');

	// Update form fields
	let updateRut = $state('76.259.812-4');
	let updatePassword = $state('');

	// Empty state form fields
	let emptyRut = $state('');
	let emptyPassword = $state('');

	function validateNow() {
		validationState = 'success';
		setTimeout(() => { validationState = 'idle'; }, 3000);
	}

	function saveAndValidate() {
		// Mock: just show success and hide form
		showUpdateForm = false;
		validationState = 'success';
		setTimeout(() => { validationState = 'idle'; }, 3000);
	}
</script>

<div class="page-content">

	<div class="page-hdr">
		<h1 class="page-title">Credenciales SII</h1>
		<p class="page-subtitle">Estas credenciales se usan para emitir DTEs en tu nombre a través del portal del SII. Tu clave se almacena de forma encriptada.</p>
	</div>

	<!-- ── Case 1: Credentials stored ── -->
	{#if credentialState === 'stored'}

	<div class="card">
		<div class="card-body">
			<div class="cred-row">
				<div class="cred-icon">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"/>
					</svg>
				</div>
				<div class="cred-info">
					<div class="cred-title-row">
						<h3 class="cred-title">Credenciales actuales</h3>
						<span class="badge" data-estado="emitido">Validada</span>
					</div>
					<div class="cred-fields">
						<div class="cred-field">
							<span class="cred-label">RUT:</span>
							<span class="cred-value font-mono">76.259.812-4</span>
						</div>
						<div class="cred-field">
							<span class="cred-label">Clave SII:</span>
							<span class="cred-value text-muted">Clave almacenada</span>
						</div>
						<div class="cred-field">
							<span class="cred-label">Última validación:</span>
							<span class="cred-value">15 ene 2026</span>
						</div>
					</div>
				</div>
			</div>
		</div>

		<div class="card-footer">
			<button onclick={validateNow} class="btn-primary">Validar ahora</button>
			<button onclick={() => showUpdateForm = !showUpdateForm} class="btn-secondary">
				{showUpdateForm ? 'Cancelar' : 'Actualizar credenciales'}
			</button>
		</div>

		{#if validationState === 'success'}
		<div class="card-feedback card-feedback--success">
			<CheckIcon />
			<span>Credenciales validadas correctamente.</span>
		</div>
		{:else if validationState === 'error'}
		<div class="card-feedback card-feedback--error">
			<AlertCircleIcon />
			<span>No se pudo validar. Verifica tu RUT y clave del SII.</span>
		</div>
		{/if}
	</div>

	<!-- Update form (toggled) -->
	{#if showUpdateForm}
	<div class="card" style="margin-top: 12px;">
		<div class="card-body">
			<h3 class="card-section-title">Actualizar credenciales</h3>
			<div class="form-fields">
				<div class="form-group">
					<label class="form-label" for="update-rut">RUT</label>
					<input id="update-rut" type="text" class="form-input font-mono" bind:value={updateRut} placeholder="76.259.812-4">
				</div>
				<div class="form-group">
					<label class="form-label" for="update-pw">Nueva clave SII</label>
					<input id="update-pw" type="password" class="form-input" bind:value={updatePassword} placeholder="Nueva clave">
				</div>
			</div>
		</div>
		<div class="card-footer">
			<button onclick={saveAndValidate} class="btn-primary">Guardar y validar</button>
			<button onclick={() => showUpdateForm = false} class="btn-secondary">Cancelar</button>
		</div>
	</div>
	{/if}

	<!-- ── Case 2: No credentials (first time) ── -->
	{:else}

	<div class="card">
		<div class="card-body">
			<div class="empty-state">
				<div class="empty-icon">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"/>
					</svg>
				</div>
				<h3 class="empty-title">Para emitir DTEs necesitamos tus credenciales del SII</h3>
				<p class="empty-desc">Tu clave se almacena de forma encriptada y solo se usa para emitir documentos en tu nombre.</p>
			</div>

			<div class="form-fields" style="max-width: 380px; margin: 0 auto;">
				<div class="form-group">
					<label class="form-label" for="empty-rut">RUT</label>
					<input id="empty-rut" type="text" class="form-input font-mono" bind:value={emptyRut} placeholder="12.345.678-9">
				</div>
				<div class="form-group">
					<label class="form-label" for="empty-pw">Clave SII</label>
					<input id="empty-pw" type="password" class="form-input" bind:value={emptyPassword} placeholder="Tu clave del SII">
				</div>
			</div>
		</div>

		<div class="card-footer card-footer--center">
			<button class="btn-primary">Guardar y validar</button>
			<p class="card-footer-hint">Al guardar, intentaremos validar tus credenciales contra el portal del SII.</p>
		</div>
	</div>

	{/if}

</div>

<style>
	.page-content {
		padding: 24px;
		max-width: 640px;
		margin: 0 auto;
		width: 100%;
	}

	/* .page-hdr, .page-title, .page-subtitle, .card, .card-body, .card-footer, .card-footer--center — global in app.css */

	.card-section-title {
		margin: 0 0 16px;
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
	}

	.card-footer-hint {
		margin: 4px 0 0;
		font-size: 11px;
		color: var(--text-faint);
		text-align: center;
	}

	.card-feedback {
		border-top: 1px solid var(--border-subtle);
		padding: 10px 20px;
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 13px;
	}

	.card-feedback--success {
		color: var(--status-emitido-text);
		background: color-mix(in srgb, var(--status-emitido-bg) 40%, transparent);
	}

	.card-feedback--error {
		color: var(--status-fallido-text);
		background: color-mix(in srgb, var(--status-fallido-bg) 40%, transparent);
	}

	/* ── Stored credentials layout ── */
	.cred-row {
		display: flex;
		align-items: flex-start;
		gap: 16px;
	}

	.cred-icon {
		width: 40px;
		height: 40px;
		border-radius: 12px;
		background: var(--primary-subtle);
		color: var(--primary-text);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.cred-info { flex: 1; }

	.cred-title-row {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 12px;
	}

	.cred-title {
		margin: 0;
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
	}

	.cred-fields {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.cred-field {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
	}

	.cred-label {
		color: var(--text-muted);
		width: 140px;
		flex-shrink: 0;
	}

	.cred-value { color: var(--text); }

	/* ── Empty state ── */
	.empty-state {
		text-align: center;
		margin-bottom: 24px;
	}

	.empty-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		background: var(--color-amber-bg);
		color: var(--color-amber-text);
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0 auto 12px;
	}

	.empty-title {
		margin: 0 0 6px;
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
	}

	.empty-desc {
		margin: 0;
		font-size: 13px;
		color: var(--text-muted);
		max-width: 400px;
		margin-inline: auto;
	}
</style>
