<script lang="ts">
	import { createTable, Render, Subscribe, createRender } from 'svelte-headless-table';
	import * as Table from '$lib/components/ui/table/index.js';
	import {
		addPagination,
		addSortBy,
		addTableFilter,
		addDataExport
	} from 'svelte-headless-table/plugins';
	import DataTableActions from './data-table-actions.svelte';
	import { Input } from '$lib/components/ui/input';

	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';

	import ArrowUpDown from 'lucide-svelte/icons/arrow-up-down';
	import type { Writable } from 'svelte/store';
	import type { Infer, SuperValidated } from 'sveltekit-superforms';
	import type { FormSchema as CommissionsSchema } from './commissionsSchema';
	import type { Commission, StaffNameEid, WineNamePrice } from '$lib/types';
	import { FileDown } from 'lucide-svelte';
	import CommissionsForm from './CommissionsForm.svelte';

	export let commissions: Writable<Commission[]>;
	export let commissionsData: SuperValidated<Infer<CommissionsSchema>>;
	export let wines: WineNamePrice[];
	export let staff: StaffNameEid[];

	const table = createTable(commissions, {
		page: addPagination(),
		sort: addSortBy(),
		filter: addTableFilter({
			fn: ({ filterValue, value }) => value.toLowerCase().includes(filterValue.toLowerCase())
		}),
		export: addDataExport({ format: 'csv' })
	});

	const columns = table.createColumns([
		table.column({
			accessor: 'name',
			header: 'Name',
			plugins: {
				sort: {
					disable: false
				},
				filter: {
					exclude: false
				}
			}
		}),
		table.column({
			accessor: 'wine',
			header: 'Wine',
			plugins: {
				sort: {
					disable: false
				},
				filter: {
					exclude: false
				}
			}
		}),
		table.column({
			accessor: 'amount',
			header: 'Amount',
			plugins: {
				sort: {
					disable: false
				},
				filter: {
					exclude: true
				}
			}
		}),
		table.column({
			accessor: 'date',
			header: 'Date',
			cell: ({ value }) => {
				const formatted = new Intl.DateTimeFormat('en-US', {
					year: 'numeric',
					month: 'long',
					day: 'numeric',
					timeZone: 'America/New_York'
				}).format(new Date(value));
				return formatted;
			},
			plugins: {
				sort: {
					disable: false
				},
				filter: {
					exclude: true
				}
			}
		}),
		table.column({
			accessor: ({ product_id }) => product_id,
			header: '',
			cell: ({ value }) => {
				return createRender(DataTableActions, { product_id: value });
			},
			plugins: {
				sort: {
					disable: true
				},
				filter: {
					exclude: true
				}
			}
		})
	]);

	const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
		table.createViewModel(columns);

	const { hasNextPage, hasPreviousPage, pageIndex } = pluginStates.page;
	const { filterValue } = pluginStates.filter;
	const { exportedData } = pluginStates.export;

	async function generate_csv() {
		let res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/tips/csv`, {
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

<Card.Root>
	<div class="flex">
		<Card.Header class="px-7">
			<Card.Title>Commissions Data</Card.Title>
			<Card.Description>A list of commissions in the database</Card.Description>
		</Card.Header>
		<div class="m-2 ml-auto flex items-center gap-2 p-2">
			<CommissionsForm {commissionsData} {wines} {staff} />
			<Button on:click={() => generate_csv()} size="sm" variant="outline" class="gap-1 text-sm">
				<FileDown class="h-3.5 w-3.5" />
				<span>Export</span>
			</Button>
		</div>
	</div>
	<Card.Content>
		<div>
			<div class="flex items-center py-4">
				<Input
					class="max-w-sm"
					placeholder="Filter by name..."
					type="text"
					bind:value={$filterValue}
				/>
			</div>
			<Table.Root {...$tableAttrs}>
				<Table.Header>
					{#each $headerRows as headerRow (headerRow.id)}
						<Subscribe rowAttrs={headerRow.attrs()} rowProps={headerRow.props()}
							><Table.Row>
								{#each headerRow.cells as cell (cell.id)}
									<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
										<Table.Head {...attrs}>
											{#if cell.id === 'name'}
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
							<Table.Row {...rowAttrs} class="bg-accent font-medium">
								{#each row.cells as cell (cell.id)}
									<Subscribe attrs={cell.attrs()} let:attrs>
										<Table.Cell {...attrs} class="hidden sm:table-cell">
											<Render of={cell.render()} />
										</Table.Cell>
									</Subscribe>
								{/each}
							</Table.Row>
						</Subscribe>
					{/each}
				</Table.Body>
			</Table.Root>
			<div class="flex items-center justify-end space-x-4 py-4">
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
	</Card.Content>
</Card.Root>
