<script lang="ts">
	import { goto } from '$app/navigation';
	import { PUBLIC_BACKEND_URL } from '$env/static/public';
	import { inputState, type Input, checkEmpty, setInputStyling } from '$lib/types/inputFields';

	// I tried to make this part a little more dynamic but in the end.
	// I was not able to get the reactivity working.

	$: emailInput = { state: inputState.Idle, value: '' } as Input;
	$: passwordInput = { state: inputState.Idle, value: '' } as Input;
	$: secondPasswordInput = { state: inputState.Idle, value: '' } as Input;
	$: showMandatory = false;
	$: showPassword = false;
	$: showEmailFormat = false;

	function checkInputs(): boolean {
		let emailResultTuple = checkEmpty(emailInput);
		emailInput = emailResultTuple[0];
		let passwordResultTuple = checkEmpty(passwordInput);
		passwordInput = passwordResultTuple[0];
		let secondPasswordresultTuple = checkEmpty(secondPasswordInput);
		secondPasswordInput = secondPasswordresultTuple[0];

		showMandatory = emailResultTuple[1] || passwordResultTuple[1] || secondPasswordresultTuple[1];
		if (showMandatory) {
			return false;
		}

		var mailformat = /^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/;
		showEmailFormat = !emailInput.value.match(mailformat);
		if (showEmailFormat) {
			emailInput.state = inputState.Error;
			emailInput = emailInput;
			return false;
		}

		showPassword = passwordInput.value !== secondPasswordInput.value;

		if (showPassword) {
			secondPasswordInput.state = inputState.Error;
			secondPasswordInput = secondPasswordInput;
			passwordInput.state = inputState.Error;
			passwordInput = passwordInput;
			return false;
		}
		return true;
	}

	async function sign_up() {
		if (!checkInputs()) {
			return;
		}

		const res = await fetch(PUBLIC_BACKEND_URL + '/sign_up', {
			method: 'POST',
			credentials: 'include',
			body: JSON.stringify({ email: emailInput.value, password: passwordInput.value }),
			headers: {
				'Content-Type': 'application/json'
			}
		});
		if (res.status == 201) {
			goto('/login');
		}
	}
</script>

<form class="card p-4 min-w-min">
	<header class="card-header text-center max-w-sm">
		Sign up with the e-mail adress you want to receive your mail in the future!
	</header>
	<label class="label my-3">
		<span class="pl-2">Email:</span>
		<input
			bind:value={emailInput.value}
			type="email"
			placeholder="your.email@adress.com"
			required
			class="input rounded-full {setInputStyling(emailInput.state)}"
			on:focus={() => (emailInput.state = inputState.Idle)}
		/>
	</label>
	<label class="label my-3">
		<span class="pl-2">Password:</span>
		<input
			bind:value={passwordInput.value}
			type="password"
			placeholder="Enter Password Here"
			required
			class="input rounded-full {setInputStyling(passwordInput.state)}"
			on:focus={() => (passwordInput.state = inputState.Idle)}
		/>
	</label>
	<label class="label my-3">
		<span class="pl-2">Enter Password Again:</span>
		<input
			bind:value={secondPasswordInput.value}
			type="password"
			placeholder="Enter Password Again"
			required
			class="input rounded-full {setInputStyling(secondPasswordInput.state)}"
			on:focus={() => (secondPasswordInput.state = inputState.Idle)}
		/>
	</label>

	<header class="card-header text-center max-w-sm text-red-400">
		{#if showMandatory}
			Please fill all fields.
		{:else if showPassword}
			Please make sure that the two passwords are the same.
		{:else if showEmailFormat}
			Invalid E-Mail format.
		{/if}
	</header>

	<div class="mt-6 flex flex-col items-center">
		<button
			on:click={sign_up}
			type="button"
			class="btn variant-ghost-surface rounded-full min-w-min"
		>
			Sign Up!
		</button>
	</div>
</form>
