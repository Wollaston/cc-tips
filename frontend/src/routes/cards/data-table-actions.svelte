<script lang="ts">
	import Ellipsis from 'lucide-svelte/icons/ellipsis';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/button';
	import { memberDetailStore } from '$lib/stores';

	export let eid: number;
	let path = `/tips/${eid}`;

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

	async function get_member_detail(eid: number) {
		let res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/staff/${eid}`, {
			method: 'post'
		});

		let data: MemberDetailResponse = await res.json();

		console.log(data);

		memberDetailStore.update(() => data);
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger asChild let:builder>
		<Button variant="ghost" builders={[builder]} size="icon" class="relative h-8 w-8 p-0">
			<span class="sr-only">Open menu</span>
			<Ellipsis class="h-4 w-4" />
		</Button>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Label>Actions</DropdownMenu.Label>
			<DropdownMenu.Separator />
			<DropdownMenu.Item on:click={() => navigator.clipboard.writeText(String(eid))}>
				Copy EID
			</DropdownMenu.Item>
			<DropdownMenu.Item on:click={() => get_member_detail(eid)}>Summary</DropdownMenu.Item>
			<DropdownMenu.Item href={path}>Detail</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
