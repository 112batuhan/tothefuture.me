<script>
	//@ts-nocheck
	import './styles.scss';

	import StarterKit from '@tiptap/starter-kit'; // maybe debloat at one point
	import TextStyle from '@tiptap/extension-text-style';
	import FontFamily from '@tiptap/extension-font-family';
	import FontSize from 'tiptap-extension-font-size';
	import Image from '@tiptap/extension-image';
	import Underline from '@tiptap/extension-underline';
	import { Color } from '@tiptap/extension-color';
	import { Editor } from '@tiptap/core';
	import { onMount } from 'svelte';

	let element;
	let editor;
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
</script>

{#if editor}
	{editor.getHTML()}
	<div class="bg-slate-600">
		<div>
			<button
				on:click={() => console.log && editor.chain().focus().toggleBold().run()}
				disabled={!editor.can().chain().focus().toggleBold().run()}
				class={editor.isActive('bold') ? 'is-active' : ''}
			>
				bold
			</button>
			<button
				on:click={() => editor.chain().focus().toggleItalic().run()}
				disabled={!editor.can().chain().focus().toggleItalic().run()}
				class={editor.isActive('italic') ? 'is-active' : ''}
			>
				italic
			</button>
			<button
				on:click={() => editor.chain().focus().toggleStrike().run()}
				disabled={!editor.can().chain().focus().toggleStrike().run()}
				class={editor.isActive('strike') ? 'is-active' : ''}
			>
				strike
			</button>
			<button
				on:click={() => editor.chain().focus().toggleCode().run()}
				disabled={!editor.can().chain().focus().toggleCode().run()}
				class={editor.isActive('code') ? 'is-active' : ''}
			>
				code
			</button>
			<button on:click={() => editor.chain().focus().unsetAllMarks().run()}> clear marks </button>
			<button
				on:click={() => editor.chain().focus().setParagraph().run()}
				class={editor.isActive('paragraph') ? 'is-active' : ''}
			>
				paragraph
			</button>

			<button
				on:click={() => editor.chain().focus().toggleBulletList().run()}
				class={editor.isActive('bulletList') ? 'is-active' : ''}
			>
				bullet list
			</button>
			<button
				on:click={() => editor.chain().focus().toggleOrderedList().run()}
				class={editor.isActive('orderedList') ? 'is-active' : ''}
			>
				ordered list
			</button>
			<button
				on:click={() => editor.chain().focus().toggleCodeBlock().run()}
				class={editor.isActive('codeBlock') ? 'is-active' : ''}
			>
				code block
			</button>
			<button
				on:click={() => editor.chain().focus().toggleBlockquote().run()}
				class={editor.isActive('blockquote') ? 'is-active' : ''}
			>
				blockquote
			</button>
			<button on:click={() => editor.chain().focus().setHorizontalRule().run()}>
				horizontal rule
			</button>
			<button on:click={() => editor.chain().focus().setHardBreak().run()}> hard break </button>
			<button
				on:click={() => editor.chain().focus().undo().run()}
				disabled={!editor.can().chain().focus().undo().run()}
			>
				undo
			</button>
			<button
				on:click={() => editor.chain().focus().redo().run()}
				disabled={!editor.can().chain().focus().redo().run()}
			>
				redo
			</button>
			<button
				on:click={() => editor.chain().focus().setFontSize('20px').run()}
				class={editor.isActive('textStyle', { fontSize: '20px' }) ? 'is-active' : ''}
			>
				resize
			</button>
			<input
				type="color"
				on:input={(e) => editor.chain().focus().setColor(e.target.value).run()}
				class={editor.isActive('textStyle').color ? 'is-active' : ''}
			/>
			<button
				on:click={() => {
					{
						const url = window.prompt('URL');
						if (url) {
							editor.chain().focus().setImage({ src: url }).run();
						}
					}
				}}
			>
				setImage
			</button>
			<button
				on:click={() => editor.chain().focus().setFontFamily('Comic Sans MS, Comic Sans').run()}
				class={editor.isActive('textStyle', { fontFamily: 'Comic Sans MS, Comic Sans' })}
			>
				Comic Sans
			</button>
			<button
				on:click={() => editor.chain().focus().setFontFamily('serif').run()}
				class={editor.isActive('textStyle', { fontFamily: 'serif' })}
			>
				serif
			</button>
			<button
				on:click={() => editor.chain().focus().toggleUnderline().run()}
				class={editor.isActive('underline') ? 'is-active' : ''}
			>
				toggleUnderline
			</button>
		</div>
	</div>
{/if}
<div class="bg-white text-cyan-400" bind:this={element} />
