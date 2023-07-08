<script lang="ts">
	import './styles.scss';
	import ItalicSVG from './editorIcons/italic.svg?component';
	import BoldSVG from './editorIcons/bold.svg?component';
	import StrikeSVG from './editorIcons/strikethrough.svg?component';
	import UnderlineIcon from './editorIcons/underline.svg?component';
	import ClearFormattingSVG from './editorIcons/clear-formatting.svg?component';
	import Undo from './editorIcons/undo.svg?component';
	import Redo from './editorIcons/redo.svg?component';
	import NumberList from './editorIcons/number-list.svg?component';
	import BulletList from './editorIcons/bullet-list.svg?component';
	import ColorIcon from './editorIcons/color.svg?component';
	import Size from './editorIcons/size.svg?component';

	import StarterKit from '@tiptap/starter-kit'; // maybe debloat at one point
	import TextStyle from '@tiptap/extension-text-style';
	import FontFamily from '@tiptap/extension-font-family';
	import FontSize from 'tiptap-extension-font-size';
	import Image from '@tiptap/extension-image';
	import Underline from '@tiptap/extension-underline';
	import { Color } from '@tiptap/extension-color';
	import { Editor } from '@tiptap/core';
	import { onMount } from 'svelte';

	let element: HTMLDivElement;

	let editor: Editor;
	let content = `
              <h2>
                Hi there,
              </h2>
              <p>
                this is a <em>basic</em> example of <strong>tiptap</strong>. Sure, there are all kind of basic text styles you’d probably expect from a text editor. But wait until you see the lists:
              </p>
              <ul>
                <li>
                  That’s a bullet list with one …
                </li>
                <li>
                  … or two list items.
                </li>
              </ul>
              <p>
                Isn’t that great? And all of that is editable. But wait, there’s more. Let’s try a code block:
              </p>
              <pre><code class="language-css">body {
          display: none;
        }</code></pre>
              <p>
                I know, I know, this is impressive. It’s only the tip of the iceberg though. Give it a try and click a little bit around. Don’t forget to check the other examples too.
              </p>
              <blockquote>
                Wow, that’s amazing. Good work, boy! 👏
                <br />
                — Mom
              </blockquote>
            `;

	onMount(() => {
		editor = new Editor({
			element: element,
			extensions: [StarterKit, TextStyle, FontSize, Color, Image, FontFamily, Underline],
			content: content,
			onTransaction: () => {
				// force re-render so `editor.isActive` works as expected
				editor = editor;
			}
		});
	});

	function setActiveClassAttr(active: boolean) {
		return active ? 'bg-white' : '';
	}

	function setColor(e: Event) {
		let target = e.target as HTMLInputElement;
		editor.chain().focus().setColor(target.value).run();
	}
</script>

{#if editor}
	{editor.getHTML()}
	<div class="bg-slate-600">
		<div class="flex gap-5">
			<button
				on:click={() => editor.chain().focus().toggleBold().run()}
				disabled={!editor.can().chain().focus().toggleBold().run()}
				class={setActiveClassAttr(editor.isActive('bold'))}
			>
				<BoldSVG class="w-5 h-5" />
			</button>
			<button
				on:click={() => editor.chain().focus().toggleItalic().run()}
				disabled={!editor.can().chain().focus().toggleItalic().run()}
				class={editor.isActive('italic') ? 'is-active' : ''}
			>
				<ItalicSVG class="w-5 h-5" />
			</button>
			<button
				on:click={() => editor.chain().focus().toggleStrike().run()}
				disabled={!editor.can().chain().focus().toggleStrike().run()}
				class={editor.isActive('strike') ? 'is-active' : ''}
			>
				<StrikeSVG class="w-5 h-5" />
			</button>
			<button
				on:click={() => editor.chain().focus().toggleUnderline().run()}
				class={editor.isActive('underline') ? 'is-active' : ''}
			>
				<UnderlineIcon class="w-5 h-5" />
			</button>
			<button on:click={() => editor.chain().focus().unsetAllMarks().run()}>
				<ClearFormattingSVG class="w-5 h-5" /></button
			>

			<button
				on:click={() => editor.chain().focus().toggleBulletList().run()}
				class={editor.isActive('bulletList') ? 'is-active' : ''}
			>
				<BulletList class="w-5 h-5" />
			</button>
			<button
				on:click={() => editor.chain().focus().toggleOrderedList().run()}
				class={editor.isActive('orderedList') ? 'is-active' : ''}
			>
				<NumberList class="w-5 h-5" />
			</button>

			<button
				on:click={() => editor.chain().focus().undo().run()}
				disabled={!editor.can().chain().focus().undo().run()}
			>
				<Undo class="w-5 h-5" />
			</button>
			<button
				on:click={() => editor.chain().focus().redo().run()}
				disabled={!editor.can().chain().focus().redo().run()}
			>
				<Redo class="w-5 h-5" />
			</button>
			<button
				on:click={() => editor.chain().focus().setFontSize('20px').run()}
				class={editor.isActive('textStyle', { fontSize: '20px' }) ? 'is-active' : ''}
			>
				<Size class="w-5 h-5" />
			</button>
			<div class="flex content-center">
				<input type="color" id="color-input" on:input={setColor} class="collapse h-0 w-0 p-0 m-0" />
				<label for="color-input">
					<ColorIcon class="w-5 h-5" />
				</label>
			</div>
			<button
				on:click={() => {
					{
						const url = window.prompt('URL');
						if (url) {
							editor.chain().focus().setImage({ src: url }).run();
						}
					}
				}}
			/>
		</div>
	</div>
{/if}
<div class="bg-white text-cyan-400" bind:this={element} />
