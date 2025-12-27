<script lang="ts">
  import ShoppingCartIcon from "@lucide/svelte/icons/shopping-basket";
  import RemoveIcon from "@lucide/svelte/icons/trash";
  import Button from "$lib/components/ui/button/button.svelte";

  let { product, addedToCart = $bindable(0) } = $props();
</script>

<div class="grid grid-cols-[auto_1fr] gap-3 w-full md:w-4/5 bg-card p-2 rounded-md border">
  <div class="overflow-hidden rounded-xl">
    <img
     src="/uploads/images/product/{product.id}.webp"
     alt={product.description}
     class="aspect-square w-20 object-contain"
    />
  </div>
  <div class="flex flex-col justify-between">
    <p class="wrap-normal text-left font-bold">{product.name}</p> 
    <div class="flex flex-col">
      <div class="flex justify-between">
        <div class="flex md:gap-1 md:flex-row flex-col">
          <span class="text-3xl font-mono font-semibold">{product.price}kr</span> 
        </div>
        <div class="flex items-center gap-3">
          {#if addedToCart == 1}
            <Button onclick={() => addedToCart--}>
              <RemoveIcon class="text-card-foreground"/>
            </Button>
          {:else}
            <Button onclick={() => addedToCart--}>-</Button>
          {/if}
          <div class="flex gap-1 justify-center">
            <ShoppingCartIcon/>
            <span>{addedToCart}</span>
          </div>
          <Button onclick={() => addedToCart++}>+</Button>
        </div>
      </div>

    </div>
  </div>
</div>
