<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

	import { FileDown } from 'lucide-svelte';

	interface Tip {
		employee: string;
		role: string;
		net_tips: number;
		total_pay_for_night: number;
		hourly_pay_for_night: number;
		duration: number;
		eid: number;
	}

	export let tips: Tip[];
	export let valid: boolean;
	export let calculations_link;
	export let template_link;
</script>

<Card.Root class="m-4">
	<Card.Header class="flex flex-row justify-between">
		<Card.Title>Calculated Tip Pool</Card.Title>
		{#if valid}
			<DropdownMenu.Root>
				<DropdownMenu.Trigger asChild let:builder>
					<Button
						variant="outline"
						size="sm"
						class="h-7 w-[150px] gap-1 text-sm"
						builders={[builder]}
					>
						<FileDown class="h-3.5 w-3.5" />
						<span class="sr-only sm:not-sr-only">Export</span>
					</Button>
				</DropdownMenu.Trigger>
				<DropdownMenu.Content>
					<DropdownMenu.Item><a href={calculations_link}>Calculations</a></DropdownMenu.Item>
					<DropdownMenu.Item><a href={template_link}>Upload Template</a></DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		{/if}
	</Card.Header>
	<Card.Content>
		<Table.Root>
			{#if !valid}
				<Table.Caption>Generate the tip pool to see the daily summary</Table.Caption>
			{/if}
			<Table.Header>
				<Table.Row>
					<Table.Head>Employee</Table.Head>
					<Table.Head>Role</Table.Head>
					<Table.Head>Net Tips</Table.Head>
					<Table.Head>Total Pay</Table.Head>
					<Table.Head>Hourly Pay</Table.Head>
					<Table.Head>Duration</Table.Head>
					<Table.Head>EID</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#if valid}
					{#if tips.length ?? 0 > 0}
						{#each tips as tip}
							<Table.Row>
								<Table.Cell class="font-medium">{tip.employee}</Table.Cell>
								<Table.Cell>{tip.role}</Table.Cell>
								<Table.Cell>{tip.net_tips}</Table.Cell>
								<Table.Cell>{tip.total_pay_for_night}</Table.Cell>
								<Table.Cell>{tip.hourly_pay_for_night}</Table.Cell>
								<Table.Cell>{tip.duration}</Table.Cell>
								<Table.Cell>{tip.eid}</Table.Cell>
							</Table.Row>
						{/each}
					{/if}
				{/if}
			</Table.Body>
		</Table.Root>
	</Card.Content>
</Card.Root>
