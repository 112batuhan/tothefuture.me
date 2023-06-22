<script lang="ts">
	import { goto } from '$app/navigation';
	import { logged_in } from '$lib/stores/login_state';

	let email = '';
	let password = '';

	async function login() {
		const res = await fetch('http://127.0.0.1:3040/login', {
			method: 'POST',
			credentials: 'include',
			body: JSON.stringify({ email: email, password: password }),
			headers: {
				'Content-Type': 'application/json'
			}
		});
		if (res.ok) {
			$logged_in = true;
			goto('/emails');
		}
	}
</script>

<div class="card p-4 min-w-min">
	<header class="card-header text-center">Login to shape your future!</header>
	<label class="label my-3">
		<span class="pl-2">Email:</span>
		<input
			bind:value={email}
			class="input rounded-full"
			type="email"
			placeholder="your.email@adress.com"
		/>
	</label>
	<label class="label my-3">
		<span class="pl-2">Password:</span>
		<input
			bind:value={password}
			class="input rounded-full"
			type="password"
			placeholder="Enter Password Here"
		/>
	</label>
	<div class="mt-6 flex flex-col items-center">
		<button type="button" on:click={login} class="btn variant-ghost-surface rounded-full min-w-min">
			Login!
		</button>
	</div>
</div>
