<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>


<div class="flex flex-col w-full items-center pt-15 gap-5">
  <div class="flex w-4/5 max-w-[400px]">
  </div>
  <div class="w-4/5 flex justify-between gap-3 p-4 bg-card text-card-foreground items-center rounded-xl border-primary border-4 max-w-[400px]">
    <p class="text-2xl text-card-foreground">Saldo</p>
    <span class="text-5xl font-mono font-semibold">593kr</span> 
  </div>
  <Button class="text-2xl text-card-foreground" variant="secondary">Lägg till pengar</Button>

  <div>

  </div>
</div>

{data.user.email}
