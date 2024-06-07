import type { Actions, PageServerLoad } from './$types';
import { fail, superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { commissionsSchema } from './commissionsSchema';
import type { Commission, StaffNameEid, WineNamePrice } from '$lib/types';

export const load: PageServerLoad = async ({ fetch }) => {
	const commissionsResponse = await fetch(`${import.meta.env.VITE_BACKEND_URL}/commissions`, {
		method: 'GET'
	});
	const commissions: Commission[] = await commissionsResponse.json();

	const staffResponse = await fetch(`${import.meta.env.VITE_BACKEND_URL}/staff/eid-name`, {
		method: 'GET'
	});
	const staff: StaffNameEid[] = await staffResponse.json();

	const winesResponse = await fetch(`${import.meta.env.VITE_BACKEND_URL}/wines/name-bottle-price`, {
		method: 'GET'
	});
	const wines: WineNamePrice[] = await winesResponse.json();

	const commissionsForm = await superValidate(zod(commissionsSchema));

	return { commissions, staff, wines, commissionsForm };
};

export const actions: Actions = {
	default: async ({ request, fetch }) => {
		const commissionsData = await request.formData();
		const commissionsForm = await superValidate(commissionsData, zod(commissionsSchema));

		if (!commissionsForm.valid) {
			return fail(400, {
				form: commissionsForm
			});
		}

		console.log(
			JSON.stringify({
				eid: commissionsData.get('eid'),
				product_id: commissionsData.get('product_id')
			})
		);

		const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/commissions`, {
			method: 'POST',
			body: JSON.stringify({
				eid: commissionsData.get('eid'),
				product_id: commissionsData.get('product_id')
			}),
			headers: { 'Content-type': 'application/json' }
		});

		const commissions: StaffMember[] = await response.json();

		console.log(commissions);

		return {
			form: commissionsForm
		};
	}
};
