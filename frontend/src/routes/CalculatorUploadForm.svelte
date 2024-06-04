<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import {
		DateFormatter,
		type DateValue,
		parseDate,
		today,
		getLocalTimeZone,
		CalendarDate
	} from '@internationalized/date';
	import { formSchema, type FormSchema } from './schema';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { buttonVariants } from '$lib/components/ui/button';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { CalendarIcon } from 'lucide-svelte';
	import { page } from '$app/stores';
	import { cn } from '$lib/utils.js';
	import { Calendar } from '$lib/components/ui/calendar/index.js';
	import * as Card from '$lib/components/ui/card/index.js';

	export let data: SuperValidated<Infer<FormSchema>> = $page.data.datePicker;

	const form = superForm(data, {
		validators: zodClient(formSchema)
	});
	const { form: formData, enhance } = form;

	const df = new DateFormatter('en-US', {
		dateStyle: 'long'
	});

	let value: DateValue | undefined;

	$: value = $formData.date ? parseDate($formData.date) : undefined;

	let placeholder: DateValue = today(getLocalTimeZone());
</script>

<Card.Root class="m-4">
	<Card.Header>
		<Card.Title>Tip Pool Calculator</Card.Title>
		<Card.Description>Use this form to run the tip pool for a specific day</Card.Description>
	</Card.Header>
	<Card.Content>
		<form class="mx-auto max-w-sm" method="POST" enctype="multipart/form-data" use:enhance>
			<Form.Field {form} name="date">
				<Form.Control let:attrs>
					<Form.Label>Labor Report Date</Form.Label>
					<Popover.Root>
						<Popover.Trigger
							{...attrs}
							class={cn(
								buttonVariants({ variant: 'outline' }),
								'w-[350px] justify-start pl-4 text-left font-normal',
								!value && 'text-muted-foreground'
							)}
						>
							{value ? df.format(value.toDate(getLocalTimeZone())) : 'Pick a date'}
							<CalendarIcon class="ml-auto h-4 w-4 opacity-50" />
						</Popover.Trigger>
						<Popover.Content class="w-auto p-0" side="top">
							<Calendar
								{value}
								bind:placeholder
								minValue={new CalendarDate(1900, 1, 1)}
								maxValue={today(getLocalTimeZone())}
								calendarLabel="Date of birth"
								initialFocus
								onValueChange={(v) => {
									if (v) {
										$formData.date = v.toString();
									} else {
										$formData.date = '';
									}
								}}
							/>
						</Popover.Content>
					</Popover.Root>
					<Form.Description>The date of the Labor Report for the Tip Pool</Form.Description>
					<Form.FieldErrors />
					<input hidden value={$formData.date} name={attrs.name} />
				</Form.Control>
			</Form.Field>
			<Form.Field {form} name="totalSales">
				<Form.Control let:attrs>
					<Form.Label>Total Sales</Form.Label>
					<Input class="w-[350px]" {...attrs} bind:value={$formData.totalSales} />
				</Form.Control>
				<Form.Description>The total sales amount from GoTab</Form.Description>
				<Form.FieldErrors />
			</Form.Field>
			<Form.Field {form} name="gotabTips">
				<Form.Control let:attrs>
					<Form.Label>Go Tab Tips</Form.Label>
					<Input class="w-[350px]" {...attrs} bind:value={$formData.gotabTips} />
				</Form.Control>
				<Form.Description>The total amount of tips on GoTab</Form.Description>
				<Form.FieldErrors />
			</Form.Field>
			<Form.Field {form} name="cashTips">
				<Form.Control let:attrs>
					<Form.Label>Cash Tips</Form.Label>
					<Input class="w-[350px]" {...attrs} bind:value={$formData.cashTips} />
				</Form.Control>
				<Form.Description>The total amount of cash tips for the night</Form.Description>
				<Form.FieldErrors />
			</Form.Field>
			<Form.Field {form} name="laborReport">
				<Form.Control let:attrs>
					<Form.Label>Upload Labor Report</Form.Label>
					<Input
						class="w-[350px]"
						{...attrs}
						bind:value={$formData.laborReport}
						type="file"
						accept="text/csv"
					/>
				</Form.Control>
				<Form.Description
					>Upload a copy of this day's labor report as exported from GoTab</Form.Description
				>
				<Form.FieldErrors />
			</Form.Field>
			<Form.Button>Submit</Form.Button>
		</form>
	</Card.Content>
</Card.Root>
