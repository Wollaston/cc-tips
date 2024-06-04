<script lang="ts">
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Form from '$lib/components/ui/form';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { registerSchema, type FormSchema } from './registerSchema';

	import { UserRoundPlus } from 'lucide-svelte';

	export let data: SuperValidated<Infer<FormSchema>>;

	const form = superForm(data, {
		validators: zodClient(registerSchema)
	});

	const { form: formData, enhance } = form;

	let open: boolean = false;
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger class={buttonVariants({ size: 'sm', variant: 'outline' })}>
		<UserRoundPlus class="h-3.5 w-3.5" />
		<span class="pl-1 text-sm">New</span>
	</Dialog.Trigger>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>New Staff Member</Dialog.Title>
			<Dialog.Description>Enroll a new Staff Member in the tip pool</Dialog.Description>
		</Dialog.Header>
		<form method="POST" action="?/register" use:enhance class="grid gap-4 py-4">
			<Form.Field {form} name="name" class="grid grid-cols-4 items-center gap-4">
				<Form.Control let:attrs>
					<Form.Label class="text-right">Name</Form.Label>
					<Input class="col-span-3" {...attrs} bind:value={$formData.name} />
				</Form.Control>
				<Form.Description class="col-span-4">This must match their GoTab name.</Form.Description>
				<Form.FieldErrors />
			</Form.Field>
			<Form.Field {form} name="card_id" class="grid grid-cols-4 items-center gap-4">
				<Form.Control let:attrs>
					<Form.Label class="text-right">Card ID</Form.Label>
					<Input class="col-span-3" {...attrs} bind:value={$formData.card_id} />
				</Form.Control>
				<Form.Description class="col-span-4"
					>This is their 10-digit card ID from rapidPay!</Form.Description
				>
				<Form.FieldErrors />
			</Form.Field>
			<Form.Field {form} name="eid" class="grid grid-cols-4 items-center gap-4">
				<Form.Control let:attrs>
					<Form.Label class="text-right">EID</Form.Label>
					<Input class="col-span-3" {...attrs} bind:value={$formData.eid} />
				</Form.Control>
				<Form.Description class="col-span-4"
					>The staff member's EID from R365 Workforce</Form.Description
				>
				<Form.FieldErrors />
			</Form.Field>
			<Dialog.Footer>
				<Form.Button>Submit</Form.Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
