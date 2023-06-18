<script>
	import { onMount } from 'svelte';

	onMount(async () => {
		let res = await fetch('http://127.0.0.1:3040/auto_login', {
			method: 'GET',
			credentials: 'include'
		});
		if (res.ok) {
			//store it
		}
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
