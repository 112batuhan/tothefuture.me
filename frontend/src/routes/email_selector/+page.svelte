<script>
	// @ts-nocheck

	let email_body = '';

	async function insert_email() {
		const res = await fetch('http://127.0.0.1:3040/insert_email', {
			method: 'DELETE',
			credentials: 'include'
		});
		if (res.ok) {
			
		}
	}
</script>

<textarea class="email_input" bind:value={email_body} />
<div>
	<input type="submit" value="submit" />
</div>

<style>
	.email_input {
		height: 600px;
		width: 600px;
	}

</style>
