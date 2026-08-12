<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { Input } from '$lib/components/ui/input/index.js';
  import SearchIcon from '@lucide/svelte/icons/search';
  import Footer from '$lib/components/Footer.svelte';
	import type { PageProps } from './$types';
  import ProductDisplay from './ProductDisplay.svelte';
  import { cart } from '$lib/storage.svelte';
  import { toast } from "svelte-sonner";
    import { backendPOST } from '$lib/utils';
    import { invalidateAll } from '$app/navigation';

	let { data }: PageProps = $props();

  data.products.forEach((p: {id: string}) => {
    if (!cart.products[p.id]) {
      cart.products[p.id] = 0;
    }
  })

  let searchTerm = $state("");
  let filteredProducts = $state(data.products);

  function onSearchInput(e: Event) {
    const value = (e.currentTarget as HTMLInputElement).value;
    searchTerm = value;
    const term = value.trim().toLowerCase();
    filteredProducts = term === ""
      ? data.products
      : data.products.filter((p: { name: string, description: string }) =>
          p.name.toLowerCase().includes(term) || p.description?.toLowerCase().includes(term)
        );
  }

  let popularProducts = $derived(data.products.filter(p => p.flags.popular));

  async function debug_add_money() {
    toast.promise(
      backendPOST("/debug/add_money", { amount: 500 }, true),
      {
        loading: "Lägger till saldo",
        success: () => {
          invalidateAll();
          return "500 kr tillagt"
        },
        error: "Något gick fel"
      }
    );
  }
</script>


<div class="flex w-full flex-col items-center gap-8 pb-10">
  <!-- Balance -->
  <div class="relative w-full max-w-2xl overflow-hidden rounded-3xl border border-primary/40 bg-gradient-to-br from-primary/25 via-card to-secondary/20 p-6 shadow-lg">
    <div class="pointer-events-none absolute -top-16 -right-16 size-48 rounded-full bg-primary/30 blur-3xl"></div>
    <div class="pointer-events-none absolute -bottom-20 -left-10 size-40 rounded-full bg-secondary/30 blur-3xl"></div>

    <div class="relative flex flex-col gap-5">
      <div class="flex flex-wrap items-end justify-between gap-4">
        <div class="flex flex-col gap-1">
          <span class="text-sm tracking-wide text-muted-foreground uppercase">Saldo</span>
          <span class="font-mono text-6xl font-bold tracking-tight text-card-foreground">{data.user.balance}<span class="text-3xl font-semibold">kr</span></span>
        </div>
        <Button href="/swish" size="lg" class="h-12 rounded-xl px-6 text-lg font-semibold text-card-foreground shadow" variant="secondary">
          + Lägg till pengar
        </Button>
      </div>
    </div>
  </div>

  <!-- Popular products -->
  {#if popularProducts.length > 0}
    <div class="flex w-full max-w-5xl flex-col items-start gap-3">
      <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">Populära produkter</h3>
      <div class="grid w-full grid-cols-2 gap-4 sm:grid-cols-4">
        {#each popularProducts as product (product.id)}
          <ProductDisplay user={data.user} {product} bind:addedToCart={cart.products[product.id]}/>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Search + all products -->
  <div class="flex w-full max-w-5xl flex-col items-start gap-4">
    <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">Alla produkter</h3>

    <div class="sticky top-16 z-10 w-full py-3 px-2">
      <div class="relative">
        <SearchIcon class="absolute left-4 top-1/2 size-5 -translate-y-1/2 text-muted-foreground"/>
        <Input
          type="text"
          placeholder="Sök bland alla produkter..."
          class="h-12 w-full rounded-full border-2 bg-card pl-11 pr-4 text-base shadow-sm focus-visible:ring-4 dark:bg-card"
          value={searchTerm}
          oninput={onSearchInput}
        />
      </div>
    </div>

    {#if filteredProducts.length === 0}
      <div class="flex w-full flex-col items-center gap-2 py-16 text-center text-muted-foreground">
        <span class="text-4xl">🔍</span>
        <p>Inga produkter matchade "{searchTerm}".</p>
      </div>
    {:else}
      <div class="grid w-full grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4">
        {#each filteredProducts as product (product.id)}
          <ProductDisplay user={data.user} {product} bind:addedToCart={cart.products[product.id]}/>
        {/each}
      </div>
    {/if}
  </div>
</div>

<Footer/>
