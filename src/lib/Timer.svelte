<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";
    import Button from "./Button.svelte";
    import { LONG_BREAK, POMODORO, SHORT_BREAK, states } from "./timer";
    import TextButton from "./TextButton.svelte";

    export let on = true;

    const dispatch = createEventDispatcher();

    let intervalID = 0;
    let time = 0;
    let currentState = 0;
    let pomodoroNumber = 1;

    function updateTime() {
        time++;
        if (time >= states[currentState].time) {
            moveToNextState();
        }
    }

    function moveToNextState() {
        if (currentState == POMODORO && pomodoroNumber % 4 != 0) {
            currentState = SHORT_BREAK;
        } else if (currentState == POMODORO && pomodoroNumber % 4 == 0) {
            currentState = LONG_BREAK;
        } else {
            currentState = POMODORO;
            pomodoroNumber++;
            dispatch("pomodoroEnd");
        }
        time = 0;
    }
    function setState(newState: number) {
        currentState = newState;
        time = 0;
    }

    $: formattedTime = ((): string => {
        const seconds = time % 60;
        const minutes = (time - seconds) / 60;
        let result: string = (minutes < 10 ? "0" : "") + minutes;
        result += (seconds < 10 ? ":0" : ":") + seconds;
        return result;
    })();

    onMount(() => {
        if (on)
            intervalID = setInterval(updateTime, 1000);
        return () => intervalID && clearInterval(intervalID);
    });

    const toggleTimer = () => {
        if (on) {
            on = false;
            clearInterval(intervalID);
        } else {
            on = true;
            intervalID = setInterval(updateTime, 1000);
        }
    };
</script>

<div
    class="p-8 bg-indigo-300 items-center justify-center rounded-md bg-clip-padding backdrop-filter backdrop-blur-[2px] bg-opacity-20 border border-indigo-100 border-opacity-20 shadow-lg"
>
    <div class="flex justify-center space-x-4">
        {#each states as state, i (state.name)}
            {#if i == currentState}
                <TextButton class="bg-gray-600 bg-opacity-40"><b>{state.name}</b></TextButton>
            {:else}
                <TextButton on:click={() => setState(i)}>{state.name}</TextButton>
            {/if}
        {/each}
    </div>
    <span class="pb-8 text-3xl bg-white bg-opacity-20">
        <h1>{formattedTime}</h1>
    </span>
    <div class="text-opacity-60 text-gray-100">
        #{pomodoroNumber}
    </div>

    <div class="flex justify-between">
        <div></div>
        <Button on:click={toggleTimer}>{on ? "Pause" : "Resume"}</Button>
        <Button on:click={moveToNextState}>Skip</Button>
    </div>
</div>
