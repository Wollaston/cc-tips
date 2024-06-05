<script lang="ts">
	import PanelLeft from 'lucide-svelte/icons/panel-left';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import { Calculator, CreditCard, Receipt } from 'lucide-svelte';
</script>

<header
	class="sticky top-0 z-30 flex h-14 items-center gap-4 border-b bg-background px-4 sm:static sm:h-auto sm:border-0 sm:bg-transparent sm:px-6"
>
	<Sheet.Root>
		<Sheet.Trigger asChild let:builder>
			<Button builders={[builder]} size="icon" variant="outline" class="sm:hidden">
				<PanelLeft class="h-5 w-5" />
				<span class="sr-only">Toggle Menu</span>
			</Button>
		</Sheet.Trigger>
		<Sheet.Content side="left" class="sm:max-w-xs">
			<nav class="grid gap-6 text-lg font-medium">
				<a
					href="/"
					class="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"
				>
					<Calculator class="h-5 w-5" />
					Calculator
				</a>
			</nav>
		</Sheet.Content>
		<Sheet.Content side="left" class="sm:max-w-xs">
			<nav class="grid gap-6 text-lg font-medium">
				<a
					href="/cards"
					class="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"
				>
					<CreditCard class="h-5 w-5" />
					Cards
				</a>
			</nav>
		</Sheet.Content>
		<Sheet.Content side="left" class="sm:max-w-xs">
			<nav class="grid gap-6 text-lg font-medium">
				<a
					href="/tips"
					class="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"
				>
					<Receipt class="h-5 w-5" />
					Tips
				</a>
			</nav>
		</Sheet.Content>
	</Sheet.Root>
</header>
