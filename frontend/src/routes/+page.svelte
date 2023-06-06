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
	<label>
		Email
		<input bind:value={email} />
	</label>
	<label>
		Username
		<input bind:value={username} />
	</label>
	<label>
		Password
		<input bind:value={password} />
	</label>
	<label>
		Password again
		<input bind:value={password2nd} />
	</label>

	{#if password != password2nd}
		<div>doesn'T match</div>
	{:else if password === ''}
		<div>doesn'T match</div>
	{:else}
		<button type="button" on:click={doPost}> log in </button>
	{/if}
</form>
<p>Result:</p>
<pre>
{result}
</pre>
