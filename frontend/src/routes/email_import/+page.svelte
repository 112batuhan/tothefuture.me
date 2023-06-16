<script>
	// @ts-nocheck

	let email_body = '';
	let send_date = '';

	async function insert_email() {
		console.log(send_date);
		const res = await fetch('http://127.0.0.1:3040/create_email', {
			method: 'POST',
			credentials: 'include',
			body: JSON.stringify({ email: email_body, date: send_date }),
			headers: {
				'Content-Type': 'application/json'
			}
		});
		if (res.ok) {
		}
	}
</script>

<textarea class="email_input" bind:value={email_body} />

<div>
	<input bind:value={send_date} type="date" />
	select a date to activate button
</div>
{#if send_date !== ''}
	<div>
		<button on:click={insert_email}>Submit</button>
	</div>
{/if}

<style>
	.email_input {
		height: 600px;
		width: 600px;
	}
</style>
