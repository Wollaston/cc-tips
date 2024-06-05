<script lang="ts">
	import * as Card from '$lib/components/ui/card';

	export let posted: boolean;
	export let average_net_hourly_pay;
	export let total_tips;
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Tip Pool Summary</Card.Title>
		<Card.Description>Summary statistics for the daily tip pool</Card.Description>
	</Card.Header>
	{#if !posted}
		<Card.Content>
			<p class="text-sm text-muted-foreground">Generate the tip pool to see the daily summary</p>
		</Card.Content>
	{:else}
		<Card.Content>
			<h1 class="text-sm text-muted-foreground">Average Hourly Rate: {average_net_hourly_pay}</h1>
			<h1 class="text-sm text-muted-foreground">Sum of Calculated Tips: {total_tips}</h1>
		</Card.Content>
	{/if}
</Card.Root>
