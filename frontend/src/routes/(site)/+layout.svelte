<script lang="ts">
  import AccountIcon from "@lucide/svelte/icons/user-round-cog";
  import Button from '$lib/components/ui/button/button.svelte';
  import ShoppingCartIcon from "@lucide/svelte/icons/shopping-basket";
	let { children } = $props();
  import { page } from '$app/state';
  import { cart } from "$lib/storage.svelte";

  let totalProductCount = $derived(cart.products.reduce((sum, p) => sum + p.amount, 0));
</script>

<nav class="fixed items-center h-16 z-10 justify-between text-secondary-foreground top-0 text-2xl p-2 flex w-full bg-background border-b border-primary"> <div class="flex items-center gap-5">
    <Button class="text-3xl bg-primary text-background rounded-md p-2 text-shadow-2xs text-shadow-accent" href="/">
      <p class="text-3xl bg-primary text-background rounded-md p-2 text-shadow-2xs text-shadow-accent">Konsfekt</p>
    </Button>
    <p class="hidden md:flex text-card-foreground">Konsulatets godisskåp app</p>
  </div>
  <div class="flex gap-3">
    {#if cart.products.length > 0}
      <Button variant="default" size="icon" class="size-12 relative" href="/kassa">
        <div class="flex justify-center items-center absolute bottom-[-0.5em] right-[-0.5em] w-[1.5em] h-[1.5em] rounded-md bg-accent ">
          {totalProductCount}
        </div>
        <ShoppingCartIcon class="size-8 text-card-foreground"/>
      </Button>
    {/if}

    <Button variant="secondary" size="icon" class="{page.url.pathname.replaceAll("/", "") ? 'hidden' : ''} size-12" href="/profil">
      <AccountIcon class="size-8 text-card-foreground"/>
    </Button>

  </div>
</nav>
<div class="w-4/5 pt-20">
  {@render children?.()}
</div>
