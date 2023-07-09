<script lang="ts">
	import ItalicSVG from './editorIcons/italic.svg?component';
	import BoldSVG from './editorIcons/bold.svg?component';
	import StrikeSVG from './editorIcons/strikethrough.svg?component';
	import UnderlineIcon from './editorIcons/underline.svg?component';
	import ClearFormattingSVG from './editorIcons/clear-formatting.svg?component';
	import UndoSVG from './editorIcons/undo.svg?component';
	import Redo from './editorIcons/redo.svg?component';
	import ColorSVG from './editorIcons/color.svg?component';
	import SizeSVG from './editorIcons/size.svg?component';
	import ImageSVG from './editorIcons/image.svg?component';

	import StarterKit from '@tiptap/starter-kit'; // maybe debloat at one point
	import TextStyle from '@tiptap/extension-text-style';
	import FontFamily from '@tiptap/extension-font-family';
	import FontSize from 'tiptap-extension-font-size';
	import Image from '@tiptap/extension-image';
	import Underline from '@tiptap/extension-underline';
	import { Color } from '@tiptap/extension-color';
	import { Editor } from '@tiptap/core';
	import { onMount } from 'svelte';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';

	let element: HTMLDivElement;

	let editor: Editor;
	let content = `<p>this is a test</p><p>this is a test for all things</p><p></p><p>a
		lon<strong>g l inea l<u>ong l</u></strong><u>inea long</u><em><u> line</u>a l
		ong line</em>a long lin&lt; s&gt;ea long lin<strong><s>ea l</s>ong linea <
		u>lon</u></strong></p><p><strong><u>g li</u></strong><u>nea lon</u>g l
		ine a lo<span style="font-size: 20px">ng linea long linea l</span>
		ong li<span>nea long li</span>nea long linea long l</p><p>inea lon
		g linea long linea long linea long linea long l inea long line
		a long linea long line</p><p>a long linea long linea long line
		a long linea long linea long linea long linea long linea lo
		ng linea l</p><p>ong linea long linea long linea long linea
		long linea long l inea long linea l</p><p>ong linea lon
		g linea long linea long linea long linea long linea
		long linea long l in</p><p>ea long linea long linea
		long linea long linea long line</p><p></p><p></p>`;

	onMount(() => {
		editor = new Editor({
			element: element,
			extensions: [StarterKit, TextStyle, FontSize, Color, Image, FontFamily, Underline],
			content: content,
			onTransaction: () => {
				setTextcolorInputValue();
				// force re-render so `editor.isActive` works as expected
				editor = editor;
			},
			onSelectionUpdate() {
				setTextSizeInputValue();
				setTextcolorInputValue();
			}
		});
	});

	const SVGStyle = 'w-5 h-5 m-2';

	const baseStyle = 'rounded-full fill-white stroke-white ';
	const activeClass = baseStyle + 'variant-filled-primary';
	const pasiveClass = baseStyle + 'variant-ghost-primary';
	function setActiveClassAttr(active: boolean) {
		return active ? activeClass : pasiveClass;
	}

	function setColor(e: Event) {
		let target = e.target as HTMLInputElement;
		editor.chain().focus().setColor(target.value).run();
	}

	function setSize(e: Event) {
		let target = e.target as HTMLInputElement;
		editor
			.chain()
			.focus()
			.setFontSize(target.value + 'px')
			.run();
	}

	let textSizeInputValue = 16;

	function setTextSizeInputValue() {
		let rawValue = editor.getAttributes('textStyle').fontSize;
		if (!rawValue) {
			textSizeInputValue = 16;
		} else {
			rawValue = rawValue.replace('px', '');
			textSizeInputValue = Number(rawValue);
		}
	}

	let textColorInputValue = '#000000';
	let colorStyling = 'fill: ' + textColorInputValue + ';';
	function setTextcolorInputValue() {
		let rawValue = editor.getAttributes('textStyle').color;
		if (!rawValue) {
			textColorInputValue = '#000000';
		} else {
			textColorInputValue = rawValue;
		}
		colorStyling = 'fill: ' + textColorInputValue + ';';
	}

	const textSizePopup: PopupSettings = {
		event: 'click',
		target: 'text-size-popup',
		placement: 'bottom'
	};
	const imgeUrlPopup: PopupSettings = {
		event: 'click',
		target: 'image-url-popup',
		placement: 'bottom'
	};
</script>

{#if editor}
	<div class="card variant-soft-surface flex gap-7 p-3 w-full flex-wrap">
		<div class="card variant-soft-surface rounded-full flex gap-2 justify-center">
			<button
				on:click={() => editor.chain().focus().toggleBold().run()}
				disabled={!editor.can().chain().focus().toggleBold().run()}
				class={setActiveClassAttr(editor.isActive('bold'))}
			>
				<BoldSVG class={SVGStyle} />
			</button>
			<button
				on:click={() => editor.chain().focus().toggleItalic().run()}
				disabled={!editor.can().chain().focus().toggleItalic().run()}
				class={setActiveClassAttr(editor.isActive('italic'))}
			>
				<ItalicSVG class={SVGStyle} />
			</button>
			<button
				on:click={() => editor.chain().focus().toggleStrike().run()}
				disabled={!editor.can().chain().focus().toggleStrike().run()}
				class={setActiveClassAttr(editor.isActive('strike'))}
			>
				<StrikeSVG class={SVGStyle} />
			</button>
			<button
				on:click={() => editor.chain().focus().toggleUnderline().run()}
				class={setActiveClassAttr(editor.isActive('underline'))}
			>
				<UnderlineIcon class={SVGStyle} />
			</button>
		</div>

		<div class="card variant-soft-surface rounded-full flex justify-center gap-2">
			<button use:popup={textSizePopup} class={pasiveClass}>
				<SizeSVG class={SVGStyle} />
			</button>
			<div class="flex content-center">
				<input
					type="color"
					id="color-input"
					on:input={setColor}
					bind:value={textColorInputValue}
					class="collapse h-0 w-0 p-0 m-0"
				/>
				<label for="color-input" class={pasiveClass}>
					<ColorSVG bind:style={colorStyling} class={SVGStyle} />
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
				class={pasiveClass}
			>
				<ImageSVG class={SVGStyle} />
			</button>
		</div>
		<div class="card variant-soft-surface rounded-full flex gap-2 justify-center">
			<button
				on:click={() => editor.chain().focus().undo().run()}
				disabled={!editor.can().chain().focus().undo().run()}
				class={pasiveClass}
			>
				<UndoSVG class={SVGStyle} />
			</button>
			<button
				on:click={() => editor.chain().focus().redo().run()}
				disabled={!editor.can().chain().focus().redo().run()}
				class={pasiveClass}
			>
				<Redo class={SVGStyle} />
			</button>
		</div>
		<button
			on:click={() => editor.chain().focus().unsetAllMarks().clearNodes().run()}
			disabled={!editor.can().chain().focus().unsetAllMarks().clearNodes().run()}
			class={pasiveClass}
		>
			<ClearFormattingSVG class={SVGStyle} />
		</button>
	</div>
{/if}
<div class="w-full overflow-auto bg-[#c2a6f5]" style="height: 550px;">
	<div class="text-black w-full" bind:this={element} />
	{#if editor}
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div class="w-full h-full" on:click={() => editor.chain().focus().run()} />
	{/if}
</div>
<!-- Popup elements -->
<div class="card p-4 shadow-xl" data-popup="text-size-popup">
	<input
		type="number"
		bind:value={textSizeInputValue}
		class="w-16 text-black"
		on:change={setSize}
	/>
</div>

<!-- Image popup -->
<div class="card p-4 shadow-xl" data-popup="image-url-popup">
	<input
		type="number"
		bind:value={textSizeInputValue}
		class="w-16 text-black"
		on:change={setSize}
	/>
</div>
