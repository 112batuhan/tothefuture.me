<script lang="ts">
	// The ordering of these imports is critical to your app working properly
	import '@skeletonlabs/skeleton/themes/theme-gold-nouveau.css';
	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
	import { goto } from '$app/navigation';
	import { logged_in } from '$lib/stores/login_state';
	import { onMount } from 'svelte';

	async function logout() {
		const res = await fetch('http://127.0.0.1:3040/logout', {
			method: 'DELETE',
			credentials: 'include'
		});
		if (res.ok || res.status === 401) {
			$logged_in = false;
			goto('/');
		}
	}

	function homepage() {
		goto('/');
	}
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar>
			<svelte:fragment slot="lead">
				<button on:click={homepage}>
					<strong class="text-xl uppercase flex-none mr-10">timecapsule-rs</strong>
				</button>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				{#if $logged_in}
					<a class="btn btn-sm variant-ghost-surface" href="/emails" target="_self"> Email List </a>
					<button on:click={logout} class="btn btn-sm variant-ghost-surface"> logout</button>
				{:else}
					<a class="btn btn-sm variant-ghost-surface" href="/sign_up" target="_self"> Sign Up </a>
					<a class="btn btn-sm variant-ghost-surface" href="/login" target="_self"> Login </a>
				{/if}
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>

	<!-- Page Route Content -->
	<div class="container p-10 mx-auto flex flex-col items-center"><slot /></div>
</AppShell>
