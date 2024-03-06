<script lang="ts">
	import { createEventDispatcher } from "svelte";
	import type { Task } from "./tasks";

	const dispatch = createEventDispatcher();

	const generateNewTask = (): Task => ({
		name: "",
		description: "",
		completedPomodoros: 0,
		estimatedPomodoros: 1,
		done: false,
	});
	let newTask: Task = generateNewTask();
</script>

<div
	class="flex flex-col space-y-2 items-start bg-indigo-300 border-indigo-200 glass-morphism"
>
	<div class="w-full flex flex-col items-start p-4 space-y-2">
		<input bind:value={newTask.name} />
		<div>Est Pomodoros</div>
		<input bind:value={newTask.estimatedPomodoros} type="number" />
	</div>
	<div class="w-full bg-indigo-600 bg-opacity-20 p-4 flex justify-end">
		<button
			on:click={() => {
				dispatch("create", newTask);
				newTask = generateNewTask();
			}}
		>
			Save
		</button>
	</div>
</div>
