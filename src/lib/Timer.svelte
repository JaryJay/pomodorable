<script lang="ts">
  import { onMount } from "svelte";

  let intervalID = 0;
  export let on = true;
  let time = 0;

  $: formattedTime = ((): string => {
    const seconds = time % 60;
    const minutes = (time - seconds) / 60;
    let result: string = (minutes < 10 ? "0" : "") + minutes;
    result += (seconds < 10 ? ":0" : ":") + seconds;
    return result;
  })();

  onMount(() => {
    if (on) {
      intervalID = setInterval(() => {
        time++;
        console.log(time);
      }, 1000);
    }

    return () => intervalID && clearInterval(intervalID);
  });

  const toggleTimer = () => {
    if (on) {
      on = false;
      clearInterval(intervalID);
    } else {
      on = true;
      intervalID = setInterval(() => {
        time++;
        console.log(time);
      }, 1000);
    }
  };
</script>

<div
  class="p-8 bg-indigo-300 items-center justify-center rounded-md bg-clip-padding backdrop-filter backdrop-blur-[2px] bg-opacity-20 border border-indigo-100 border-opacity-20 shadow-lg"
>
  <span class="pb-8 text-3xl bg-white bg-opacity-20">
    <h1>{formattedTime}</h1>
  </span>

  <button on:click={toggleTimer}>{on ? "Pause" : "Resume"}</button>
</div>
