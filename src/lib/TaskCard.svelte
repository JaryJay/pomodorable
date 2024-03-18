<script lang="ts">
	import CheckCircle from "./icons/CheckCircle.svelte";
	import type { Task } from "./tasks";
	import Button from "./Button.svelte";
	export let task: Task;

	let expanded = false;

	let nameInput: HTMLInputElement;
	let container: HTMLDivElement;

	function onKeyDown(event: KeyboardEvent) {
		if (event.key == "Enter") {
			expanded = false;
		}
	}
</script>

<div class="transition-all glass-morphism">
	{#if expanded}
	<div class="flex flex-col space-y-2 items-start">
		<div class="w-full flex flex-col items-start p-4 space-y-2">
			<input
				bind:value={task.name}
				bind:this={nameInput}
				on:keydown={onKeyDown}
				placeholder="Task name"
				class="w-full p-4 bg-transparent placeholder:text-white placeholder:text-opacity-25 border-none outline-none text-lg"
			/>
			<div>Est Pomodoros</div>
			<div class="flex max-h-12 space-x-2">
				<input
					bind:value={task.estimatedPomodoros}
					on:keydown={onKeyDown}
					type="number"
					required
					class="p-4 w-24 bg-indigo-200 bg-opacity-10 border-opacity-10 border-none active:border-none glass-morphism-basic"
				/>
				<button on:click={() => task.estimatedPomodoros++} tabindex="-1"
					>+</button
				>
				<button
					on:click={() =>
						(task.estimatedPomodoros = Math.max(
							0,
							task.estimatedPomodoros - 1,
						))}
					tabindex="-1">-</button
				>
			</div>
		</div>
		<div
			class="w-full space-x-2 bg-indigo-600 bg-opacity-20 p-4 flex justify-end"
		>
			<Button on:click={() => (expanded = false)}>Save</Button>
		</div>
	</div>
	{:else}
		<div
			class=" flex p-4 space-x-2 justify-between bg-indigo-300 rounded-md bg-clip-padding backdrop-filter backdrop-blur-[2px] bg-opacity-40 border border-indigo-200 border-opacity-40 shadow-lg"
		>
			<div class="flex space-x-2">
				<button on:click={() => (task.done = !task.done)}>
					<CheckCircle
						class="size-6 hover:scale-120 text-white transition-all {task.done
							? 'hover:text-opacity-95'
							: 'text-opacity-25 hover:text-opacity-50'}"
					/>
				</button>

				<div>
					<p>{task.name}</p>
				</div>
			</div>
			<div class="flex space-x-2">
				<p>{task.completedPomodoros} / {task.estimatedPomodoros}</p>
				<button
					on:click={() => (expanded = true)}
					class="hover:bg-indigo-300 rounded-sm bg-opacity-40 transition-all size-6"
				>
					.
					<!--Placeholder-->
				</button>
			</div>
		</div>
	{/if}
</div>
