import type { PageServerLoad } from './$types';

interface SummaryStats {
	net_tips_sum: number;
	total_pay: number;
	total_hours: number;
	average_hourly: number;
}

export const load: PageServerLoad = async ({ fetch, params }) => {
	const tipsResponse = await fetch(`${import.meta.env.VITE_BACKEND_URL}/tips/${params.eid}`, {
		method: 'GET'
	});

	const tippedDays: TippedDay[] = await tipsResponse.json();

	const detailResponse = await fetch(`${import.meta.env.VITE_BACKEND_URL}/staff/${params.eid}`, {
		method: 'GET'
	});
	const staffDetail: StaffMember = await detailResponse.json();

	const summaryResponse = await fetch(
		`${import.meta.env.VITE_BACKEND_URL}/staff/${params.eid}/summary`,
		{
			method: 'GET'
		}
	);
	const summaryStats: SummaryStats = await summaryResponse.json();

	return { tippedDays, staffDetail, summaryStats };
};
