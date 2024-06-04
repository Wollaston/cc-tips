<script>
	import './../app.pcss';
	import SideNav from '$lib/SideNav.svelte';
	import Header from '$lib/Header.svelte';
</script>

<div class="flex min-h-screen w-full flex-col bg-muted/40">
	<SideNav />
	<div class="flex flex-col sm:gap-4 sm:py-4 sm:pl-14">
		<Header />
		<slot></slot>
	</div>
</div>
