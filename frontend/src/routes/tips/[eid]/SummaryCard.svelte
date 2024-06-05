<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';

	export let summary;
</script>

<div>
	<Card.Root class="overflow-hidden">
		<Card.Header class="flex flex-row items-start bg-muted/50">
			<div class="grid gap-0.5">
				<Card.Title class="group flex items-center gap-2 text-lg"
					>Key Database Information</Card.Title
				>
			</div>
		</Card.Header>
		<Card.Content class="p-6 text-sm">
			<div class="grid gap-3">
				<div class="font-semibold">IDs</div>
				<ul class="grid gap-3">
					<li class="flex items-center justify-between">
						<span class="text-muted-foreground"> rapidPay! Card ID </span>
						<span>{summary.card_id}</span>
					</li>
					<li class="flex items-center justify-between">
						<span class="text-muted-foreground"> R365 EID </span>
						<span>{summary.eid}</span>
					</li>
				</ul>
			</div></Card.Content
		>
		<Card.Footer class="flex flex-row items-center border-t bg-muted/50 px-6 py-3">
			<div class="text-xs text-muted-foreground">
				Please contact administrator to make any updates or changes admin@curiouscorkwine.com
			</div>
		</Card.Footer>
	</Card.Root>
</div>
