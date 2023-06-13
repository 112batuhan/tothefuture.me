<script lang="ts">
	import { AceEditor } from 'svelte-ace';
	import 'brace/mode/html';
	import 'brace/theme/chrome';
	import { html } from 'js-beautify';
	var text = `<!DOCTYPE html>
<html>
<head>
  <title>Random HTML with Styling</title>
  <style>
    body {
      font-family: Arial, sans-serif;
      background-color: #f2f2f2;
    }

    h1 {
      color: #333333;
      text-align: center;
    }

    p {
      color: #666666;
      line-height: 1.5;
      margin-bottom: 20px;
    }

    .container {
      width: 80%;
      margin: 0 auto;
      background-color: #ffffff;
      padding: 20px;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .btn {
      display: inline-block;
      padding: 10px 20px;
      background-color: #428bca;
      color: #ffffff;
      text-decoration: none;
      border-radius: 5px;
    }

    .btn:hover {
      background-color: #3071a9;
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>Welcome to Random HTML</h1>
    <p>This is a randomly generated HTML code snippet with some styling. Feel free to modify it to suit your needs.</p>
    <p>Here's a sample button:</p>
    <a href="#" class="btn">Click Me!</a>
  </div>
</body>
</html>
`;


</script>

<AceEditor
	on:selectionChange={(obj) => console.log(obj.detail)}
	on:paste={(obj) => console.log(obj.detail)}
	on:input={() => (text = html(text))}
	on:focus={() => console.log('focus')}
	on:documentChange={(obj) => console.log(`document change : ${obj.detail}`)}
	on:cut={() => console.log('cut')}
	on:cursorChange={() => console.log('cursor change')}
	on:copy={() => console.log('copy')}
	on:init={(editor) => console.log(editor.detail)}
	on:commandKey={(obj) => console.log(obj.detail)}
	on:changeMode={(obj) => console.log(`change mode : ${obj.detail}`)}
	on:blur={() => (text = html(text))}
	width="100%"
	height="300px"
	lang="html"
	theme="chrome"
	value={text}
/>

<button on:click={() => (text = html(text))}> Click me </button>

<div>{@html text}</div>
