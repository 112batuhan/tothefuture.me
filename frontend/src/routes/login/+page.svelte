<script lang="ts">
	import { goto } from '$app/navigation';
	import { PUBLIC_BACKEND_URL } from '$env/static/public';
	import { LoginState, loginStore, userEmail } from '$lib/stores/loginState';
	import { inputState, type Input, checkEmpty, setInputStyling } from '$lib/types/inputFields';

	$: emailInput = { state: inputState.Idle, value: '' } as Input;
	$: passwordInput = { state: inputState.Idle, value: '' } as Input;

	$: showMandatory = false;

	$: showEmailFormat = false;

	function checkInputs(): boolean {
		let emailResultTuple = checkEmpty(emailInput);
		emailInput = emailResultTuple[0];
		let passwordResultTuple = checkEmpty(passwordInput);
		passwordInput = passwordResultTuple[0];

		showMandatory = emailResultTuple[1] || passwordResultTuple[1];
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

		return true;
	}

	async function login() {
		if (!checkInputs()) {
			return;
		}

		const res = await fetch(PUBLIC_BACKEND_URL + '/login', {
			method: 'POST',
			credentials: 'include',
			body: JSON.stringify({ email: emailInput.value, password: passwordInput.value }),
			headers: {
				'Content-Type': 'application/json'
			}
		});
		if (res.status == 201) {
			$loginStore = LoginState.Logged;
			let response_json = await res.json();
			$userEmail = response_json.email;
			goto('/emails');
		} else {
			$loginStore = LoginState.Not;
		}
	}
</script>

<div class="card p-4 min-w-min">
	<header class="card-header text-center">Login to shape your future!</header>
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

	<header class="card-header text-center max-w-sm text-red-400">
		{#if showMandatory}
			Please fill all fields.
		{:else if showEmailFormat}
			Invalid E-Mail format.
		{/if}
	</header>
	<div class="mt-6 flex flex-col items-center">
		<button type="button" on:click={login} class="btn variant-ghost-surface rounded-full min-w-min">
			Login!
		</button>
	</div>
</div>
