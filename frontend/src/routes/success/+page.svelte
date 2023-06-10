<script>
	// @ts-nocheck

	import { goto } from '$app/navigation';

	async function sayHello(e) {
		const res = await fetch('http://127.0.0.1:3040/logout', {
			method: 'DELETE',
			credentials: 'include'
		});
		if (res.ok) {
			goto('/sign_in');
		}
	}
</script>

<button on:click={sayHello}> Click me </button>
