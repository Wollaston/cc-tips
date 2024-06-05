<script lang="ts">
	import { BookUser } from 'lucide-svelte';

	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import { memberDetailStore } from '$lib/stores';

	interface MemberDetailResponse {
		staff_member: MemberDetail;
		tips: TipSummary[];
	}

	interface MemberDetail {
		name: string;
		eid: number;
		card_id: string;
	}

	interface TipSummary {
		date: string;
		net_tips: number;
	}

	let tips_summary: MemberDetailResponse | undefined;

	memberDetailStore.subscribe((value) => {
		tips_summary = value;
	});
</script>

<div>
	<Card.Root class="overflow-hidden">
		{#if tips_summary === undefined}
			<Card.Header class="flex flex-row items-start bg-muted/50">
				<div class="grid gap-0.5">
					<Card.Title class="group flex items-center gap-2 text-lg"
						>Select a Staff Member to Load Summary</Card.Title
					>
				</div>
			</Card.Header>
		{/if}
		{#if tips_summary !== undefined}
			<Card.Header class="flex flex-row items-start bg-muted/50">
				<div class="grid gap-0.5">
					<Card.Title class="group flex items-center gap-2 text-lg"
						>{tips_summary.staff_member.name}</Card.Title
					>
				</div>
				<div class="ml-auto flex items-center gap-1">
					<Button size="sm" variant="outline" class="h-8 gap-1">
						<BookUser class="h-3.5 w-3.5" />
						<span class="lg:sr-only xl:not-sr-only xl:whitespace-nowrap">Detail</span>
					</Button>
				</div>
			</Card.Header>
			<Card.Content class="p-6 text-sm">
				<div class="grid gap-3">
					<div class="grid gap-3">
						<div class="font-semibold">Tips Summary for Last 10 Tipped Shifts</div>
						<dl class="grid gap-3">
							<div class="flex items-center justify-between">
								<dt class="font-semibold">Date</dt>
								<dd class="font-semibold">Net Tips</dd>
							</div>
							{#if tips_summary.tips.length === 0}
								<h3 class="text-muted-foreground">No Tips in Database</h3>
							{:else}
								{#each tips_summary.tips as tip}
									<div class="flex items-center justify-between">
										<dt class="text-muted-foreground">
											{new Intl.DateTimeFormat('en-US', {
												weekday: 'long',
												year: 'numeric',
												month: 'long',
												day: 'numeric',
												timeZone: 'America/New_York'
											}).format(new Date(tip.date + 'T00:00:00.000'))}
										</dt>
										<dd>
											{new Intl.NumberFormat('en-US', {
												style: 'currency',
												currency: 'USD'
											}).format(tip.net_tips)}
										</dd>
									</div>
								{/each}
							{/if}
						</dl>
					</div>
					<Separator class="my-2" />
					<div class="font-semibold">IDs</div>
					<ul class="grid gap-3">
						<li class="flex items-center justify-between">
							<span class="text-muted-foreground"> rapidPay! Card ID </span>
							<span>{tips_summary.staff_member.card_id}</span>
						</li>
						<li class="flex items-center justify-between">
							<span class="text-muted-foreground"> R365 EID </span>
							<span>{tips_summary.staff_member.eid}</span>
						</li>
					</ul>
				</div></Card.Content
			>
			<Card.Footer class="flex flex-row items-center border-t bg-muted/50 px-6 py-3">
				<div class="text-xs text-muted-foreground">
					Summary only a reflection of data in the database. Any tips calculated outside are not
					represented.
				</div>
			</Card.Footer>
		{/if}
	</Card.Root>
</div>
