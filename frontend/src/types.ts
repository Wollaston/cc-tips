interface StaffMember {
	name: string;
	card_id: string;
	eid: number;
	created: string;
	modified: string;
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

interface MemberDetail {
	name: string;
	eid: number;
	card_id: string;
}

interface MemberDetailResponse {
	staff_member: MemberDetail;
	tips: TipSummary[];
}

interface TipSummary {
	date: string;
	net_tips: number;
}

interface TippedDay {
	name: string;
	employee: string;
	role: string;
	net_tips: number;
	total_pay_for_night: number;
	hourly_pay_for_night: number;
	tipped_hour_for_night: number;
	duration: number;
	eid: number;
	date: string;
	created: string;
	modified: string;
}

export * from './types';
