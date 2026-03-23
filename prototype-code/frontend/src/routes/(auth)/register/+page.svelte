<script lang="ts">
	import { goto } from '$app/navigation';
	import { apiFetch } from '$lib/api/client';
	import ErrorAlert from '$lib/components/ErrorAlert.svelte';

	let name = $state('');
	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let acceptedTerms = $state(false);
	let errorMessage = $state('');
	let loading = $state(false);

	async function handleSubmit(e: Event) {
		e.preventDefault();
		errorMessage = '';

		if (password !== confirmPassword) {
			errorMessage = 'Las contraseñas no coinciden.';
			return;
		}
		if (!acceptedTerms) {
			errorMessage = 'Debes aceptar los términos de uso.';
			return;
		}

		loading = true;
		try {
			const res = await apiFetch('/auth/register', {
				method: 'POST',
				body: JSON.stringify({ name, email, password }),
			});

			if (res.ok) {
				goto('/login');
				return;
			}

			const body = await res.json().catch(() => ({}));
			if (res.status === 409) {
				errorMessage = body.error ?? 'Ya existe una cuenta con este email.';
			} else {
				errorMessage = body.error ?? 'Ocurrió un error. Intenta de nuevo.';
			}
		} catch {
			errorMessage = 'No se pudo conectar al servidor.';
		} finally {
			loading = false;
		}
	}
</script>

<div class="auth-card">
	<h2 class="auth-card__title">Crear Cuenta</h2>

	<form onsubmit={handleSubmit}>
		<div class="form-fields">
			<div class="form-group">
				<label for="nombre" class="form-label">Nombre</label>
				<input type="text" id="nombre" bind:value={name} placeholder="Tu nombre" required class="form-input">
			</div>
			<div class="form-group">
				<label for="email" class="form-label">Email</label>
				<input type="email" id="email" bind:value={email} placeholder="tu@email.com" required class="form-input">
			</div>
			<div class="form-group">
				<label for="contrasena" class="form-label">Contraseña</label>
				<input type="password" id="contrasena" bind:value={password} placeholder="Elige una contraseña" required class="form-input">
			</div>
			<div class="form-group">
				<label for="confirmar" class="form-label">Confirmar contraseña</label>
				<input type="password" id="confirmar" bind:value={confirmPassword} placeholder="Repite la contraseña" required class="form-input">
			</div>
		</div>

		<div class="terms-row">
			<input type="checkbox" id="terminos" bind:checked={acceptedTerms} class="terms-checkbox">
			<label for="terminos" class="terms-label">
				Acepto los <a href="/terminos" class="link-primary">términos de uso y política de privacidad</a>
			</label>
		</div>

		<ErrorAlert message={errorMessage} />

		<button type="submit" disabled={loading} class="btn-primary btn-full">
			{loading ? 'Creando cuenta...' : 'Crear cuenta'}
		</button>
	</form>

	<div class="auth-card__footer">
		<p class="footer-text">
			¿Ya tienes cuenta? <a href="/login" class="link-primary">Iniciar sesión</a>
		</p>
	</div>
</div>

<style>
	.terms-row {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		margin-bottom: 24px;
	}

	.terms-checkbox {
		margin-top: 2px;
		width: 16px;
		height: 16px;
		flex-shrink: 0;
		accent-color: var(--primary);
		cursor: pointer;
	}

	.terms-label {
		font-size: 14px;
		color: var(--text-secondary);
		cursor: pointer;
	}

	/* .btn-full — global in app.css */
</style>
