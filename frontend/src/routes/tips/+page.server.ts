import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/tips`, {
		method: 'GET'
	});

	const tippedDays: TippedDay[] = await response.json();

	return { tippedDays };
};
