<script>
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



<div>
	<a href="/sign_up">sign up</a>
</div>
<div>
	<a href="/sign_in">sign in</a>
</div>
<div>
	<a href="/log_out">logout</a>
</div>
<div>
	<a href="/email_editor">email editor</a>
</div>
<div>
	<a href="/email_import">email import</a>
</div>
<div>
	<a href="/email_selector">email selector</a>
</div>
