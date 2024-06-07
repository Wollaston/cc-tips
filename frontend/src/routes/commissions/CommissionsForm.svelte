<script lang="ts">
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as Form from '$lib/components/ui/form';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { commissionsSchema, type FormSchema as CommissionsSchema } from './commissionsSchema';
	import Check from 'svelte-radix/Check.svelte';
	import CaretSort from 'svelte-radix/CaretSort.svelte';
	import { tick } from 'svelte';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import * as Command from '$lib/components/ui/command/index.js';
	import { cn } from '$lib/utils.js';
	import { UserRoundPlus } from 'lucide-svelte';

	import type { StaffNameEid, WineNamePrice } from '$lib';
	import { page } from '$app/stores';

	export let commissionsData: SuperValidated<Infer<CommissionsSchema>> = $page.data.combobox;

	export let wines: WineNamePrice[];
	export let staff: StaffNameEid[];

	const form = superForm(commissionsData, {
		validators: zodClient(commissionsSchema)
	});

	const { form: formData, enhance } = form;

	let staffOpen: boolean = false;

	function closeStaffAndFocusTrigger(triggerId: string) {
		staffOpen = false;
		tick().then(() => {
			document.getElementById(triggerId)?.focus();
		});
	}

	let wineOpen: boolean = false;

	function closeWineAndFocusTrigger(triggerId: string) {
		wineOpen = false;
		tick().then(() => {
			document.getElementById(triggerId)?.focus();
		});
	}
</script>

<Dialog.Root>
	<Dialog.Trigger class={buttonVariants({ size: 'sm', variant: 'outline' })}>
		<UserRoundPlus class="h-3.5 w-3.5" />
		<span class="pl-1 text-sm">New</span>
	</Dialog.Trigger>
	<Dialog.Content class="sm:max-w-[550px]">
		<Dialog.Header>
			<Dialog.Title>Create a New Commission</Dialog.Title>
			<Dialog.Description
				>Select a staff member from the list and the wine sold to generate a default commission</Dialog.Description
			>
		</Dialog.Header>
		<form method="POST" use:enhance class="grid gap-4 py-4">
			<Form.Field {form} name="eid" class="flex flex-col">
				<Popover.Root bind:open={staffOpen} let:ids>
					<Form.Control let:attrs>
						<Form.Label>Staff Member</Form.Label>
						<Popover.Trigger
							class={cn(
								buttonVariants({ variant: 'outline' }),
								'w-[500px] justify-between',
								!$formData.eid && 'text-muted-foreground'
							)}
							role="combobox"
							{...attrs}
						>
							{staff.find((f) => f.eid === $formData.eid)?.name ?? 'Select Staff Member'}
							<CaretSort class="ml-2 h-4 w-4 shrink-0 opacity-50" />
						</Popover.Trigger>
						<input hidden value={$formData.eid} name={attrs.name} />
					</Form.Control>
					<Popover.Content class="w-[500px] p-0">
						<Command.Root>
							<Command.Input autofocus placeholder="Search staff..." class="h-9" />
							<Command.Empty>No staff member found.</Command.Empty>
							<Command.Group>
								{#each staff as member}
									<Command.Item
										value={String(member.name)}
										onSelect={() => {
											$formData.eid = member.eid;
											closeStaffAndFocusTrigger(ids.trigger);
										}}
									>
										{member.name}
										<Check
											class={cn(
												'ml-auto h-4 w-4',
												member.eid !== $formData.eid && 'text-transparent'
											)}
										/>
									</Command.Item>
								{/each}
							</Command.Group>
						</Command.Root>
					</Popover.Content>
				</Popover.Root>
				<Form.Description>This is the staff member that earned the commission.</Form.Description>
				<Form.FieldErrors />
			</Form.Field>
			<Form.Field {form} name="product_id" class="flex flex-col">
				<Popover.Root bind:open={wineOpen} let:ids>
					<Form.Control let:attrs>
						<Form.Label>Wine</Form.Label>
						<Popover.Trigger
							class={cn(
								buttonVariants({ variant: 'outline' }),
								'w-[500px] justify-between',
								!$formData.product_id && 'text-muted-foreground'
							)}
							role="combobox"
							{...attrs}
						>
							{wines.find((f) => f.product_id === $formData.product_id)?.name ?? 'Select Wine'}
							<CaretSort class="ml-2 h-4 w-4 shrink-0 opacity-50" />
						</Popover.Trigger>
						<input hidden value={$formData.product_id} name={attrs.name} />
					</Form.Control>
					<Popover.Content class="w-[500px] p-0">
						<Command.Root>
							<Command.Input autofocus placeholder="Search wines..." class="h-9" />
							<Command.Empty>No wine found.</Command.Empty>
							<Command.Group>
								{#each wines as wine}
									<Command.Item
										value={wine.name}
										onSelect={() => {
											$formData.product_id = wine.product_id;
											closeWineAndFocusTrigger(ids.trigger);
										}}
									>
										{wine.name}
										<Check
											class={cn(
												'ml-auto h-4 w-4',
												wine.product_id !== $formData.product_id && 'text-transparent'
											)}
										/>
									</Command.Item>
								{/each}
							</Command.Group>
						</Command.Root>
					</Popover.Content>
				</Popover.Root>
				<Form.Description>The wine sold for a commissions.</Form.Description>
				<Form.FieldErrors />
			</Form.Field>
			<Dialog.Footer>
				<Form.Button>Submit</Form.Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
