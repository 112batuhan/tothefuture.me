<script>
	// @ts-nocheck

	import { goto } from '$app/navigation';

	let email = '';
	let password = '';

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
			credentials: 'include',
			headers: {
				Accept: ' application/x-www-form-urlencoded',
				'Content-Type': ' application/x-www-form-urlencoded'
			},
			body: formData.toString()
		});
		if (res.ok) {
			goto('/');
		}
	}
</script>

<form action="http://127.0.0.1:3040/sign_in" method="post" on:submit|preventDefault={doPost}>
	<div class="container">
		<div class="form_field">
			<label for="email"> Email </label>
			<input bind:value={email} name="email" id="email" />
		</div>
		<div class="form_field">
			<label for="password"> Password </label>
			<input bind:value={password} name="password" id="password" />
		</div>

		<div class="submit_button">
			<input type="submit" value="log in" />
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
	.submit_button {
		margin-top: 0.5rem;
		text-align: center;
		margin-bottom: 0.5rem;
	}
</style>
