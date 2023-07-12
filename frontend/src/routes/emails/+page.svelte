<script lang="ts">
	import { goto } from '$app/navigation';
	import { PUBLIC_BACKEND_URL } from '$env/static/public';
	import { LoginState, loginStore } from '$lib/stores/login_state';
	import { Accordion, AccordionItem, Paginator } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	let emails: any = [];

	onMount(async () => {
		try {
			const res = await fetch(PUBLIC_BACKEND_URL + '/get_emails', {
				method: 'GET',
				credentials: 'include'
			});

			if (res.status == 200) {
				emails = await res.json();
				// Paginator email length update
				emails.forEach((item: any, i: number) => {
					item.display_id = i;
				});
				page.size = emails.length;
			} else if (res.status == 401) {
				$loginStore = LoginState.Not;
				goto('/');
			} else {
				emails = [];
			}
		} catch (error) {
			console.error(error);
			emails = [];
		}
	});

	let page = {
		offset: 0,
		limit: 5,
		size: emails.length,
		amounts: [1, 2, 5, 10]
	};
	$: paginatedEmails = emails.slice(
		page.offset * page.limit,
		page.offset * page.limit + page.limit
	);
	let active_item = -1;

	function calculateLastActivePage(amount_event: any) {
		if (active_item < 0) {
			page.offset = 0;
		} else {
			page.offset = Math.floor(active_item / amount_event.detail);
			console.log(page);
		}
	}
</script>

<div class="card p-4 w-[100%] min-w-[600px]">
	{#if emails.length > 0}<Paginator
			class="m-3"
			bind:settings={page}
			on:amount={calculateLastActivePage}
			showNumerals
			maxNumerals={1}
		/>
		<Accordion>
			{#each paginatedEmails as email}
				<AccordionItem
					on:toggle={() => (active_item = email.display_id)}
					class="variant-ghost-surface rounded-md"
					open={active_item == email.display_id}
				>
					<svelte:fragment slot="lead">&#9993;&#65039;</svelte:fragment>
					<svelte:fragment slot="summary">
						<div class="variant-ghost-surface p-1 grid rounded-2xl grid-cols-12 gap-4 items-center">
							<div class="pl-10 col-span-8"><strong>{email.subject}</strong></div>
							<div class="justify-self-center text-center">Send Date {email.send_date}</div>
							<div class="justify-self-center text-center">{email.is_html ? 'HTML' : 'TEXT'}</div>
							{#if active_item == email.display_id}
								<div class="justify-self-center pr-3 col-span-2 flex flex-wrap justify-center">
									<button class="btn variant-filled-primary p-1 m-1">Edit</button>
									<button class="btn variant-filled-primary p-1 m-1">Send</button>
									<button class="btn variant-filled-primary p-1 m-1">Hide</button>
								</div>
							{/if}
						</div>
					</svelte:fragment>
					<svelte:fragment slot="content">
						<div
							style="border-radius:0.3rem; background-color:#c2a6f5; 100%; height: 350px; overflow: hidden;"
						>
							{#if email.is_html}
								<iframe
									style="max-width: 100%; max-height: 100%; width: 100%; height: 100%;"
									title="preview"
									srcdoc={email.body}
								/>{:else}
								<iframe
									style="max-width: 100%; max-height: 100%; width: 100%; height: 100%; text-color:#000000;"
									title="preview"
									srcdoc={email.body}
								/>{/if}
						</div>
					</svelte:fragment>
				</AccordionItem>
			{/each}
		</Accordion>
	{:else}
		<p>No emails found.</p>
	{/if}
</div>
