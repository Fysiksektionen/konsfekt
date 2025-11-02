<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
  import "../app.css";
  import { ModeWatcher } from "mode-watcher";
	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<ModeWatcher />
<div class="flex flex-col items-center bg-background text-foreground h-screen w-full">
  {@render children?.()}
</div>
