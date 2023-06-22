<script lang="ts">
	//@ts-nocheck
	//these libs are beyond me with their weird errors
	import { AceEditor } from 'svelte-ace';
	import 'brace/mode/html';
	import 'brace/theme/monokai';
	import { html } from 'js-beautify';
	import { onMount } from 'svelte';

	export let text = `
              <head>
                <style>
                  body {
                    font-family: Arial, sans-serif;
                    background-color: #f2f2f2;
                  }

                  h1 {
                    color: #333;
                    text-align: center;
                  }

                  p {
                    color: #666;
                    text-align: center;
                  }
                </style>
              </head>
              <body>
                <h1>Hello, World!</h1>
                <p>This is a short HTML document with some styling.</p>
              </body>
              
              `;

	function format() {
		text = html(text, { preserve_newlines: true, wrap_line_length: 80 });
	}

	onMount(() => {
		format();
	});
</script>

<div class="flex flex-col gap-4 sm:flex-row">
	<div class="card variant-soft-surface p-4 w-full sm:w-1/2">
		<div class="flex justify-between">
			<div class="text-xl p-2 ml-5"><strong>Editor</strong></div>
			<button on:click={format} class="card variant-ghost-surface p-2 mr-5">Beautify</button>
		</div>
		<div class="mt-3">
			<AceEditor width="100%" height="600px" lang="html" theme="monokai" bind:value={text} />
		</div>
	</div>
	<div class="card variant-soft-surface w-full sm:w-1/2 p-4">
		<div class="text-xl p-2 ml-5 mb-3"><strong>preview</strong></div>
		<div style="width: 100%; height: 600px; overflow: hidden;">
			<iframe
				style="max-width: 100%; max-height: 100%; width: 100%; height: 100%;"
				title="preview"
				srcdoc={text}
			/>
		</div>
	</div>
</div>
