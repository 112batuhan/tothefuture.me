<script lang="ts">
	function beforeunload(event: BeforeUnloadEvent) {
		event.preventDefault();
		return (event.returnValue = '');
	}
</script>

<svelte:window on:beforeunload={beforeunload} />
