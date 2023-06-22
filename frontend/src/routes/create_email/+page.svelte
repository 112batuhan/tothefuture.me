<script lang="ts">
	import { Stepper, Step } from '@skeletonlabs/skeleton';
	import { SlideToggle } from '@skeletonlabs/skeleton';
	import HtmlEditor from '$lib/components/HtmlEditor.svelte';

	let is_html: boolean = false;
</script>

<div class="card p-4 w-[100%] min-w-[300px]">
	<Stepper>
		<Step>
			<svelte:fragment slot="header">
				<div class="text-center">Welcome to the mail builder!</div>
			</svelte:fragment>
			<div class="text-center">By following this, you will be able to create your future!</div>
		</Step>
		<Step>
			<svelte:fragment slot="header">
				<div class="text-center">Select the date that we should send this mail</div>
			</svelte:fragment>
			<div class="flex justify-center p-10">
				<input type="date" class="input max-w-sm" />
			</div>
		</Step>
		<Step>
			<svelte:fragment slot="header">
				<div class="text-center">Select the type of the mail body</div>
			</svelte:fragment>
			<div class="text-center">
				Select text for creating a simple text based mail body. Select HTML if you are experienced
				with it and want to create customised email bodies.
			</div>
			<div class="flex justify-center p-10">
				<div class="card variant-ghost-surface flex justify-center items-center">
					<div class="p-4">Text</div>
					<SlideToggle name="slide" bind:checked={is_html} class="p-4" />
					<div class="p-4">HTML</div>
				</div>
			</div>
		</Step>
		<Step>
			<svelte:fragment slot="header">
				<div class="text-center">Edit the mail!</div>
			</svelte:fragment>
			<div class="text-center"><HtmlEditor /></div>
		</Step>
		<!-- ... -->
	</Stepper>
</div>
