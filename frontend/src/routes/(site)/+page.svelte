<script lang="ts">
  import ProductScroller from '$lib/components/ProductScroller.svelte';
  import Button from '$lib/components/ui/button/button.svelte';
	import type { PageProps } from './$types';
  import ProductDisplay from './ProductDisplay.svelte';
    import { cart } from '$lib/storage.svelte';

	let { data }: PageProps = $props();
  type Product = {
    id: string
  }
  const productsInCart = $state<Record<string, number>>(Object.fromEntries(data.products.map((p: Product) => [p.id, 0])));
  $effect(() => {
    cart.products = Object.entries(productsInCart)
                          .filter(([_,inCart]) => inCart > 0)
                          .map(([id, amount]) => ({id, amount}));
  })
</script>


<div class="flex flex-col w-full items-center gap-5">
  <div class="flex w-4/5 max-w-[400px]">
  </div>
    <div class="w-4/5 flex justify-between gap-3 p-4 bg-card text-card-foreground items-center rounded-xl border-primary border-4 max-w-[400px]">
      <p class="text-2xl text-card-foreground">Saldo</p>
      <span class="text-5xl font-mono font-semibold">{data.user.balance}kr</span> 
    </div>

  <Button class="text-2xl text-card-foreground" variant="secondary">Lägg till pengar</Button>
  
  <!--
  <div class="flex w-full flex-col items-center gap-2">
    <div class="flex w-full justify-start">
      <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">Nyligen köpta produkter</h3>
    </div>
    <ProductScroller/>
  </div>
  -->

  

  <div class="flex w-full flex-col items-center gap-2">
    <div class="flex w-full justify-start">
      <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">Alla produkter</h3>
    </div>
    <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mt-3">
      {#each data.products as product}
        <ProductDisplay {product} bind:addedToCart={productsInCart[product.id]}/>
      {/each}
    </div>
  </div>
</div>
