<script lang="ts">
	import { createTable, Render, Subscribe, createRender } from 'svelte-headless-table';
	import {
		addDataExport,
		addPagination,
		addSortBy,
		addTableFilter
	} from 'svelte-headless-table/plugins';
	import { type Writable } from 'svelte/store';
	import * as Table from '$lib/components/ui/table';
	import DataTableActions from './data-table-actions.svelte';
	import { Input } from '$lib/components/ui/input';
	import CalendarIcon from 'svelte-radix/Calendar.svelte';
	import {
		CalendarDate,
		DateFormatter,
		type DateValue,
		getLocalTimeZone,
		parseDate,
		today
	} from '@internationalized/date';
	import type { Infer, SuperValidated } from 'sveltekit-superforms';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { page } from '$app/stores';
	import { cn } from '$lib/utils.js';
	import { Button, buttonVariants } from '$lib/components/ui/button/index.js';
	import { Calendar } from '$lib/components/ui/calendar/index.js';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import * as Form from '$lib/components/ui/form/index.js';

	import ArrowUpDown from 'lucide-svelte/icons/arrow-up-down';
	import { tipsSchema, type FormSchema as TipsSchema } from './schema';

	export let data: SuperValidated<Infer<TipsSchema>> = $page.data.datePicker;
	export let tippedDays: Writable<TippedDay[]>;

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

	const table = createTable(tippedDays, {
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
			accessor: 'tipped_hour_for_night',
			header: 'Tipped Hour',
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
			header: 'Total Hourly',
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
			accessor: ({ eid }) => String(eid),
			header: '',
			cell: ({ value }) => {
				return createRender(DataTableActions, { eid: value });
			},
			plugins: {
				sort: {
					disable: true
				}
			}
		})
	]);

	$: form = superForm(data, {
		validators: zodClient(tipsSchema),
		invalidateAll: false,
		resetForm: false
	});

	$: ({ form: formData, enhance } = form);

	const df = new DateFormatter('en-US', {
		dateStyle: 'long'
	});

	let startDateValue: DateValue | undefined;
	$: startDateValue = $formData.startDate ? parseDate($formData.startDate) : undefined;

	let endDateValue: DateValue | undefined;
	$: endDateValue = $formData.endDate ? parseDate($formData.endDate) : undefined;

	let placeholder: DateValue = today(getLocalTimeZone());

	const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
		table.createViewModel(columns);

	const { filterValue } = pluginStates.filter;
	const { hasNextPage, hasPreviousPage, pageIndex } = pluginStates.page;
	export let { exportedData } = pluginStates.export;
</script>

<div class="rounded-md border">
	<Input class="m-2 max-w-sm" placeholder="Filter table..." type="text" bind:value={$filterValue} />
	<div class="flex items-center py-4">
		<form method="POST" enctype="multipart/form-data" class="m-2 flex items-end gap-2" use:enhance>
			<Form.Field {form} name="startDate" class="flex flex-col">
				<Form.Control let:attrs>
					<Form.Label>Start Date</Form.Label>
					<Popover.Root>
						<Popover.Trigger
							{...attrs}
							class={cn(
								buttonVariants({ variant: 'outline' }),
								'w-[280px] justify-start pl-4 text-left font-normal',
								!startDateValue && 'text-muted-foreground'
							)}
						>
							{startDateValue
								? df.format(startDateValue.toDate(getLocalTimeZone()))
								: 'Pick a date'}
							<CalendarIcon class="ml-auto h-4 w-4 opacity-50" />
						</Popover.Trigger>
						<Popover.Content class="w-auto p-0" side="top">
							<Calendar
								value={startDateValue}
								bind:placeholder
								minValue={new CalendarDate(2024, 1, 1)}
								maxValue={today(getLocalTimeZone())}
								calendarLabel="Start Date"
								initialFocus
								onValueChange={(v) => {
									if (v) {
										$formData.startDate = v.toString();
									} else {
										$formData.startDate = '';
									}
								}}
							/>
						</Popover.Content>
					</Popover.Root>
					<Form.Description>The start date of the date range</Form.Description>
					<Form.FieldErrors />
					<input hidden value={$formData.startDate} name={attrs.name} />
				</Form.Control>
			</Form.Field>
			<Form.Field {form} name="endDate" class="flex flex-col">
				<Form.Control let:attrs>
					<Form.Label>End Date</Form.Label>
					<Popover.Root>
						<Popover.Trigger
							{...attrs}
							class={cn(
								buttonVariants({ variant: 'outline' }),
								'w-[280px] justify-start pl-4 text-left font-normal',
								!endDateValue && 'text-muted-foreground'
							)}
						>
							{endDateValue ? df.format(endDateValue.toDate(getLocalTimeZone())) : 'Pick a date'}
							<CalendarIcon class="ml-auto h-4 w-4 opacity-50" />
						</Popover.Trigger>
						<Popover.Content class="w-auto p-0" side="top">
							<Calendar
								value={endDateValue}
								bind:placeholder
								minValue={new CalendarDate(2024, 1, 1)}
								maxValue={today(getLocalTimeZone())}
								calendarLabel="Start Date"
								initialFocus
								onValueChange={(v) => {
									if (v) {
										$formData.endDate = v.toString();
									} else {
										$formData.endDate = '';
									}
								}}
							/>
						</Popover.Content>
					</Popover.Root>
					<Form.Description>The end date of the date range</Form.Description>
					<Form.FieldErrors />
					<input hidden value={$formData.endDate} name={attrs.name} />
				</Form.Control>
			</Form.Field>
			<Button type="submit">Submit</Button>
		</form>
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
