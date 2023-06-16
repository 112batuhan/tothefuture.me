<script>
	// @ts-nocheck
	import { onMount } from 'svelte';
	let emails = '';

	onMount(async () => {
		let res = await fetch('http://127.0.0.1:3040/get_emails', {
			method: 'GET',
			credentials: 'include'
		})
			.then((response) => response.json())
			.then((json) => {
				emails = JSON.stringify(json);
			});
	});
</script>

{emails}
