<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";
    import {
        isPermissionGranted,
        requestPermission,
        sendNotification,
    } from "@tauri-apps/api/notification";
    import Button from "./Button.svelte";
    import MaterialSymbolsSkipNextRounded from "~icons/material-symbols/skip-next-rounded";
    import MaterialSymbolsSettings from '~icons/material-symbols/settings';
    import { LONG_BREAK, POMODORO, SHORT_BREAK, states } from "./timer";
    import TextButton from "./TextButton.svelte";
    import { completedPomodoroCount } from "./store";
    import { settings } from "./settings";

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
        if (currentState == POMODORO) {
            completedPomodoroCount.update((c) => c + 1);

            if (pomodoroNumber % 4 != 0) {
                sendNotification({
                    title: "Time's up!",
                    body: "Time to take a short break.",
                    sound: "IM",
                });
                currentState = SHORT_BREAK;
            } else if (pomodoroNumber % 4 == 0) {
                sendNotification({
                    title: "Time's up!",
                    body: "Time to take a long break.",
                    sound: "IM",
                });
                currentState = LONG_BREAK;
            }
        } else {
            sendNotification({
                title: "Break's over!",
                body: "Time to start your pomodoro.",
                sound: "Reminder",
            });
            currentState = POMODORO;
            pomodoroNumber++;
        }
        time = 0;
    }
    function setState(newState: number) {
        currentState = newState;
        time = 0;
    }

    $: formattedTime = ((): string => {
        const displayTime = settings.countdown
            ? states[currentState].time - time
            : time;
        const seconds = displayTime % 60;
        const minutes = (displayTime - seconds) / 60;
        let result: string = (minutes < 10 ? "0" : "") + minutes;
        result += (seconds < 10 ? ":0" : ":") + seconds;
        return result;
    })();

    onMount(() => {
        if (on) intervalID = setInterval(updateTime, 1000);
        return () => intervalID && clearInterval(intervalID);
    });

    onMount(async () => {
        let permissionGranted = await isPermissionGranted();

        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === "granted";
        }
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

    const openSettings = () => {

    }
</script>

<div
    class="space-y-2 p-8 bg-indigo-300 items-center justify-center rounded-md bg-clip-padding backdrop-filter backdrop-blur-[2px] bg-opacity-20 border border-indigo-100 border-opacity-20 shadow-lg"
>
    <div class="flex justify-center space-x-4 mb-4">
        {#each states as state, i (state.name)}
            {#if i == currentState}
                <TextButton class="bg-gray-600 bg-opacity-40"
                    ><b>{state.name}</b></TextButton
                >
            {:else}
                <TextButton on:click={() => setState(i)}
                    >{state.name}</TextButton
                >
            {/if}
        {/each}
    </div>
    <span class="text-3xl bg-white bg-opacity-20">
        <h1>{formattedTime}</h1>
    </span>
    <div class="text-opacity-60 text-gray-100">
        #{pomodoroNumber}
    </div>

    <div class="grid grid-cols-3 px-4 gap-20">
        <div />
        <Button on:click={toggleTimer} class=""
            >{on ? "Pause" : "Resume"}</Button
        >
        <div class="flex align-middle justify-end">
            <TextButton on:click={openSettings}>
                <MaterialSymbolsSettings />
            </TextButton>
            <TextButton on:click={moveToNextState}>
                <MaterialSymbolsSkipNextRounded
                    class="text-lg opacity-60 hover:opacity-80 transition-all"
                />
            </TextButton>
        </div>
    </div>
</div>
