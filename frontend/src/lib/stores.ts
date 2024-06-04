import { writable, type Writable } from 'svelte/store';

export const staffStore: Writable<StaffMember[]> = writable<StaffMember[]>([]);
