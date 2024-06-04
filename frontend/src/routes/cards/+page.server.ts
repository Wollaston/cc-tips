import type { Actions, PageServerLoad } from './$types';
import { fail, superValidate } from 'sveltekit-superforms';
import { formSchema } from './schema';
import { zod } from 'sveltekit-superforms/adapters';

export const load: PageServerLoad = async ({ fetch }) => {
	const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/staff`, {
		method: 'GET'
	});

	const staff: StaffMember[] = await response.json();

	return { staff, form: await superValidate(zod(formSchema)) };
};

export const actions: Actions = {
	default: async ({ request, fetch }) => {
		const formData = await request.formData();
		const form = await superValidate(formData, zod(formSchema));

		if (!form.valid) {
			return fail(400, {
				form
			});
		}

		const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/staff`, {
			method: 'POST',
			body: JSON.stringify(form.data),
			headers: { 'Content-type': 'application/json' }
		});

		const staff: StaffMember[] = await response.json();

		return {
			form,
			staff
		};
	}
};
