<script lang="ts">
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Form from '$lib/components/ui/form';
	import { importSchema, type FormSchema } from './importSchema';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { FileUp } from 'lucide-svelte';

	export let data: SuperValidated<Infer<FormSchema>>;

	const importForm = superForm(data, {
		validators: zodClient(importSchema)
	});

	const { form: formData, enhance } = importForm;
</script>

<Dialog.Root>
	<Dialog.Trigger class={buttonVariants({ size: 'sm', variant: 'outline' })}>
		<FileUp class="h-3.5 w-3.5" />
		<span class="pl-1 text-sm">Import</span>
	</Dialog.Trigger>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Import Staff</Dialog.Title>
			<Dialog.Description
				>Import staff to the database, including GoTab name, rapidPay! card number, and R365 EID</Dialog.Description
			>
		</Dialog.Header>
		<form
			method="POST"
			action="?/import"
			enctype="multipart/form-data"
			use:enhance
			class="grid gap-4 py-4"
		>
			<Form.Field form={importForm} name="importFile">
				<Form.Control let:attrs>
					<Form.Label>Upload Staff Data</Form.Label>
					<Input
						class="w-[350px]"
						{...attrs}
						bind:value={$formData.importFile}
						type="file"
						accept="text/csv"
					/>
				</Form.Control>
				<Form.Description>Upload Staff information as a .csv</Form.Description>
				<Form.FieldErrors />
			</Form.Field>
			<Dialog.Footer>
				<Form.Button>Submit</Form.Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
