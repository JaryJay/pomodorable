<script lang="ts">
	let saveLocation = "";
</script>

<div class="">
	<form on:submit|preventDefault={() => {}}>
		<input bind:value={saveLocation} />
	</form>
</div>
