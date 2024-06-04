<script lang="ts">
	import { Progress } from '$lib/components/ui/progress/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
</script>

<div class="grid gap-4 sm:grid-cols-2 md:grid-cols-4 lg:grid-cols-2 xl:grid-cols-4">
	<Card.Root>
		<Card.Header class="pb-2">
			<Card.Description>GoTab Tips</Card.Description>
			<Card.Title class="text-4xl">GoTab Tips</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="text-xs text-muted-foreground">+25% from last week</div>
		</Card.Content>
		<Card.Footer>
			<Progress value={25} aria-label="25% increase" />
		</Card.Footer>
	</Card.Root>
	<Card.Root>
		<Card.Header class="pb-2">
			<Card.Description>Cash Tips</Card.Description>
			<Card.Title class="text-4xl">Cash Tips</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="text-xs text-muted-foreground">+25% from last week</div>
		</Card.Content>
		<Card.Footer>
			<Progress value={25} aria-label="25% increase" />
		</Card.Footer>
	</Card.Root>
	<Card.Root>
		<Card.Header class="pb-2">
			<Card.Description>Top Beers by Quantity</Card.Description>
			<Card.Title class="text-4xl">$1329</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="text-xs text-muted-foreground">+25% from last week</div>
		</Card.Content>
		<Card.Footer>
			<Progress value={25} aria-label="25% increase" />
		</Card.Footer>
	</Card.Root>
	<Card.Root>
		<Card.Header class="pb-2">
			<Card.Description>Top Beers by Subtotal</Card.Description>
			<Card.Title class="text-3xl">$5,329</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="text-xs text-muted-foreground">+10% from last month</div>
		</Card.Content>
		<Card.Footer>
			<Progress value={12} aria-label="12% increase" />
		</Card.Footer>
	</Card.Root>
</div>
