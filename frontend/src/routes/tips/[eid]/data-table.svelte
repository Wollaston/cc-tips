<script lang="ts">
	import { createTable, Render, Subscribe } from 'svelte-headless-table';
	import {
		addDataExport,
		addPagination,
		addSortBy,
		addTableFilter
	} from 'svelte-headless-table/plugins';
	import { readable } from 'svelte/store';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';

	import ArrowUpDown from 'lucide-svelte/icons/arrow-up-down';

	interface TippedDay {
		name: string;
		employee: string;
		role: string;
		net_tips: number;
		total_pay_for_night: number;
		hourly_pay_for_night: number;
		tipped_hour_for_night: number;
		duration: number;
		eid: number;
		date: string;
		created: string;
		modified: string;
	}

	export let tippedDays: TippedDay[];

	const table = createTable(readable(tippedDays), {
		page: addPagination({
			initialPageSize: 25
		}),
		sort: addSortBy(),
		filter: addTableFilter({
			fn: ({ filterValue, value }) => value.toLowerCase().includes(filterValue.toLowerCase())
		}),
		export: addDataExport({ format: 'csv' })
	});
	const columns = table.createColumns([
		table.column({
			accessor: 'name',
			header: 'Name'
		}),
		table.column({
			accessor: 'role',
			header: 'Role'
		}),
		table.column({
			accessor: 'date',
			header: 'Date',
			cell: ({ value }) => {
				const formatted = new Intl.DateTimeFormat('en-US', {
					weekday: 'long',
					year: 'numeric',
					month: 'long',
					day: 'numeric',
					timeZone: 'America/New_York'
				}).format(new Date(value + 'T00:00:00.000'));
				return formatted;
			}
		}),
		table.column({
			accessor: 'net_tips',
			header: 'Net Tips',
			cell: ({ value }) => {
				const formatted = new Intl.NumberFormat('en-US', {
					style: 'currency',
					currency: 'USD'
				}).format(value);
				return formatted;
			}
		}),
		table.column({
			accessor: 'total_pay_for_night',
			header: 'Total Pay',
			cell: ({ value }) => {
				const formatted = new Intl.NumberFormat('en-US', {
					style: 'currency',
					currency: 'USD'
				}).format(value);
				return formatted;
			}
		}),
		table.column({
			accessor: 'hourly_pay_for_night',
			header: 'Hourly Pay',
			cell: ({ value }) => {
				const formatted = new Intl.NumberFormat('en-US', {
					style: 'currency',
					currency: 'USD'
				}).format(value);
				return formatted;
			}
		})
	]);

	const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
		table.createViewModel(columns);

	const { filterValue } = pluginStates.filter;
	const { hasNextPage, hasPreviousPage, pageIndex } = pluginStates.page;
	export let { exportedData } = pluginStates.export;
</script>

<div class="rounded-md border">
	<div class="flex items-center py-4">
		<Input
			class="m-2 max-w-sm"
			placeholder="Filter table..."
			type="text"
			bind:value={$filterValue}
		/>
	</div>
	<Table.Root {...$tableAttrs}>
		<Table.Header>
			{#each $headerRows as headerRow}
				<Subscribe rowProps={headerRow.props()} rowAttrs={headerRow.attrs()}>
					<Table.Row>
						{#each headerRow.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
								<Table.Head {...attrs}>
									{#if cell.id == 'net_tips' || cell.id === 'total_pay_for_night' || cell.id === 'hourly_pay_for_night'}
										<Button variant="ghost" on:click={props.sort.toggle}>
											<Render of={cell.render()} />
											<ArrowUpDown class={'ml-2 h-4 w-4'} />
										</Button>
									{:else if cell.id === 'name' || cell.id === 'role' || cell.id === 'date'}
										<Button variant="ghost" on:click={props.sort.toggle}>
											<Render of={cell.render()} />
											<ArrowUpDown class={'ml-2 h-4 w-4'} />
										</Button>
									{:else}
										<Render of={cell.render()} />
									{/if}
								</Table.Head>
							</Subscribe>
						{/each}
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Header>
		<Table.Body {...$tableBodyAttrs}>
			{#each $pageRows as row (row.id)}
				<Subscribe rowAttrs={row.attrs()} let:rowAttrs>
					<Table.Row {...rowAttrs}>
						{#each row.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs>
								<Table.Cell {...attrs}>
									{#if cell.id === 'net_tips' || cell.id === 'total_pay_for_night' || cell.id === 'hourly_pay_for_night'}
										<div class="font-medium">
											<Render of={cell.render()} />
										</div>
									{:else if cell.id === 'name'}
										<div class="w-[150px] font-normal">
											<Render of={cell.render()} />
										</div>
									{:else}
										<Render of={cell.render()} />
									{/if}
								</Table.Cell>
							</Subscribe>
						{/each}
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Body>
	</Table.Root>
	<div class="m-2 flex items-center justify-end space-x-4 py-4">
		<Button
			variant="outline"
			size="sm"
			on:click={() => ($pageIndex = $pageIndex - 1)}
			disabled={!$hasPreviousPage}>Previous</Button
		>
		<Button
			variant="outline"
			size="sm"
			disabled={!$hasNextPage}
			on:click={() => ($pageIndex = $pageIndex + 1)}>Next</Button
		>
	</div>
</div>
