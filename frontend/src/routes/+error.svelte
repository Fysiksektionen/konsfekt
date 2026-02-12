<script lang="ts">
  import { page } from '$app/state';
  import Button from '$lib/components/ui/button/button.svelte';
</script>

<div class="flex flex-col items-center justify-center gap-6 flex-1 p-8">
  <p class="text-[8rem] font-mono font-bold leading-none text-primary">{page.status}</p>
  <p class="text-2xl text-muted-foreground">{page.error?.message}</p>
  <Button href="/" variant="secondary" class="text-lg">Tillbaka till startsidan</Button>
</div>
