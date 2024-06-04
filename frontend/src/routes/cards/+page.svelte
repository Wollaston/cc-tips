<script lang="ts">
	import StaffTable from './StaffTable.svelte';
	import type { PageData } from './$types';
	import { writable } from 'svelte/store';
	import TipsSummary from './TipsSummary.svelte';

	export let data: PageData;

	const staff = writable(data.staff);
	$: $staff = data.staff;
</script>

<main
	class="grid flex-1 items-start gap-4 p-2 sm:px-6 sm:py-0 md:gap-8 lg:grid-cols-3 xl:grid-cols-3"
>
	<div class="grid auto-rows-max items-start gap-4 md:gap-8 lg:col-span-2">
		<StaffTable {staff} registerData={data.registerForm} importData={data.importForm} />
	</div>
	<TipsSummary />
</main>
