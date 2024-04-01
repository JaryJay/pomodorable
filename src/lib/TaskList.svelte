<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import Button from "./Button.svelte";
    import CreateTaskForm from "./CreateTaskForm.svelte";
    import TaskCard from "./TaskCard.svelte";
    import type { Task } from "./tasks";
    import { completedPomodoroCount } from "./store";

    let currentTask: Task | null = null;
    let openedTask: Task | null = null;
    let draggedTaskIndex: number = -1;
    let tasks: Task[] = [];

    async function loadUserSettings() {
        invoke("load_tasks")
            .then((result: unknown) => {
                if (result) tasks = JSON.parse(result as string) as Task[];
            })
            .catch(console.error);
    }

    function save() {
        invoke("save_data", { data: JSON.stringify(tasks) });
    }

    onMount(() => {
        loadUserSettings();
        completedPomodoroCount.subscribe((value) => {
            if (currentTask) currentTask.completedPomodoros++;
        });
    });

    function addTask(task: any) {
        if (tasks.length == 0) {
            currentTask = task;
        }
        tasks = [...tasks, task as Task];
        save();
    }

    function deleteTaskCard(index: number) {
        tasks = tasks.filter((_, i) => i !== index);
        save();
    }

    function onDragOver(i: number) {
        if (draggedTaskIndex == -1) return;
        // Move the dragged task to the new position
        if (draggedTaskIndex < i) {
            tasks = [
                ...tasks.slice(0, draggedTaskIndex),
                ...tasks.slice(draggedTaskIndex + 1, i + 1),
                tasks[draggedTaskIndex],
                ...tasks.slice(i + 1),
            ];
        } else if (draggedTaskIndex > i) {
            tasks = [
                ...tasks.slice(0, i),
                tasks[draggedTaskIndex],
                ...tasks.slice(i, draggedTaskIndex),
                ...tasks.slice(draggedTaskIndex + 1),
            ];
        }
        draggedTaskIndex = i;
    }

    let expandNewTaskForm = false;
</script>

<div class="p-4 md:p-6 lg:p-8 space-y-4 bg-indigo-300 border-indigo-100 glass-morphism" class:cursor-grabbing={draggedTaskIndex != -1}>
    <div class="flex justify-between w-full">
        <h2 class="text-lg">Task List</h2>
        <div><button>.</button></div>
    </div>
    <!-- svelte-ignore a11y-incorrect-aria-attribute-type -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="space-y-4" on:dragover|preventDefault on:dragenter|preventDefault>
        {#each tasks as task, i (task.id)}
            <TaskCard
                on:click={() => (currentTask = task)}
                selected={task == currentTask}
                bind:task
                on:delete={() => deleteTaskCard(i)}
                on:complete={save}
                on:open={() => (openedTask = task)}
                on:close={() => (openedTask = null)}
                opened={openedTask == task}
                dragged={draggedTaskIndex == i}
                on:dragstart={() => (draggedTaskIndex = i) && (openedTask = null)}
                on:dragend={() => (draggedTaskIndex = -1) && save()}
                on:dragover={() => onDragOver(i)}
            />
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
