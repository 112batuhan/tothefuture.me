<script lang="ts">
	import { Accordion, AccordionItem } from '@skeletonlabs/skeleton';
</script>

<div class="card p-4 w-[100%] min-w-[600px]">
	<Accordion autocollapse>
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
		<AccordionItem class="variant-ghost-surface">
			<svelte:fragment slot="lead">&#9993;&#65039;</svelte:fragment>
			<svelte:fragment slot="summary">(summary)</svelte:fragment>
			<svelte:fragment slot="content">(content)</svelte:fragment>
		</AccordionItem>
		<!-- ... -->
	</Accordion>
</div>
