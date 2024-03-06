<script lang="ts">
	import CheckCircle from "./icons/CheckCircle.svelte";
	import type { Task } from "./tasks";
	export let task: Task;
	console.log({ task });
</script>

<div
	class="flex p-4 space-x-2 justify-between bg-indigo-300 rounded-md bg-clip-padding backdrop-filter backdrop-blur-[2px] bg-opacity-40 border border-indigo-200 border-opacity-40 shadow-lg"
>
	<div class="flex space-x-2">
		{#if task.done}
			<CheckCircle className="text-gray-900" />
		{:else}
			<CheckCircle className="text-gray-100" />
		{/if}

		<div>
			<p>{task.name}</p>
		</div>
	</div>
	<div>
		{task.completedPomodoros} / {task.estimatedPomodoros}
	</div>
</div>
