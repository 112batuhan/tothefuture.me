<script lang="ts">
	import { Accordion, AccordionItem } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	let emails = '';

	onMount(async () => {
		await fetch('http://127.0.0.1:3040/get_emails', {
			method: 'GET',
			credentials: 'include'
		})
			.then((response) => response.json())
			.then((json) => {
				emails = json;
			});
	});
</script>

<div class="card p-4 w-[100%] min-w-[600px]">
	<Accordion autocollapse>
		{#each emails as email}
			<AccordionItem class="variant-ghost-surface">
				<svelte:fragment slot="lead">&#9993;&#65039;</svelte:fragment>
				<svelte:fragment slot="summary">
					<div class="flex">
						<div>can i have divs here</div>
						<div>can i have divs here</div>
					</div>
				</svelte:fragment>
				<svelte:fragment slot="content">(content)</svelte:fragment>
			</AccordionItem>
		{/each}
	</Accordion>
</div>
