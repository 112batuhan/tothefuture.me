<script>
	let email = '';
	let username = '';
	let password = '';
	let password2nd = '';
	/**
	 * @type {string | null}
	 */
	let result = null;

	async function doPost() {
		console.log('hi');
		const res = await fetch('http://127.0.0.1:3040/users', {
			method: 'POST',
			body: JSON.stringify({
				email,
				username,
				password
			}),
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			}
		});

		result = await res.text();
	}
</script>

<form>
	<div class="container">
		<div class="form_field">
			<label for="email"> Email </label>
			<input bind:value={email} id="email" />
		</div>
		<div class="form_field">
			<label for="username"> Username </label>
			<input bind:value={username} id="username" />
		</div>
		<div class="form_field">
			<label for="password"> Password </label>
			<input bind:value={password} id="password" />
		</div>
		<div class="form_field">
			<label for="second_password"> Confirm password </label>
			<input bind:value={password2nd} id="second_password" />
		</div>

		<div class="sign_in_button">
			{#if password != password2nd}
				Passwords doesn't match
			{:else}
				<button type="button" on:click={doPost}> log in </button>
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
