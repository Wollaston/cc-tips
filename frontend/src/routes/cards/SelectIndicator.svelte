<script>
	export let isSelected; //: Writable<boolean>;
	export let isSomeSubRowsSelected; //: Readable<boolean>;
</script>

<input type="checkbox" bind:checked={$isSelected} indeterminate={$isSomeSubRowsSelected} />
