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
	class={`transition-all flex flex-col space-y-2 items-start bg-indigo-300 border-indigo-200 glass-morphism`}
>
	<div class="w-full flex flex-col items-start p-4 space-y-2">
		<input
			bind:value={newTask.name}
			placeholder="Task name"
			class="w-full p-4 bg-indigo-200 bg-opacity-10 border-opacity-10 border-none active:border-none glass-morphism-basic"
		/>
		<div>Est Pomodoros</div>
		<div class="flex max-h-12 space-x-2">
			<input
				bind:value={newTask.estimatedPomodoros}
				type="number"
				class="p-4 w-24 bg-indigo-200 bg-opacity-10 border-opacity-10 border-none active:border-none glass-morphism-basic"
			/>
			<button on:click={() => newTask.estimatedPomodoros++}>+</button>
			<button
				on:click={() =>
					(newTask.estimatedPomodoros = Math.max(
						0,
						newTask.estimatedPomodoros - 1,
					))}>-</button
			>
		</div>
	</div>
	<div
		class="w-full space-x-2 bg-indigo-600 bg-opacity-20 p-4 flex justify-end"
	>
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
