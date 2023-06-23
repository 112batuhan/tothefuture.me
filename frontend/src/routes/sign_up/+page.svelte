<script lang="ts">
	import { goto } from '$app/navigation';
	import { PUBLIC_BACKEND_URL } from '$env/static/public';

	let email = '';
	let password = '';

	async function sign_up() {
		const res = await fetch(PUBLIC_BACKEND_URL + '/sign_up', {
			method: 'POST',
			credentials: 'include',
			body: JSON.stringify({ email: email, password: password }),
			headers: {
				'Content-Type': 'application/json'
			}
		});
		if (res.ok) {
			goto('/login');
		}
	}
</script>

<div class="card p-4 min-w-min">
	<header class="card-header text-center max-w-sm">
		Sign up with the e-mail adress you want to receive your mail in the future!
	</header>
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
	<label class="label my-3">
		<span class="pl-2">Enter Password Again:</span>
		<input class="input rounded-full" type="password" placeholder="Enter Password Here" />
	</label>
	<div class="mt-6 flex flex-col items-center">
		<button
			on:click={sign_up}
			type="button"
			class="btn variant-ghost-surface rounded-full min-w-min"
		>
			Sign Up!
		</button>
	</div>
</div>
