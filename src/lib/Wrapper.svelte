<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { exists } from "@tauri-apps/api/fs";
	import { appConfigDir } from "@tauri-apps/api/path";
	import { CloseRequestedEvent, appWindow } from "@tauri-apps/api/window";
	import { setContext } from "svelte";

	setContext("startingTasks", []);

	async function loadUserSettings() {
		invoke("load_tasks")
			.then((result: unknown) => {
				setContext("startingTasks", JSON.parse(result as string));
			})
			.catch(console.error);
	}

	// appWindow.onCloseRequested(async (event: CloseRequestedEvent) => {
	// 	await invoke("save_data");
	// 	// event.preventDefault();
	// });
</script>
