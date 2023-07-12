import { writable } from 'svelte/store';

export enum LoginState{
    Logged,
    Not,
    FirstLogin,
}

export const loginStore = writable(LoginState.FirstLogin);
export const userEmail = writable("")
