<script lang="ts">
	import Button from "./Button.svelte";
	import CreateTaskForm from "./CreateTaskForm.svelte";
	import TaskCard from "./TaskCard.svelte";
	import type { Task } from "./tasks";

	let tasks: Task[] = [];
	function addTask(task: any) {
		tasks = [...tasks, task as Task];
	}

	let expandNewTaskForm = false;
</script>

<div class="p-8 space-y-4 bg-indigo-300 border-indigo-100 glass-morphism">
	<div class="flex justify-between w-full">
		<h2 class="text-lg">Task List</h2>
		<div><button>.</button></div>
	</div>
	<div class="space-y-4 transition-all">
		{#each tasks as task (task.name)}
			<TaskCard bind:task />
		{/each}
		<CreateTaskForm
			on:create={(event) => addTask(event.detail)}
			bind:expanded={expandNewTaskForm}
		/>
		{#if !expandNewTaskForm}
			<Button
				on:click={(event) => {
					event.stopPropagation();
					expandNewTaskForm = true;
				}}
				class="w-full items-center"
			>
				Add Task
			</Button>
		{/if}
	</div>
</div>
