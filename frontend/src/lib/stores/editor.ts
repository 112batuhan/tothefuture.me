import { writable } from 'svelte/store';

export interface EmailData {
	id: number;
	is_hidden: boolean;
	is_html: boolean;
	is_sent: boolean;
	owner: number;
	send_date: string;
	subject: string;
	body: string;
}

export const editorData = writable<EmailData>({
	id: 0,
	is_hidden: false,
	is_html: false,
	is_sent: false,
	owner: 0,
	send_date: '',
	subject: '',
	body: ''
});
