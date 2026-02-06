<script lang="ts">
  import { Toaster } from "$lib/components/ui/sonner/index.js";
  import AccountIcon from "@lucide/svelte/icons/user-round-cog";
  import Button from '$lib/components/ui/button/button.svelte';
  import ShoppingCartIcon from "@lucide/svelte/icons/shopping-basket";
  import { page } from '$app/state';
  import { cart } from "$lib/storage.svelte";
  import { onMount } from "svelte";
  import LogoButton from "$lib/components/LogoButton.svelte";

  let { children, data } = $props();

  let totalProductCount = $derived(
    Object.entries(cart.products).reduce((sum, [, n]) => sum + n, 0)
  );

  let cartTotal = $derived(
    Object.entries(cart.products).reduce((sum, [id, qty]) => {
      const product = data.products.find((p: { id: string }) => String(p.id) === id);
      return sum + (product?.price ?? 0) * qty;
    }, 0)
  );

  let hasEnoughMoney = $derived(data.user.balance >= cartTotal);
  let isProfilePage = $derived(page.url.pathname.startsWith("/profil"));

  onMount(() => {
    let localCart = localStorage.getItem("cart");
    if (localCart) {
      cart.products = JSON.parse(localCart);
    }
  })

  $effect(() => {
    let stringCart = JSON.stringify(cart.products);
    localStorage.setItem("cart", stringCart);
  })
</script>

<Toaster />

<nav class="fixed items-center h-16 z-10 justify-between text-secondary-foreground top-0 text-2xl p-2 flex w-full bg-background border-b border-primary">
  <div class="flex items-center gap-5">
    <LogoButton />
    <p class="hidden md:flex text-card-foreground">Konsulatets godisskåp app</p>
  </div>
  <div class="flex gap-3 items-center">
    {#if totalProductCount > 0}
      <span class="text-2xl font-mono font-semibold {hasEnoughMoney ? 'text-card-foreground' : 'text-red-500'}">
        {cartTotal}kr
      </span>
    {/if}
    {#if totalProductCount > 0}
      <Button variant="default" size="icon" class="size-12 relative" href="/kassa">
        <div class="flex justify-center items-center absolute bottom-[-0.5em] right-[-0.5em] w-[1.5em] h-[1.5em] rounded-md bg-accent">
          {totalProductCount}
        </div>
        <ShoppingCartIcon class="size-8 text-card-foreground"/>
      </Button>
    {/if}

    <Button
      variant="secondary"
      size="icon"
      class="{isProfilePage ? 'hidden' : ''} size-12"
      href="/profil">
      <AccountIcon class="size-8 text-card-foreground"/>
    </Button>
  </div>
</nav>
<div class="w-4/5 pt-20">
  {@render children?.()}
</div>
