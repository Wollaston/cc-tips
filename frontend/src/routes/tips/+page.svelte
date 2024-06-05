<script lang="ts">
	import File from 'lucide-svelte/icons/file';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import DataTable from './data-table.svelte';
	import { writable, type Readable } from 'svelte/store';
	import type { ActionData, PageData } from './$types';

	interface TippedDay {
		name: string;
		employee: string;
		role: string;
		net_tips: number;
		total_pay_for_night: number;
		hourly_pay_for_night: number;
		duration: number;
		eid: number;
		date: string;
		created: string;
		modified: string;
	}

	export let data: PageData;
	export let form: ActionData;

	let exportedData: Readable<string>;

	const tippedDays = writable<TippedDay[]>([]);
	$: $tippedDays = form?.tippedDays ?? [];

	async function generate_csv() {
		console.log($exportedData);
	}
</script>

<main class="grid flex-1 items-start gap-4 p-4 sm:px-6 sm:py-0 md:gap-8">
	<Card.Root
		data-x-chunk-name="dashboard-06-chunk-1"
		data-x-chunk-description="A list of products in a table with actions. Each row has an image, name, status, price, total sales, created at and actions."
	>
		<div class="flex items-center">
			<Card.Header>
				<Card.Title>Tips</Card.Title>
				<Card.Description>Tips calculated and available in the Tip Pool database.</Card.Description>
			</Card.Header>
			<div class="ml-auto flex items-center gap-2">
				<Button on:click={() => generate_csv()} size="sm" variant="outline" class="m-2 h-7 gap-1">
					<File class="h-3.5 w-3.5" />
					<span class="sr-only sm:not-sr-only sm:whitespace-nowrap"> Export </span>
				</Button>
			</div>
		</div>
		<Card.Content>
			<DataTable bind:exportedData data={data.tipsForm} {tippedDays} />
		</Card.Content>
		<Card.Footer>
			<div class="text-xs text-muted-foreground">
				Data is limited to tips imported into the Tip Pool database
			</div>
		</Card.Footer>
	</Card.Root>
</main>
