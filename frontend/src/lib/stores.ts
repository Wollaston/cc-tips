import { writable, type Writable } from 'svelte/store';

const staffStore: Writable<StaffMember[]> = writable<StaffMember[]>([]);

const memberDetailStore: Writable<MemberDetailResponse> = writable<MemberDetailResponse>();

export { staffStore, memberDetailStore };
