<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { cart } from "$lib/storage.svelte";
  import type { PageProps } from './$types';
    import CartProductDisplay from './CartProductDisplay.svelte';
	let { data }: PageProps = $props();
  type Product = { id: string }
  let productsInCart = $derived(data.products.filter((p: Product) => (p.id in cart.products) && cart.products[p.id] > 0))
</script>

<div class="flex flex-col w-full items-center gap-3">
  {#each productsInCart as product}
    <CartProductDisplay {product} bind:addedToCart={cart.products[product.id]}/> 
  {/each}
</div>

