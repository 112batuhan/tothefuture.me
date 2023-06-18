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
</script>

{#each emails as email}
	<div>
		{email.subject}, {email.is_html === true ? 'HTML' : 'MD'}, {email.send_date}
	</div>
{:else}
	<div>there is nothing</div>
{/each}
