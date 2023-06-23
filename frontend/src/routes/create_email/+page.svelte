<script lang="ts">
	import { Stepper, Step } from '@skeletonlabs/skeleton';
	import { SlideToggle } from '@skeletonlabs/skeleton';
	import HtmlEditor from '$lib/components/HtmlEditor.svelte';
	import { goto } from '$app/navigation';
	import { PUBLIC_BACKEND_URL } from '$env/static/public';

	let date_step_lock = false;

	function check_date() {
		date_step_lock = Date.parse(date) > Date.now();
	}

	let date: string = '';
	let subject: string = '';
	let is_html: boolean = false;
	let body_text: string = '';
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
			const res = await fetch(PUBLIC_BACKEND_URL + '/create_email', {
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

			if (res.ok) {
				goto('/emails');
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
			<div class="text-center"><HtmlEditor bind:text={body_html} /></div>
		</Step>
		<!-- ... -->
	</Stepper>
</div>
