<script lang="ts">
	import { logged_in, user_email } from '$lib/stores/login_state';
	import { goto } from '$app/navigation';

	import { afterNavigate } from '$app/navigation';
	import { page } from '$app/stores';
	import MdEditor from '$lib/components/MdEditor.svelte';

	afterNavigate(async ({ from }) => {
		// Run only on first login.
		// Can be simplified because in first login from returns undefined but this is more safe
		if (from?.url.origin != $page.url.origin) {
			let res = await fetch('http://127.0.0.1:3040/auto_login', {
				method: 'GET',
				credentials: 'include'
			});
			if (res.ok) {
				$logged_in = true;
				let response_json = await res.json();
				$user_email = response_json.email;
				goto('/emails');
			}
		}
	});
</script>

<MdEditor />
