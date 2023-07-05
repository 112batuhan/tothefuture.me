<script lang="ts">
	import type { Editor } from '@tiptap/core';
	export let editor: Editor;
	export let svg: String;
</script>

<button
	on:click={() => editor.chain().focus().toggleBold().run()}
	disabled={!editor.can().chain().focus().toggleBold().run()}
	class={editor.isActive('bold') ? 'is-active' : ''}
>
	<img src={svg} class="w-3 h-3" />
</button>
