<script lang="ts">
	import ErrorAlert from '$lib/components/ErrorAlert.svelte';

	// Backend: UC2 (sign in) is not implemented yet.
	// Form is functional UI; submit will be wired when UC2 is done.
	let email = $state('');
	let password = $state('');
	let errorMessage = $state('');
	let loading = $state(false);

	function handleSubmit(e: Event) {
		e.preventDefault();
		// TODO UC2: POST /auth/login → store JWT → goto('/')
		errorMessage = 'Inicio de sesión no implementado aún.';
	}
</script>

<div class="auth-card">
	<h2 class="auth-card__title">Iniciar Sesión</h2>

	<form onsubmit={handleSubmit}>
		<div class="form-fields">
			<div class="form-group">
				<label for="email" class="form-label">Email</label>
				<input type="email" id="email" bind:value={email} placeholder="tu@email.com" required class="form-input">
			</div>
			<div class="form-group">
				<label for="contrasena" class="form-label">Contraseña</label>
				<input type="password" id="contrasena" bind:value={password} placeholder="Tu contraseña" required class="form-input">
			</div>
		</div>

		<ErrorAlert message={errorMessage} />

		<button type="submit" disabled={loading} class="btn-primary btn-full">
			{loading ? 'Iniciando...' : 'Iniciar sesión'}
		</button>
	</form>

	<div class="auth-card__footer">
		<p><a href="/reset-password" class="link-subtle">¿Olvidaste tu contraseña?</a></p>
		<p class="footer-text">
			¿No tienes cuenta? <a href="/register" class="link-primary">Crear cuenta</a>
		</p>
	</div>
</div>

<style>
	/* .btn-full — global in app.css */
</style>
