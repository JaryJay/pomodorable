<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import Button from "./Button.svelte";
    import CreateTaskForm from "./CreateTaskForm.svelte";
    import TaskCard from "./TaskCard.svelte";
    import type { Task } from "./tasks";
    import { completedPomodoroCount } from "./store";

    let currentTask: Task | null = null;
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
            console.log({ value });
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

    let expandNewTaskForm = false;
</script>

<div class="p-8 space-y-4 bg-indigo-300 border-indigo-100 glass-morphism">
    <div class="flex justify-between w-full">
        <h2 class="text-lg">Task List</h2>
        <div><button>.</button></div>
    </div>
    <div class="space-y-4 transition-all">
        {#each tasks as task, i (task.name)}
            <TaskCard bind:task on:delete={() => deleteTaskCard(i)} on:complete={save} />
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
