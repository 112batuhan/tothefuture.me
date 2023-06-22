<script lang="ts">
	import { Accordion, AccordionItem } from '@skeletonlabs/skeleton';
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
			} else {
				emails = [];
			}
		} catch (error) {
			console.error(error);
			emails = [];
		}
	});
</script>

<div class="card p-4 w-[100%] min-w-[600px]">
	{#if emails.length > 0}
		<Accordion autocollapse>
			{#each emails as email}
				<AccordionItem class="variant-ghost-surface">
					<svelte:fragment slot="lead">&#9993;&#65039;</svelte:fragment>
					<svelte:fragment slot="summary">
						<div class="flex">
							<div>can I have divs here</div>
							<div>can I have divs here</div>
						</div>
					</svelte:fragment>
					<svelte:fragment slot="content">(content)</svelte:fragment>
				</AccordionItem>
			{/each}
		</Accordion>
	{:else}
		<p>No emails found.</p>
	{/if}
</div>
