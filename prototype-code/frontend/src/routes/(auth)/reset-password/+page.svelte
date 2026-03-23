<script lang="ts">
	import ErrorAlert from '$lib/components/ErrorAlert.svelte';
	import CheckIcon from '$lib/components/CheckIcon.svelte';

	type Step = 'request' | 'confirmation' | 'new-password' | 'success';

	let step = $state<Step>('request');

	// Step 1: request
	let email = $state('');

	function sendLink() {
		if (email) step = 'confirmation';
	}

	// Step 3: new password
	let newPw = $state('');
	let confirmPw = $state('');
	let pwError = $state('');

	function resetPassword() {
		if (!newPw || newPw !== confirmPw) {
			pwError = 'Las contraseñas no coinciden.';
			return;
		}
		pwError = '';
		step = 'success';
	}
</script>

<!-- Step 1: Request link -->
{#if step === 'request'}
<div class="auth-card">
	<h2 class="auth-card__title">Recuperar Contraseña</h2>
	<p class="step-desc">Ingresa tu email y te enviaremos un enlace para restablecer tu contraseña.</p>

	<div class="form-fields">
		<div class="form-group">
			<label class="form-label" for="rp-email">Email</label>
			<input
				id="rp-email"
				type="email"
				class="form-input"
				bind:value={email}
				placeholder="tu@email.com"
				onkeydown={(e) => e.key === 'Enter' && sendLink()}
			>
		</div>
	</div>

	<button onclick={sendLink} class="btn-primary btn-full">Enviar enlace</button>

	<div class="auth-card__footer">
		<p class="footer-text">
			<a href="/login" class="link-subtle">← Volver a iniciar sesión</a>
		</p>
	</div>
</div>

<!-- Step 2: Confirmation -->
{:else if step === 'confirmation'}
<div class="auth-card">
	<div class="success-icon-wrap">
		<div class="success-icon">
			<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
			</svg>
		</div>
		<h3 class="success-title">¡Revisa tu correo!</h3>
		<p class="success-desc">Enviamos un enlace de recuperación a</p>
		<p class="success-email">{email || 'tu email'}</p>
		<p class="success-hint">Si no lo ves, revisa tu carpeta de spam.</p>
	</div>

	<button class="btn-secondary btn-full">Reenviar enlace</button>

	<div class="auth-card__footer">
		<p class="footer-text">
			<a href="/login" class="link-subtle">← Volver a iniciar sesión</a>
		</p>
	</div>
</div>

<!-- Step 3: New password (arrived via email link) -->
{:else if step === 'new-password'}
<div class="auth-card">
	<h2 class="auth-card__title">Nueva Contraseña</h2>

	<ErrorAlert message={pwError} />

	<div class="form-fields">
		<div class="form-group">
			<label class="form-label" for="rp-new">Nueva contraseña</label>
			<input id="rp-new" type="password" class="form-input" bind:value={newPw} placeholder="Mínimo 8 caracteres">
		</div>
		<div class="form-group">
			<label class="form-label" for="rp-confirm">Confirmar contraseña</label>
			<input id="rp-confirm" type="password" class="form-input" bind:value={confirmPw} placeholder="Repite la contraseña">
		</div>
	</div>

	<button onclick={resetPassword} class="btn-primary btn-full">Restablecer contraseña</button>
</div>

<!-- Step 4: Success -->
{:else if step === 'success'}
<div class="auth-card">
	<div class="success-icon-wrap">
		<div class="success-icon">
			<CheckIcon size={24} />
		</div>
		<h3 class="success-title">¡Contraseña actualizada!</h3>
		<p class="success-desc">Tu contraseña fue restablecida correctamente. Ya puedes iniciar sesión.</p>
	</div>

	<a href="/login" class="btn-primary btn-full" style="text-decoration: none; text-align: center;">Iniciar sesión</a>
</div>
{/if}

<!-- Demo step switcher (remove in production) -->
<div class="demo-nav">
	<button onclick={() => step = 'request'} class:demo-active={step === 'request'}>1</button>
	<button onclick={() => step = 'confirmation'} class:demo-active={step === 'confirmation'}>2</button>
	<button onclick={() => step = 'new-password'} class:demo-active={step === 'new-password'}>3</button>
	<button onclick={() => step = 'success'} class:demo-active={step === 'success'}>4</button>
</div>

<style>
	.step-desc {
		margin: 0 0 20px;
		font-size: 14px;
		color: var(--text-muted);
	}

	/* ── Success states ── */
	.success-icon-wrap {
		text-align: center;
		margin-bottom: 20px;
	}

	.success-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		background: var(--primary-subtle);
		color: var(--primary-text);
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0 auto 12px;
	}

	.success-title {
		margin: 0 0 6px;
		font-size: 16px;
		font-weight: 700;
		color: var(--text);
	}

	.success-desc {
		margin: 0;
		font-size: 14px;
		color: var(--text-muted);
	}

	.success-email {
		margin: 4px 0 8px;
		font-size: 14px;
		font-weight: 600;
		color: var(--text);
	}

	.success-hint {
		margin: 0 0 20px;
		font-size: 12px;
		color: var(--text-faint);
	}

	/* ── Demo nav (prototype only) ── */
	.demo-nav {
		display: flex;
		gap: 6px;
		margin-top: 12px;
		justify-content: center;
	}

	.demo-nav button {
		width: 24px;
		height: 24px;
		border-radius: 50%;
		border: 1px solid var(--border);
		background: var(--bg-surface);
		cursor: pointer;
		font-size: 11px;
		color: var(--text-muted);
		font-family: inherit;
	}

	.demo-nav button.demo-active {
		background: var(--primary);
		border-color: var(--primary);
		color: white;
	}
</style>
