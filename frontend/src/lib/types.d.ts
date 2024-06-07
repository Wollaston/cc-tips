export interface StaffMember {
	name: string;
	card_id: string;
	eid: number;
	created: string;
	modified: string;
}

export interface CalculationsResponse {
	calculations_link: string;
	template_link: string;
	summary: Summary;
	tips: Tip[];
}

export interface Summary {
	total_tips: number;
	average_net_hourly_pay: number;
}

export interface Tip {
	employee: string;
	role: string;
	net_tips: number;
	total_pay_for_night: number;
	hourly_pay_for_night: number;
	duration: number;
	eid: number;
}

export interface MemberDetail {
	name: string;
	eid: number;
	card_id: string;
}

export interface MemberDetailResponse {
	staff_member: MemberDetail;
	tips: TipSummary[];
}

export interface TipSummary {
	date: string;
	net_tips: number;
}

export interface TippedDay {
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

export interface StaffNameEid {
	name: string;
	eid: number;
}

export interface WineNamePrice {
	base_price: number;
	display_price: string;
	name: string;
	product_id: number;
}

export interface Commission {
	name: string;
	wine: string;
	amount: number;
	product_id: number;
	date: Date;
}

export interface Thing {
	tb: string;
	id: i32;
}
