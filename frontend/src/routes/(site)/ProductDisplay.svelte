<script lang="ts">
  import ShoppingCartIcon from "@lucide/svelte/icons/shopping-basket";
  import Button from "$lib/components/ui/button/button.svelte";

  let { product, addedToCart = $bindable(0) } = $props();
</script>

<div class="flex relative flex-col bg-card p-2 rounded-md border">
    <p class="truncate text-left font-bold">{product.name}</p> 
    <div class="overflow-hidden rounded-xl">
      <img
       src="/uploads/images/product/{product.id}.webp"
       alt={product.description}
       class="aspect-square object-contain"
      />
    </div>
    <div class="flex justify-center w-full">

    <span>{product.price} kr</span>
    </div>
    <div class="flex justify-between items-center">
      {#if addedToCart == 0}
        <Button disabled>-</Button>
      {:else}
        <Button onclick={() => addedToCart--}>-</Button>
        <div class="flex gap-1 justify-center">
          <ShoppingCartIcon/>
          <span>{addedToCart}</span>
        </div>
      {/if}
      <Button onclick={() => addedToCart++}>+</Button>
    </div>
    {#if addedToCart}
      <div class="m-2 absolute top-0 right-0">
      </div>
    {/if}
</div>
