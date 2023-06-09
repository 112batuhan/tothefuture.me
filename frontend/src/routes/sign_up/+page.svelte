<script>
	// @ts-nocheck

	import { goto } from '$app/navigation';

	let email = '';
	let username = '';
	let password = '';
	let password2nd = '';
	/**
	 * @type {string | null}
	 */
	let result = null;

	async function doPost(e) {
		const data = new FormData(e.target);
		const formData = new URLSearchParams();

		for (var [key, value] of data.entries()) {
			formData.append(key, value);
		}
		const res = await fetch(e.target.action, {
			method: 'POST',
			headers: {
				Accept: ' application/x-www-form-urlencoded',
				'Content-Type': ' application/x-www-form-urlencoded'
			},
			body: formData.toString()
		});
		if (res.ok) {
			goto('/success');
		}
	}
</script>

<form action="http://127.0.0.1:3040/users" method="post" on:submit|preventDefault={doPost}>
	<div class="container">
		<div class="form_field">
			<label for="email"> Email </label>
			<input bind:value={email} name="email" id="email" />
		</div>
		<div class="form_field">
			<label for="username"> Username </label>
			<input bind:value={username} name="username" id="username" />
		</div>
		<div class="form_field">
			<label for="password"> Password </label>
			<input bind:value={password} name="password" id="password" />
		</div>
		<div class="form_field">
			<label for="second_password"> Confirm password </label>
			<input bind:value={password2nd} id="second_password" />
		</div>

		<div class="sign_in_button">
			{#if password != password2nd}
				Passwords doesn't match
			{:else}
				<input type="submit" value="log in" />
			{/if}
		</div>
	</div>
</form>
<p>Result:</p>
<pre>
{result}
</pre>

<style>
	.container {
		border: 0.1rem solid;
		border-color: rgb(0, 0, 0);
		border-radius: 1rem;
		width: fit-content;
	}
	.form_field {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding-inline: 1rem;
	}
	.sign_in_button {
		margin-top: 0.5rem;
		text-align: center;
		margin-bottom: 0.5rem;
	}
</style>
