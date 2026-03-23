<script lang="ts">
	import ErrorAlert from '$lib/components/ErrorAlert.svelte';
	import CheckIcon from '$lib/components/CheckIcon.svelte';

	// Personal info
	let nombre = $state('Juan');
	let telefono = $state('+56 9 1234 5678');
	let sobreMi = $state('Vendo productos de belleza y cuidado personal en MercadoLibre.');
	let instagram = $state('@juanvendedor');
	let facebook = $state('');
	let linkedin = $state('');
	let tiktok = $state('');

	let profileSaved = $state(false);

	function saveProfile() {
		profileSaved = true;
		setTimeout(() => { profileSaved = false; }, 3000);
	}

	// Update email
	let newEmail = $state('');
	let confirmEmail = $state('');
	let emailPassword = $state('');
	let emailError = $state('');
	let emailSaved = $state(false);

	function updateEmail() {
		if (!newEmail || newEmail !== confirmEmail) {
			emailError = 'Los emails no coinciden.';
			return;
		}
		emailError = '';
		emailSaved = true;
		setTimeout(() => { emailSaved = false; }, 3000);
	}

	// Delete account
	let deletePassword = $state('');
	let deleteChecked = $state(false);
	let deleteError = $state('');
</script>

<div class="page-content">

	<div class="page-hdr">
		<h1 class="page-title">Mi Perfil</h1>
	</div>

	<!-- Profile header card -->
	<div class="card profile-header-card">
		<div class="avatar-row">
			<div class="avatar-wrap">
				<div class="avatar">
					<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
						<path d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
					</svg>
				</div>
				<button class="avatar-edit-btn" title="Cambiar foto">
					<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"/>
						<path d="M15 13a3 3 0 11-6 0 3 3 0 016 0z"/>
					</svg>
				</button>
			</div>
			<div>
				<h2 class="profile-greeting">Hola, {nombre || 'usuario'}!</h2>
				<p class="profile-member">Miembro desde 10 ene 2026</p>
			</div>
		</div>
	</div>

	<!-- Personal info card -->
	<div class="card">
		<div class="card-body">
			<h3 class="section-heading">Información Personal</h3>

			<div class="form-fields">
				<!-- Email (read-only) -->
				<div class="form-group">
					<label class="form-label" for="email-readonly">Email</label>
					<div class="input-readonly" id="email-readonly">
						juan.vendedor@email.com
						<span class="text-faint font-small"> (no editable)</span>
					</div>
				</div>

				<div class="form-group">
					<label class="form-label" for="profile-nombre">Nombre</label>
					<input id="profile-nombre" type="text" class="form-input" bind:value={nombre} placeholder="¿Cómo quieres que te saludemos?">
				</div>

				<div class="form-group">
					<label class="form-label" for="profile-tel">Teléfono <span class="label-optional">(opcional)</span></label>
					<input id="profile-tel" type="tel" class="form-input" bind:value={telefono} placeholder="+56 9 XXXX XXXX">
				</div>

				<div class="form-group">
					<label class="form-label" for="profile-sobre">Sobre mí <span class="label-optional">(opcional)</span></label>
					<textarea id="profile-sobre" class="form-input form-textarea" bind:value={sobreMi} rows="3" placeholder="Cuéntanos sobre ti o tu negocio"></textarea>
				</div>
			</div>

			<!-- Social media sub-section -->
			<div class="sub-section">
				<h3 class="section-heading">Redes Sociales <span class="label-optional normal-case">(opcional)</span></h3>
				<div class="form-fields">
					<div class="form-group">
						<label class="form-label" for="profile-ig">Instagram</label>
						<input id="profile-ig" type="text" class="form-input" bind:value={instagram} placeholder="@usuario">
					</div>
					<div class="form-group">
						<label class="form-label" for="profile-fb">Facebook</label>
						<input id="profile-fb" type="text" class="form-input" bind:value={facebook} placeholder="URL o nombre">
					</div>
					<div class="form-group">
						<label class="form-label" for="profile-li">LinkedIn</label>
						<input id="profile-li" type="text" class="form-input" bind:value={linkedin} placeholder="URL o nombre">
					</div>
					<div class="form-group">
						<label class="form-label" for="profile-tt">TikTok</label>
						<input id="profile-tt" type="text" class="form-input" bind:value={tiktok} placeholder="@usuario">
					</div>
				</div>
			</div>
		</div>

		<div class="card-footer">
			<button onclick={saveProfile} class="btn-primary">Guardar cambios</button>
			{#if profileSaved}
				<div class="inline-feedback inline-feedback--success">
					<CheckIcon size={14} />
					<span>Cambios guardados correctamente.</span>
				</div>
			{/if}
		</div>
	</div>

	<!-- Update email card -->
	<div class="card">
		<div class="card-body">
			<h3 class="section-heading">Actualizar Email</h3>
			<p class="card-desc">Tu email actual: <strong class="text-ink">juan.vendedor@email.com</strong></p>

			<ErrorAlert message={emailError} />

			<div class="form-fields">
				<div class="form-group">
					<label class="form-label" for="new-email">Nuevo email</label>
					<input id="new-email" type="email" class="form-input" bind:value={newEmail} placeholder="nuevo@email.com">
				</div>
				<div class="form-group">
					<label class="form-label" for="confirm-email">Confirmar nuevo email</label>
					<input id="confirm-email" type="email" class="form-input" bind:value={confirmEmail} placeholder="nuevo@email.com">
				</div>
				<div class="form-group">
					<label class="form-label" for="email-pw">Contraseña actual <span class="label-optional">(para confirmar)</span></label>
					<input id="email-pw" type="password" class="form-input" bind:value={emailPassword} placeholder="Tu contraseña">
				</div>
			</div>
		</div>

		<div class="card-footer">
			<button onclick={updateEmail} class="btn-primary">Actualizar email</button>
			{#if emailSaved}
				<div class="inline-feedback inline-feedback--success">
					<CheckIcon size={14} />
					<span>Email actualizado. Revisa tu bandeja para confirmar.</span>
				</div>
			{/if}
		</div>
	</div>

	<!-- Change password link card -->
	<div class="card">
		<a href="/cambiar-contrasena" class="change-pw-link">
			<div class="change-pw-icon">
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<path d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
				</svg>
			</div>
			<div class="change-pw-text">
				<span class="change-pw-title">Cambiar contraseña</span>
				<p class="change-pw-subtitle">Actualiza tu contraseña de acceso</p>
			</div>
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="chevron-icon">
				<path d="M9 5l7 7-7 7"/>
			</svg>
		</a>
	</div>

	<!-- Danger zone -->
	<div class="card card--danger">
		<div class="card-body">
			<h3 class="section-heading danger-heading">Zona de Peligro</h3>

			<div class="danger-warning">
				<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="danger-icon">
					<path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
				</svg>
				<div>
					<p class="danger-title">Eliminar cuenta permanentemente</p>
					<p class="danger-desc">Esta acción es permanente y no se puede deshacer. Se eliminarán todos tus datos, tiendas conectadas y DTEs asociados.</p>
				</div>
			</div>

			<ErrorAlert message={deleteError} />

			<div class="form-fields" style="margin-top: 16px;">
				<div class="form-group">
					<label class="form-label" for="delete-pw">Confirma tu contraseña para continuar</label>
					<input id="delete-pw" type="password" class="form-input" bind:value={deletePassword} placeholder="Tu contraseña">
				</div>
				<div class="delete-confirm-row">
					<input type="checkbox" id="delete-check" bind:checked={deleteChecked} style="accent-color: var(--color-red);">
					<label for="delete-check" class="delete-confirm-label">
						Entiendo que esta acción es irreversible y acepto eliminar mi cuenta
					</label>
				</div>
			</div>
		</div>

		<div class="card-footer card-footer--danger">
			<button class="btn-danger" disabled={!deleteChecked || !deletePassword}>
				Eliminar cuenta permanentemente
			</button>
		</div>
	</div>

</div>

<style>
	.page-content {
		padding: 24px;
		max-width: 640px;
		margin: 0 auto;
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.page-hdr { margin-bottom: 4px; }

	/* .page-title, .card, .card--danger, .card-body, .card-footer, .card-footer--danger — global in app.css */

	.card-desc {
		margin: 0 0 16px;
		font-size: 14px;
		color: var(--text-secondary);
	}

	/* ── Profile header ── */
	.profile-header-card { padding: 20px; }

	.avatar-row {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.avatar-wrap { position: relative; flex-shrink: 0; }

	.avatar {
		width: 64px;
		height: 64px;
		border-radius: 12px;
		background: var(--bg-page);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-faint);
	}

	.avatar-edit-btn {
		position: absolute;
		bottom: -4px;
		right: -4px;
		width: 24px;
		height: 24px;
		border-radius: 50%;
		background: var(--bg-surface);
		border: 1px solid var(--border);
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		color: var(--text-muted);
		transition: background 0.15s;
	}

	.avatar-edit-btn:hover { background: var(--bg-page); }

	.profile-greeting {
		margin: 0;
		font-size: 16px;
		font-weight: 700;
		color: var(--text);
	}

	.profile-member {
		margin: 2px 0 0;
		font-size: 13px;
		color: var(--text-muted);
	}

	/* ── Form helpers ── */
	.input-readonly {
		width: 100%;
		background: color-mix(in srgb, var(--bg-page) 40%, transparent);
		border: 1px solid var(--border-subtle);
		border-radius: 8px;
		padding: 10px 16px;
		font-size: 14px;
		color: var(--text-secondary);
	}

	.form-textarea { resize: none; }

	.label-optional {
		font-weight: 400;
		color: var(--text-faint);
	}

	.sub-section {
		margin-top: 24px;
		padding-top: 20px;
		border-top: 1px solid var(--border-subtle);
	}

	/* ── Inline feedback ── */
	.inline-feedback {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
	}

	.inline-feedback--success { color: var(--status-emitido-text); }

	/* ── Change password link ── */
	.change-pw-link {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px 20px;
		text-decoration: none;
		transition: background 0.15s;
	}

	.change-pw-link:hover { background: color-mix(in srgb, var(--bg-page) 40%, transparent); }

	.change-pw-icon {
		width: 32px;
		height: 32px;
		border-radius: 8px;
		background: var(--bg-page);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		color: var(--text-muted);
	}

	.change-pw-text { flex: 1; }

	.change-pw-title {
		display: block;
		font-size: 14px;
		font-weight: 500;
		color: var(--text);
	}

	.change-pw-subtitle {
		margin: 2px 0 0;
		font-size: 12px;
		color: var(--text-muted);
	}

	.chevron-icon { color: var(--text-faint); flex-shrink: 0; }

	/* ── Danger zone ── */
	/* .section-heading — global in app.css; danger-heading overrides color + margin */
	.danger-heading { color: var(--color-red); margin-bottom: 16px; }

	.danger-warning {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		margin-bottom: 16px;
	}

	.danger-icon { color: var(--color-red); flex-shrink: 0; margin-top: 2px; }

	.danger-title {
		margin: 0 0 4px;
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
	}

	.danger-desc {
		margin: 0;
		font-size: 13px;
		color: var(--text-secondary);
	}

	.delete-confirm-row {
		display: flex;
		align-items: flex-start;
		gap: 10px;
	}

	.delete-confirm-label {
		font-size: 13px;
		color: var(--text-secondary);
		cursor: pointer;
		line-height: 1.5;
	}

	.text-ink { color: var(--text); }
</style>
