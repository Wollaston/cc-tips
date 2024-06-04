import { fail, superValidate, withFiles } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { formSchema } from './schema';
import type { PageServerLoad, Actions } from './$types';

export const load: PageServerLoad = async () => {
	return {
		form: await superValidate(zod(formSchema))
	};
};

export const actions = {
	default: async ({ request }) => {
		const formData = await request.formData();
		console.log(formData);
		const superform = await superValidate(formData, zod(formSchema));

		if (!superform.valid) return fail(400, withFiles({ form: superform }));

		const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/calculations`, {
			method: 'POST',
			body: formData
		});

		const calculationsResponse: CalculationsResponse = await response.json();

		console.log(calculationsResponse);

		return withFiles({ calculationsResponse, form: superform });
	}
} satisfies Actions;
