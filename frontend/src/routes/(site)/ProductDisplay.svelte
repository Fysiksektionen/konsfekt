<script lang="ts">
  import ShoppingCartIcon from "@lucide/svelte/icons/shopping-basket";
  import FlagIcon from "@lucide/svelte/icons/flag";
  import FlagOffIcon from "@lucide/svelte/icons/flag-off";
  import Button from "$lib/components/ui/button/button.svelte";

  let { product, addedToCart = $bindable(0) } = $props();

  let markedSoldOut = $state(false);
  let flagToggleHovered = $state(false);
   
  async function toggleSoldOutMark() {
    markedSoldOut = !markedSoldOut;
  }

</script>

<div class="flex relative flex-col bg-card p-2 rounded-md border">
    <p class="truncate text-left font-bold">{product.name}</p>
    <div class="relative overflow-hidden rounded-xl">
      <Button 
        onclick={() => toggleSoldOutMark()} 
        onmouseenter={() => flagToggleHovered = true}
        onmouseleave={() => flagToggleHovered = false}
        variant="outline" 
        class="absolute right-0 top-0 m-1 hover:text-red-500 {markedSoldOut ? 'text-red-500' : ''}">
        {#if markedSoldOut}
          {#if flagToggleHovered}
            <FlagOffIcon/>
          {:else}
            <FlagIcon/>
          {/if}
        {:else}
          <FlagIcon/>
        {/if}
      </Button>
      <img
       src="/uploads/images/product/{product.id}.webp"
       alt={product.description}
       class="aspect-square object-contain"
      />
    </div>
    <div class="flex justify-center w-full">
      <span class="text-3xl font-mono font-semibold">{product.price}kr</span> 
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
</div>
