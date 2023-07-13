<script lang="ts">
	import { Stepper, Step } from '@skeletonlabs/skeleton';
	import { SlideToggle } from '@skeletonlabs/skeleton';
	import HtmlEditor from '$lib/components/HtmlEditor.svelte';
	import { goto } from '$app/navigation';
	import { PUBLIC_BACKEND_URL } from '$env/static/public';
	import { LoginState, loginStore } from '$lib/stores/loginState';
	import TextEditor from '$lib/components/TextEditor/TextEditor.svelte';

	let date_step_lock = false;

	function check_date() {
		date_step_lock = Date.parse(date) > Date.now();
	}

	let date: string = '';
	let subject: string = '';
	let is_html: boolean = false;
	let body_text: string = `<p>This is an example text.</p>
	<p><strong>You</strong> <em>can</em> <s>apply</s> <u>basic</u> <strong><em><s><u>styles.</u></s></em></strong></p>
	<p>You <span style="font-size: 11px">can</span> change <span style="font-size: 25px">the</span> text size.</p>
	<p>You <span style="color: #1a5fb4">can </span>change <span style="color: #c01c28">the
		</span> text c<span style="color: #986a44">olor.</span></p>
	<p>Image import is work in progress.</p>
	<p></p>
	<p>Have fun.</p>`;
	let body_html: string = `
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

	async function create_email() {
		try {
			const res = await fetch(PUBLIC_BACKEND_URL + '/email', {
				method: 'POST',
				credentials: 'include',
				body: JSON.stringify({
					is_html: is_html,
					subject: subject,
					date: date,
					body: is_html ? body_html : body_text
				}),
				headers: {
					'Content-Type': 'application/json'
				}
			});

			if (res.status == 201) {
				goto('/emails');
			} else if (res.status == 401) {
				$loginStore = LoginState.Not;
				goto('/');
			}
		} catch (error) {
			console.error(error);
		}
	}
</script>

<div class="card p-4 w-[100%] min-w-[300px]">
	<Stepper on:complete={create_email}>
		<Step>
			<svelte:fragment slot="header">
				<div class="text-center">Welcome to the mail builder!</div>
			</svelte:fragment>
			<svelte:fragment slot="navigation">
				<button class="btn variant-ghost-error" on:click={() => goto('/emails')}>Abort</button>
			</svelte:fragment>
			<div class="text-center">By following this, you will be able to create your future!</div>
		</Step>
		<Step locked={!(date !== '' && date_step_lock)}>
			<svelte:fragment slot="header">
				<div class="text-center">Select the date that we should send this mail</div>
			</svelte:fragment>
			<div class="flex justify-center p-10">
				<input type="date" bind:value={date} on:change={check_date} class="input max-w-sm" />
			</div>
			{#if date !== '' && !date_step_lock}
				<div class="text-center">
					Please select a date in the future. I haven't invented time machine yet. 😔
				</div>
			{/if}
		</Step>
		<Step locked={subject === ''}>
			<svelte:fragment slot="header">
				<div class="text-center">Enter a subject for this email</div>
			</svelte:fragment>
			<div class="flex justify-center p-10">
				<input type="text" bind:value={subject} class="input" />
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
				<div class="text-center">Edit the Email!</div>
			</svelte:fragment>
			{#if is_html}
				<div class="text-center"><HtmlEditor bind:text={body_html} /></div>
			{:else}
				<TextEditor initialContent={body_text} bind:exportContent={body_text} />
			{/if}
		</Step>
		<!-- ... -->
	</Stepper>
</div>
