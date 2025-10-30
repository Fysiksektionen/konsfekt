<script lang="ts">
  import Account from "@lucide/svelte/icons/user-round-cog";
  import Button from '$lib/components/ui/button/button.svelte';
	let { children } = $props();
  import { page } from '$app/state';
</script>

<nav class="fixed items-center h-16 justify-between text-secondary-foreground top-0 text-2xl p-2 flex w-full bg-background border-b border-primary"> <div class="flex items-center gap-5">
    <Button class="text-3xl bg-primary text-background rounded-md p-2 text-shadow-2xs text-shadow-accent" href="/">
      <p class="text-3xl bg-primary text-background rounded-md p-2 text-shadow-2xs text-shadow-accent">Konsfekt</p>
    </Button>
    <p class="hidden md:flex text-card-foreground">Konsulatets godisskåp app</p>
  </div>
  <Button variant="secondary" size="icon" class="{page.url.pathname.replaceAll("/", "") ? 'hidden' : ''} size-12" href="/profil">
    <Account class="size-8 text-card-foreground"/>
  </Button>
</nav>
<div class="w-4/5 pt-20">
  {@render children?.()}
</div>
