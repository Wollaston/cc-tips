import type { Actions, PageServerLoad } from './$types';
import { fail, superValidate, withFiles } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { registerSchema } from './registerSchema';
import { importSchema } from './importSchema';
import { memberDetailSchema } from './memberDetailSchema';

export const load: PageServerLoad = async ({ fetch }) => {
	const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/staff`, {
		method: 'GET'
	});

	const staff: StaffMember[] = await response.json();

	const registerForm = await superValidate(zod(registerSchema));
	const importForm = await superValidate(zod(importSchema));

	return { staff, registerForm, importForm };
};

export const actions: Actions = {
	register: async ({ request, fetch }) => {
		const registerData = await request.formData();
		const registerForm = await superValidate(registerData, zod(registerSchema));

		if (!registerForm.valid) {
			return fail(400, {
				form: registerForm
			});
		}

		const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/staff`, {
			method: 'POST',
			body: JSON.stringify(registerForm.data),
			headers: { 'Content-type': 'application/json' }
		});

		const staff: StaffMember[] = await response.json();

		return {
			form: registerForm,
			staff
		};
	},

	import: async ({ request, fetch }) => {
		const importData = await request.formData();
		const importForm = await superValidate(importData, zod(importSchema));

		if (!importForm.valid) return fail(400, withFiles({ form: importForm }));

		const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/import-staff`, {
			method: 'POST',
			body: importData
		});

		const importResponse: StaffMember[] = await response.json();

		console.log(importResponse);

		return withFiles({ importResponse, form: importForm });
	},

	member: async ({ request, fetch }) => {
		const memberDetailData = await request.formData();
		const memberDetailForm = await superValidate(memberDetailData, zod(memberDetailSchema));

		if (!memberDetailForm.valid) {
			return fail(400, {
				form: memberDetailForm
			});
		}

		const response = await fetch(
			`${import.meta.env.VITE_BACKEND_URL}/staff/${memberDetailForm.data.eid}`,
			{
				method: 'POST'
			}
		);

		const memberDetail: MemberDetail = await response.json();

		return {
			form: memberDetailForm,
			memberDetail
		};
	}
};
