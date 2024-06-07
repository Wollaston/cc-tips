<script lang="ts">
	import type { PageData } from './$types';
	import { writable } from 'svelte/store';
	import CommissionsTable from './CommissionsTable.svelte';
	import CommissionsSummary from './CommissionsSummary.svelte';

	export let data: PageData;

	const commissions = writable(data.commissions);
	$: $commissions = data.commissions;
</script>

<main
	class="grid flex-1 items-start gap-4 p-2 sm:px-6 sm:py-0 md:gap-8 lg:grid-cols-3 xl:grid-cols-3"
>
	<div class="grid auto-rows-max items-start gap-4 md:gap-8 lg:col-span-2">
		<CommissionsTable
			{commissions}
			commissionsData={data.commissionsForm}
			wines={data.wines}
			staff={data.staff}
		/>
	</div>
	<CommissionsSummary />
</main>
