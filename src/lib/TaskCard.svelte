<script lang="ts">
	import type { Task } from "./tasks";
	import Button from "./Button.svelte";
	import MaterialSymbolsCheckCircleRounded from '~icons/material-symbols/check-circle-rounded';
	import MaterialSymbolsMoreVert from '~icons/material-symbols/more-vert';
	import TextButton from "./TextButton.svelte";
	import { createEventDispatcher } from "svelte";

	const dispatch = createEventDispatcher();

	export let task: Task;
	let editTaskCopy: Task = JSON.parse(JSON.stringify(task));

	let expanded = false;

	let nameInput: HTMLInputElement;
	let container: HTMLDivElement;

	function onKeyDown(event: KeyboardEvent) {
		if (event.key == "Enter") {
			expanded = false;
		}
	}

	function onDelete() {
		dispatch("delete");
	}

	function onCancel() {
		expanded = false;
		editTaskCopy = JSON.parse(JSON.stringify(task));
	}

	function onSave() {
		expanded = false;
		task = editTaskCopy;
		editTaskCopy = JSON.parse(JSON.stringify(task));
	}
</script>

<div class="transition-all glass-morphism">
	{#if expanded}
	<div class="flex flex-col space-y-2 items-start">
		<div class="w-full flex flex-col items-start p-4 space-y-2">
			<input
				bind:value={editTaskCopy.name}
				bind:this={nameInput}
				on:keydown={onKeyDown}
				placeholder="Task name"
				class="w-full p-4 bg-transparent placeholder:text-white placeholder:text-opacity-25 border-none outline-none text-lg"
			/>
			<div>Est Pomodoros</div>
			<div class="flex max-h-12 space-x-2">
				<input
					bind:value={editTaskCopy.estimatedPomodoros}
					on:keydown={onKeyDown}
					type="number"
					required
					class="p-4 w-24 bg-indigo-200 bg-opacity-10 border-opacity-10 border-none active:border-none glass-morphism-basic"
				/>
				<button on:click={() => editTaskCopy.estimatedPomodoros++} tabindex="-1"
					>+</button
				>
				<button
					on:click={() =>
						(editTaskCopy.estimatedPomodoros = Math.max(
							0,
							editTaskCopy.estimatedPomodoros - 1,
						))}
					tabindex="-1">-</button
				>
			</div>
		</div>
		<div
			class="w-full space-x-2 bg-indigo-600 bg-opacity-20 p-4 flex justify-between"
		>
			<TextButton on:click={onDelete} class="text-white text-opacity-70 hover:text-opacity-100">Delete</TextButton>
			<div class="space-x-2">
				<TextButton on:click={onCancel} class="text-white text-opacity-70 hover:text-opacity-100">Cancel</TextButton>
				<Button on:click={onSave}>Save</Button>
			</div>
		</div>
	</div>
	{:else}
		<div
			class="flex p-4 space-x-2 justify-between bg-indigo-300 rounded-md bg-clip-padding backdrop-filter backdrop-blur-[2px] bg-opacity-40 border border-indigo-200 border-opacity-40 shadow-lg"
		>
			<div class="flex space-x-2">
				<button on:click={() => (task.done = !task.done)}>
					<MaterialSymbolsCheckCircleRounded
						class="text-6 hover:scale-120 text-white transition-all {task.done
							? 'hover:text-opacity-95'
							: 'text-opacity-25 hover:text-opacity-50'}"
					/>
				</button>

				<div>
					<p>{task.name}</p>
				</div>
			</div>
			<div class="flex space-x-2 align-middle">
				<p>{task.completedPomodoros} / {task.estimatedPomodoros}</p>
				<button
					on:click={() => expanded = true}
					class="hover:bg-indigo-300 rounded-full bg-opacity-40 transition-all"
				>
					<MaterialSymbolsMoreVert class="text-6" />
				</button>
			</div>
		</div>
	{/if}
</div>
