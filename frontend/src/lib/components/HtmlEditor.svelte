<script lang="ts">
	//@ts-nocheck
	//these libs are beyond me with their weird errors

	import { html } from 'js-beautify';
	import { onMount } from 'svelte';

	export let text = '';
	let editor;

	function format() {
		text = html(text, { preserve_newlines: true, wrap_line_length: 80 });
	}

	onMount(async () => {
		format();
		await import('brace/mode/html');
		await import('brace/theme/monokai');
		editor = (await import('svelte-ace')).default;
		console.log(editor);
	});
</script>

<div class="flex flex-col gap-4 lg:flex-row">
	<div class="card variant-soft-surface p-4 w-full lg:w-1/2">
		<div class="flex justify-between">
			<div class="text-xl p-2 ml-5"><strong>Editor</strong></div>
			<button on:click={format} class="card variant-ghost-surface p-2 mr-5">Beautify</button>
		</div>
		<div class="mt-3">
			{#if editor}
				<svelte:component
					this={editor}
					width="100%"
					height="550px"
					lang="html"
					theme="monokai"
					bind:value={text}
				/>
			{/if}
		</div>
	</div>
	<div class="card variant-soft-surface w-full lg:w-1/2 p-4">
		<div class="text-xl p-2 ml-5 mb-3 flex justify-start"><strong>Preview</strong></div>
		<div style="width: 100%; height: 550px; overflow: hidden;">
			<iframe
				style="max-width: 100%; max-height: 100%; width: 100%; height: 100%;"
				title="preview"
				srcdoc={text}
			/>
		</div>
	</div>
</div>
