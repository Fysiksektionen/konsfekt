<script lang="ts">
  import ProductScroller from '$lib/components/ProductScroller.svelte';
  import Button from '$lib/components/ui/button/button.svelte';
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
    )
  }
</script>


<div class="flex flex-col w-full items-center gap-5">
  <div class="flex w-4/5 max-w-[400px]">
  </div>
    <div class="w-4/5 flex justify-between gap-3 p-4 bg-card text-card-foreground items-center rounded-xl border-primary border-4 max-w-[400px]">
      <p class="text-2xl text-card-foreground">Saldo</p>
      <span class="text-5xl font-mono font-semibold">{data.user.balance}kr</span> 
    </div>

  <Button onclick={() => debug_add_money()} class="text-2xl text-card-foreground" variant="secondary">Lägg till pengar</Button>
  
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
        <ProductDisplay {product} bind:addedToCart={cart.products[product.id]}/>
      {/each}
    </div>
  </div>
</div>
