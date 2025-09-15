<script lang="ts">
    import { onMount } from "svelte";
    import TaskList from "./TaskList.svelte";
    import Timer from "./Timer.svelte";
    import { register, unregister } from "@tauri-apps/api/globalShortcut";

    onMount(() => {
        (async () => {
            await unregister("CommandOrControl+Alt+Shift+P")
            await register("CommandOrControl+Alt+Shift+P", () => {
                console.log("Shortcut triggered");
            })
            console.log("Registered shortcut.")
        })()

        return () => unregister("CommandOrControl+Alt+Shift+P")
    })
    
</script>

<div
    class="lg:space-y-8 md:space-y-6 space-y-4 lg:p-8 md:p-6 p-0 transition-all"
>
    <Timer />
    <TaskList />
</div>
