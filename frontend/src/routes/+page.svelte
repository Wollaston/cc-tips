<script lang="ts">
	import type { ActionData, PageData } from './$types.js';
	import CalculatorUploadForm from './CalculatorUploadForm.svelte';
	import SummaryCard from './SummaryCard.svelte';
	import TipsTable from './TipsTable.svelte';
	export let data: PageData;
	export let form: ActionData;

	$: template_link = `${import.meta.env.VITE_BACKEND_URL}/public/downloads/${form?.calculationsResponse?.template_link}`;
	$: calculations_link = `${import.meta.env.VITE_BACKEND_URL}/public/downloads/${form?.calculationsResponse?.calculations_link}`;
</script>

<div
	class="grid flex-1 items-start gap-4 p-2 sm:px-6 sm:py-0 md:gap-8 lg:grid-cols-3 xl:grid-cols-3"
>
	<div>
		<CalculatorUploadForm data={data.form} />
		<SummaryCard
			posted={form?.form.posted ?? false}
			average_net_hourly_pay={form?.calculationsResponse?.summary.average_net_hourly_pay ?? 0}
			total_tips={form?.calculationsResponse?.summary.total_tips ?? 0}
		/>
	</div>
	<div class="xl:col-span-2">
		<TipsTable
			bind:template_link
			bind:calculations_link
			valid={form?.form.valid ?? false}
			tips={form?.calculationsResponse?.tips ?? []}
		/>
	</div>
</div>
