import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { tipsSchema } from './schema';
import type { PageServerLoad, Actions } from './$types.js';
import { fail } from '@sveltejs/kit';

export const load: PageServerLoad = async () => {
	const tipsForm = await superValidate(zod(tipsSchema));

	return { tipsForm };
};

export const actions: Actions = {
	default: async ({ request }) => {
		const formData = await request.formData();
		console.log(formData);
		const form = await superValidate(formData, zod(tipsSchema));
		if (!form.valid) {
			return fail(400, {
				form
			});
		}

		const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/tips`, {
			method: 'POST',
			body: formData
		});

		const tippedDays: TippedDay[] = await response.json();

		return {
			tippedDays,
			form
		};
	}
};
