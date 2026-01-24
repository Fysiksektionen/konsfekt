<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { cart } from "$lib/storage.svelte";
  import type { PageProps } from './$types';
  import SadIcon from "@lucide/svelte/icons/frown";
    import CartProductDisplay from './CartProductDisplay.svelte';
    import { backendPOST } from '$lib/utils';
    import { toast } from 'svelte-sonner';
    import { goto, invalidateAll } from '$app/navigation';
	let { data }: PageProps = $props();
  type Product = { id: string }
  let productsInCart = $derived(data.products.filter((p: Product) => (p.id in cart.products) && cart.products[p.id] > 0))
  let total = $derived(productsInCart.reduce((sum: number, p: Product) => sum + p.price * cart.products[p.id], 0))

  async function buyProducts() {
    let cartArray = [];
    for (const [id, quantity] of Object.entries(cart.products)) {
      if (quantity > 0) {
        cartArray.push({ id: Number(id), quantity })
      }
    }
    let response = await backendPOST("/buy_products", { products: cartArray }, true);
    if (response.ok) {
      const spent = total;
      await goto("/");
      invalidateAll();
      cart.products = {};
      toast.success(`Ditt köp på ${spent}kr har genomförts`)
    }
  }

  let hasEnoughMoney = $derived(data.user.balance >= total);
</script>
{#snippet productSum()}
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
{/snippet}

<div class="flex flex-col w-full justify-center items-center gap-3">
  {#each productsInCart as product}
    <CartProductDisplay {product} bind:addedToCart={cart.products[product.id]}/> 
  {/each}
  {#if productsInCart.length > 0}
    <div class="h-56">
    </div>
    <div class="fixed flex flex-col md:grid grid-cols-2 bg-background p-3 rounded-t-xl border-4 border-b-0 border-primary bottom-0 h-fit w-4/5">
      <div class="hidden md:flex flex-col">
        <p class="text-2xl text-card-foreground">Total</p>
        <span class="text-5xl font-mono font-semibold">{total}kr</span> 
        <div class="flex gap-1 flex-wrap">
          =
          {@render productSum()}
        </div>
      </div>
      <div class="flex flex-col gap-3 justify-center items-center">
        {#if hasEnoughMoney}
          <Button onclick={() => buyProducts()} class="md:scale-200 scale-150">
            <p class="text-card-foreground">Betala</p>
            <span class="font-mono font-semibold">{total}kr</span> 
          </Button>
        {:else}
          <Button disabled class="md:scale-200 scale-150">
            Otillräckligt saldo ({data.user.balance}kr)
          </Button>
        {/if}
        <div class="flex gap-1 flex-wrap md:hidden">
            ( {@render productSum()} )
        </div>
      </div>
    </div>
  {:else}
    <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">Din varukorg är tom</h3>
    <SadIcon size="140"/>
    <Button href="/" variant="secondary">Lägg till produkter</Button>
  {/if}
</div>



