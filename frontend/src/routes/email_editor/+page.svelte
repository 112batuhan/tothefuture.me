<script lang="ts">
	// @ts-nocheck
	// nocheck is used to suppress svelte-ace error.
	import { AceEditor } from 'svelte-ace';
	import 'brace/mode/html';
	import 'brace/theme/chrome';
	import { html } from 'js-beautify';
	import { onMount } from 'svelte';

	let text = `<!DOCTYPE html>
              <html>
              <head>
                <title>Short HTML with Styling</title>
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
              </html>
              `;

	function format() {
		text = html(text, { preserve_newlines: true, wrap_line_length: 80 });
	}

	onMount(() => {
		format();
	});
</script>

<AceEditor class
	on:selectionChange={(obj) => console.log(obj.detail)}
	on:paste={(obj) => console.log(obj.detail)}
	on:input={(obj) => console.log(obj.detail)}
	on:focus={() => console.log('focus')}
	on:documentChange={(obj) => console.log(`document change : ${obj.detail}`)}
	on:cut={() => console.log('cut')}
	on:cursorChange={() => console.log('cursor change')}
	on:copy={() => console.log('copy')}
	on:init={(editor) => console.log(editor.detail)}
	on:commandKey={(obj) => console.log(obj.detail)}
	on:changeMode={(obj) => console.log(`change mode : ${obj.detail}`)}
	on:blur={() => console.log('blur')}
	width="100%"
	height="600px"
	lang="html"
	theme="chrome"
	bind:value={text}
/>

<button on:click={format}> Click me </button>


<iframe title="preview" srcdoc={text} />

