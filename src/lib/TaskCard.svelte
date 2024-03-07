<script lang="ts">
	import CheckCircle from "./icons/CheckCircle.svelte";
	import type { Task } from "./tasks";
	export let task: Task;
</script>

<div
	class=" transition-all flex p-4 space-x-2 justify-between bg-indigo-300 rounded-md bg-clip-padding backdrop-filter backdrop-blur-[2px] bg-opacity-40 border border-indigo-200 border-opacity-40 shadow-lg"
>
	<div class="flex space-x-2">
		<button on:click={() => task.done = !task.done}>
			<CheckCircle class="size-6 hover:scale-120 text-white transition-all {task.done ? 'hover:text-opacity-95' : 'text-opacity-25 hover:text-opacity-50'}" />
		</button>

		<div>
			<p>{task.name}</p>
		</div>
	</div>
	<div>
		{task.completedPomodoros} / {task.estimatedPomodoros}
	</div>
</div>
