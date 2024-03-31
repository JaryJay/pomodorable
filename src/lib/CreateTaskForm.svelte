<script lang="ts">
	import { createEventDispatcher, onMount } from "svelte";
	import type { Task } from "./tasks";
	import Button from "./Button.svelte";
    import Timer from "./Timer.svelte";

	export let expanded = false;

	let nameInput: HTMLInputElement;
	let container: HTMLDivElement;

	const dispatch = createEventDispatcher();

	const generateNewTask = (): Task => ({
		id: Date.now(),
		name: "",
		description: "",
		completedPomodoros: 0,
		estimatedPomodoros: 1,
		done: false,
	});
	let newTask: Task = generateNewTask();

	function handleCreate() {
		if (!newTask.estimatedPomodoros) newTask.estimatedPomodoros = 0;
		dispatch("create", newTask);
		newTask = generateNewTask();
		nameInput.focus();
	}

	function handleMousePressOutside(event: MouseEvent) {
		if (expanded && container && !event.composedPath().includes(container)) {
			expanded = false;
		}
	}
	function onKeyDown(event: KeyboardEvent) {
		if (event.key == 'Enter') {
			handleCreate();
		}
	}
	onMount(() => {
		document.addEventListener("mousedown", handleMousePressOutside);
		return () => document.removeEventListener("mousedown", handleMousePressOutside);
	});
</script>

<div
	class="{!expanded
		? 'max-h-0 overflow-clip'
		: 'max-h-96'} transition-all flex flex-col space-y-2 items-start bg-indigo-300 border-indigo-200 glass-morphism"
	bind:this={container}
>
	<div class="w-full flex flex-col items-start p-4 space-y-2">
		<input
			bind:value={newTask.name}
			bind:this={nameInput}
			on:keydown={onKeyDown}
			placeholder="Task name"
			class="w-full p-4 bg-transparent placeholder:text-white placeholder:text-opacity-25 border-none outline-none text-lg"
		/>
		<div>Est Pomodoros</div>
		<div class="flex max-h-12 space-x-2">
			<input
				bind:value={newTask.estimatedPomodoros}
				on:keydown={onKeyDown}
				type="number"
				required
				class="p-4 w-24 bg-indigo-200 bg-opacity-10 border-opacity-10 border-none active:border-none glass-morphism-basic"
			/>
			<button on:click={() => newTask.estimatedPomodoros++} tabindex="-1"
				>+</button
			>
			<button
				on:click={() =>
					(newTask.estimatedPomodoros = Math.max(
						0,
						newTask.estimatedPomodoros - 1,
					))}
				tabindex="-1">-</button
			>
		</div>
	</div>
	<div
		class="w-full space-x-2 bg-indigo-600 bg-opacity-20 p-4 flex justify-end"
	>
		<Button on:click={handleCreate}>Save</Button>
	</div>
</div>
