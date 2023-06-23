<script lang="ts">
	import { Accordion, AccordionItem, Paginator } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	let emails: string | any[] = [];

	onMount(async () => {
		try {
			const res = await fetch('http://127.0.0.1:3040/get_emails', {
				method: 'GET',
				credentials: 'include'
			});

			if (res.ok) {
				emails = await res.json();
				// Paginator email length update
				page.size = emails.length;
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
</script>

<div class="card p-4 w-[100%] min-w-[600px]">
	{#if emails.length > 0}
		<Accordion autocollapse>
			{#each paginatedEmails as email}
				<AccordionItem class="variant-ghost-surface rounded-md">
					<svelte:fragment slot="lead">&#9993;&#65039;</svelte:fragment>
					<svelte:fragment slot="summary">
						<div class="variant-ghost-surface p-1 grid rounded-2xl grid-cols-6 gap-4 items-center">
							<div class="pl-10 col-span-3"><strong>{email.subject}</strong></div>
							<div class="justify-self-center">Date To Send: {email.send_date}</div>
							<div class="justify-self-center">{email.is_html ? 'HTML' : 'TEXT'}</div>
							<div class="justify-self-end pr-3 flex flex-wrap justify-center">
								<button class="btn variant-filled-primary p-1 m-1">Edit</button>
								<button class="btn variant-filled-primary p-1 m-1">Send</button>
								<button class="btn variant-filled-primary p-1 m-1">Hide</button>
							</div>
						</div>
					</svelte:fragment>
					<svelte:fragment slot="content">
						<!-- ... -->

						<div
							style="border-radius:0.3rem; background-color:#c2a6f5; 100%; height: 350px; overflow: hidden;"
						>
							<iframe
								style="max-width: 100%; max-height: 100%; width: 100%; height: 100%;"
								title="preview"
								srcdoc={email.body}
							/>
						</div>
					</svelte:fragment>
				</AccordionItem>
			{/each}
		</Accordion>
	{:else}
		<p>No emails found.</p>
	{/if}

	<Paginator class="m-3" bind:settings={page} showNumerals maxNumerals={1} />
</div>
