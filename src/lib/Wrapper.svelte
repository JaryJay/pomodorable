<script lang="ts">
    import TaskList from "./TaskList.svelte";
    import Timer from "./Timer.svelte";
</script>

<div>
    <Timer on:pomodoroEnd={() => {}} />
    <TaskList />
</div>
