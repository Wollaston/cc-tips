<script lang="ts">
	import { File } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';

	import DataTable from './data-table.svelte';
	import type { Readable } from 'svelte/store';
	import SummaryCard from './SummaryCard.svelte';

	export let data;
	let exportedData: Readable<string>;

	async function generate_csv() {
		let res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/staff/csv`, {
			method: 'POST',
			body: $exportedData,
			headers: {
				'content-type': 'text/plain'
			}
		});

		let text = await res.text();
		console.log(text);

		var testLink = document.createElement('a');
		testLink.href = `${import.meta.env.VITE_BACKEND_URL}/${text}`;
		testLink.setAttribute('download', '');
		testLink.click();
	}
</script>

<div class="flex w-full flex-col">
	<main class="flex flex-1 flex-col gap-4 p-4 md:gap-8 md:p-8">
		<div class="grid gap-4 md:grid-cols-2 md:gap-8 lg:grid-cols-4">
			<Card.Root
				data-x-chunk-name="dashboard-01-chunk-0"
				data-x-chunk-description="A card showing the total revenue in USD and the percentage difference from last month."
			>
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Total Tips</Card.Title>
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">
						{new Intl.NumberFormat('en-US', {
							style: 'currency',
							currency: 'USD'
						}).format(data.summaryStats.net_tips_sum)}
					</div>
				</Card.Content>
			</Card.Root>
			<Card.Root
				data-x-chunk-name="dashboard-01-chunk-1"
				data-x-chunk-description="A card showing the total subscriptions and the percentage difference from last month."
			>
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Total Pay</Card.Title>
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">
						{new Intl.NumberFormat('en-US', {
							style: 'currency',
							currency: 'USD'
						}).format(data.summaryStats.total_pay)}
					</div>
				</Card.Content>
			</Card.Root>
			<Card.Root
				data-x-chunk-name="dashboard-01-chunk-2"
				data-x-chunk-description="A card showing the total sales and the percentage difference from last month."
			>
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Average Hourly Pay</Card.Title>
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">
						{new Intl.NumberFormat('en-US', {
							style: 'currency',
							currency: 'USD'
						}).format(data.summaryStats.average_hourly)}
					</div>
				</Card.Content>
			</Card.Root>
			<Card.Root
				data-x-chunk-name="dashboard-01-chunk-3"
				data-x-chunk-description="A card showing the total active users and the percentage difference from last hour."
			>
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Total Hours</Card.Title>
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">
						{new Intl.NumberFormat('en', {
							maximumFractionDigits: 2
						}).format(data.summaryStats.total_hours)}
					</div>
				</Card.Content>
			</Card.Root>
		</div>
		<div class="grid gap-4 md:gap-8 lg:grid-cols-2 xl:grid-cols-3">
			<Card.Root
				class="xl:col-span-2"
				data-x-chunk-name="dashboard-01-chunk-4"
				data-x-chunk-description="A card showing a table of recent transactions with a link to view all transactions."
			>
				<Card.Header class="flex flex-row items-center">
					<div class="grid gap-2">
						<Card.Title>{data.staffDetail.name}</Card.Title>
						<Card.Description>Calculated Tip Pool Data</Card.Description>
					</div>
					<Button on:click={() => generate_csv()} size="sm" variant="outline" class="ml-auto gap-1">
						<File class="h-3.5 w-3.5" />
						<span class="sr-only sm:not-sr-only sm:whitespace-nowrap"> Export </span>
					</Button>
				</Card.Header>
				<Card.Content>
					<DataTable bind:exportedData tippedDays={data.tippedDays} />
				</Card.Content>
			</Card.Root>
			<SummaryCard summary={data.staffDetail} />
		</div>
	</main>
</div>
