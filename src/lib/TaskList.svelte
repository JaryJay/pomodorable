<script lang="ts">
	import CreateTaskForm from "./CreateTaskForm.svelte";
	import TaskCard from "./TaskCard.svelte";
	import type { Task } from "./tasks";

	let tasks: Task[] = [];
	function addTask(task: any) {
		tasks = [...tasks, task as Task];
	}

	let expandNewTaskForm = false;
</script>

<div
	class="p-8 space-y-4 bg-indigo-300 rounded-md bg-clip-padding backdrop-filter backdrop-blur-[2px] bg-opacity-20 border border-indigo-100 border-opacity-20 shadow-lg"
>
	<div class="flex justify-between w-full">
		<h2 class="text-lg">Task List</h2>
		<div><button>.</button></div>
	</div>
	<div class="space-y-4">
		{#each tasks as task (task.name)}
			<TaskCard {task} />
		{/each}
		{#if expandNewTaskForm}
			<CreateTaskForm on:create={(event) => addTask(event.detail)} />
		{:else}
			<button
				on:click={() => (expandNewTaskForm = true)}
				class="w-full items-center"
			>
				Add Task
			</button>
		{/if}
	</div>
</div>
