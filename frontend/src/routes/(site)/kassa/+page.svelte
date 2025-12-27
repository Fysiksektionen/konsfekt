<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { cart } from "$lib/storage.svelte";
  import type { PageProps } from './$types';
  import SadIcon from "@lucide/svelte/icons/frown";
    import CartProductDisplay from './CartProductDisplay.svelte';
	let { data }: PageProps = $props();
  type Product = { id: string }
  let productsInCart = $derived(data.products.filter((p: Product) => (p.id in cart.products) && cart.products[p.id] > 0))
  let total = $derived(productsInCart.reduce((sum: number, p: Product) => sum + p.price * cart.products[p.id], 0))
</script>

<div class="flex flex-col w-full justify-center items-center gap-3">
  {#each productsInCart as product}
    <CartProductDisplay {product} bind:addedToCart={cart.products[product.id]}/> 
  {/each}
  {#if productsInCart.length > 0}
    <div class="h-56">
    </div>
    <div class="fixed flex flex-col md:grid grid-cols-2 bg-background p-3 rounded-t-xl border-4 border-b-0 border-primary bottom-0 h-fit w-4/5">
      <div>
        <p class="text-2xl text-card-foreground">Total</p>
        <span class="text-5xl font-mono font-semibold">{total}kr</span> 
        <div class="flex gap-1 flex-wrap">
          (
          {#each productsInCart as product, i}
            <div class="flex">
              <span class="font-mono font-semibold">{product.price}</span>
              {#if cart.products[product.id] > 1}
              <span>x</span><span class="font-mono font-semibold">{cart.products[product.id]}</span> 
              {/if}
            </div>
            {#if i < productsInCart.length - 1}
            <span>+</span> 
            {/if}
          {/each}
          )
        </div>
      </div>
      <div class="flex justify-center items-center">
        <Button class="">
          <p class="text-card-foreground">Betala</p>
          <span class="font-mono font-semibold">{total}kr</span> 
        </Button>
      </div>
    </div>
  {:else}
    <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">Din varukorg är tom</h3>
    <SadIcon size="140"/>
    <Button href="/" variant="secondary">Lägg till produkter</Button>
  {/if}
</div>



