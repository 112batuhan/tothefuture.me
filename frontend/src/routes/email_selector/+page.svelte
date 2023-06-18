<script>
	// @ts-nocheck

	import { onMount } from 'svelte';
	import { each } from 'svelte/internal';

	let emails = '';

	onMount(async () => {
		let res = await fetch('http://127.0.0.1:3040/get_emails', {
			method: 'GET',
			credentials: 'include'
		})
			.then((response) => response.json())
			.then((json) => {
				emails = json;
			});
	});

	async function send_mail(email_id) {
		const res = await fetch('http://127.0.0.1:3040/send_email/' + email_id, {
			method: 'GET',
			credentials: 'include'
		});
	}
</script>

<table>
	{#each emails as email}
		<tr class="email_line">
			<td class="line_item body_type">{email.is_html === true ? 'HTML' : 'MD'}</td>
			<td class="line_item subject">{email.subject}</td>
			<td class="line_item send_date">{email.send_date}</td>
			<td class="line_item"
				><button class="trial_button" on:click={() => send_mail(email.id)}>Send it to me</button
				></td
			>
		</tr>
	{:else}
		<div>there is nothing</div>
	{/each}
</table>

<style>
	.line_item {
		border-style: solid;
		border-radius: 1rem;
		border-width: 0.1rem;
		padding: 1rem;
		text-align: center;
	}
	.trial_button {
		display: block;
		margin: auto;
	}
</style>
