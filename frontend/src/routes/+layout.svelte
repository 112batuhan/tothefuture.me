<script lang="ts">
	// The ordering of these imports is critical to your app working properly
	import '@skeletonlabs/skeleton/themes/theme-gold-nouveau.css';
	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
	import { afterNavigate, goto } from '$app/navigation';
	import { loginStore, userEmail, LoginState } from '$lib/stores/loginState';
	import { page } from '$app/stores';
	import { PUBLIC_BACKEND_URL } from '$env/static/public';
	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { Modal } from '@skeletonlabs/skeleton';

	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	async function logout() {
		const res = await fetch(PUBLIC_BACKEND_URL + '/logout', {
			method: 'DELETE',
			credentials: 'include'
		});
		if (res.ok || res.status === 401) {
			$loginStore = LoginState.Not;
			goto('/');
		}
	}

	let logged_in_hrefs = [
		{ link: '/create_email', text: 'Create Email' },
		{ link: '/emails', text: 'Emails' }
	];
	let logged_out_hrefs = [
		{ link: '/sign_up', text: 'Sign Up' },
		{ link: '/login', text: 'Login' }
	];

	afterNavigate(async ({ from }) => {
		// Run only on first login.
		if ($loginStore === LoginState.FirstLogin) {
			try {
				let res = await fetch(PUBLIC_BACKEND_URL + '/auto_login', {
					method: 'GET',
					credentials: 'include'
				});
				if (res.ok) {
					$loginStore = LoginState.Logged;
					let response_json = await res.json();
					$userEmail = response_json.email;
				} else {
					$loginStore = LoginState.Not;
					goto('/');
				}
			} catch {
				$loginStore = LoginState.Not;
			}
		}
	});
</script>

<!-- Modal is supposed to be defined once-->
<Modal buttonPositive="variant-filled-error" />

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
			<svelte:fragment slot="lead">
				<button on:click={() => goto('/')}>
					<strong class="text-xl flex-none mr-10">ToTheFuture.Me</strong>
				</button>
			</svelte:fragment>

			{#if $loginStore === LoginState.Logged}
				<div class="card p-2 variant-soft-surface">&#9993;&#65039; {$userEmail}</div>
			{/if}

			<svelte:fragment slot="trail">
				{#if $loginStore === LoginState.Logged}
					{#each logged_in_hrefs as href}
						{#if $page.route.id != href.link}
							<button on:click={() => goto(href.link)} class="btn btn-sm variant-ghost-surface">
								{href.text}
							</button>
						{/if}
					{/each}
					<button on:click={logout} class="btn btn-sm variant-ghost-surface"> Logout</button>
				{:else if $loginStore === LoginState.Not}
					{#each logged_out_hrefs as href}
						{#if $page.route.id != href.link}
							<button on:click={() => goto(href.link)} class="btn btn-sm variant-ghost-surface">
								{href.text}
							</button>
						{/if}
					{/each}
				{/if}
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>

	<!-- Page Route Content -->
	<div class="container p-10 mx-auto flex flex-col items-center">
		<slot />
	</div>
</AppShell>
