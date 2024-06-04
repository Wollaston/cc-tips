interface StaffMember {
	name: string;
	card_id: string;
	created: string;
}

interface CalculationsResponse {
	calculations_link: string;
	template_link: string;
	summary: Summary;
	tips: Tip[];
}

interface Summary {
	total_tips: number;
	average_net_hourly_pay: number;
}

interface Tip {
	employee: string;
	role: string;
	net_tips: number;
	total_pay_for_night: number;
	hourly_pay_for_night: number;
	duration: number;
	eid: number;
}