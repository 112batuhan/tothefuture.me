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
		const res = await fetch('https://httpbin.org/post', {
			method: 'POST',
			body: JSON.stringify({
				email,
				username,
				password
			})
		});

		const json = await res.json();
		result = JSON.stringify(json);
	}
</script>

<form>
	<div class="container">
		<div class="form_field">
			<label for="email"> Email </label>
			<input bind:value={email} id="email" />
		</div>
        <div class="form_field">
			<label for="username"> Email </label>
			<input bind:value={email} id="username" />
		</div>
        <div class="form_field">
			<label for="password"> Email </label>
			<input bind:value={email} id="password" />
		</div>
        <div class="form_field">
			<label for="second_password"> Email </label>
			<input bind:value={email} id="second_password" />
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
    .form_field{
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
