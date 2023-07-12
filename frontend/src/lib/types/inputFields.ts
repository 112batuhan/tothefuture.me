export enum inputState {
	Idle,
	Warning,
	Error
}

export interface Input {
	state: inputState;
	value: string;
}

export function setInputStyling(state: inputState): string {
	switch (state) {
		case inputState.Warning:
			return 'input-warning';
		case inputState.Error:
			return 'input-error';
	}
	return '';
}

export function checkEmpty(inputField: Input): [Input, boolean] {
	let condition = inputField.value === '';
	if (condition) {
		inputField.state = inputState.Error;
	}
	// To trigger reactivity, I return and set the Input object
	return [inputField, condition];
}
