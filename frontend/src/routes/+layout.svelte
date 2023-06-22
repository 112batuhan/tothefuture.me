<script lang="ts">
	// The ordering of these imports is critical to your app working properly
	import '@skeletonlabs/skeleton/themes/theme-gold-nouveau.css';
	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
	import { goto } from '$app/navigation';
	import { logged_in, user_email } from '$lib/stores/login_state';

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
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
			<svelte:fragment slot="lead">
				<button on:click={() => goto('/')}>
					<strong class="text-xl uppercase flex-none mr-10">timecapsule-rs</strong>
				</button>
			</svelte:fragment>

			{#if $logged_in}
				<div class="card p-4 variant-soft-surface">{$user_email}</div>
			{/if}

			<svelte:fragment slot="trail">
				{#if $logged_in}
					<button on:click={() => goto('/create_email')} class="btn btn-sm variant-ghost-surface">
						Create Email
					</button>
					<button on:click={() => goto('/emails')} class="btn btn-sm variant-ghost-surface">
						Email List
					</button>
					<button on:click={logout} class="btn btn-sm variant-ghost-surface"> Logout</button>
				{:else}
					<button on:click={() => goto('/sign_up')} class="btn btn-sm variant-ghost-surface">
						Sign Up
					</button>
					<button on:click={() => goto('/login')} class="btn btn-sm variant-ghost-surface">
						Login
					</button>
				{/if}
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>

	<!-- Page Route Content -->
	<div class="container p-10 mx-auto flex flex-col items-center">
		<slot />
	</div>
</AppShell>
