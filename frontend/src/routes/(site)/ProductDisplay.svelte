<script lang="ts">
  import ShoppingCartIcon from "@lucide/svelte/icons/shopping-basket";
  import FlagIcon from "@lucide/svelte/icons/flag";
  import Button from "$lib/components/ui/button/button.svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { backendPOST } from "$lib/utils";
    import { toast } from "svelte-sonner";

  let { product, addedToCart = $bindable(0) } = $props();

  async function markSoldOut() {
    let response = await backendPOST("/mark_sold_out", { id: Number(product.id) }, true);
    if (response.ok) {
      toast.success(product.name + " rapporterad som slutsåld.");
    } else {
      toast.error("Kunde inte rapportera slutsåld: " + response.statusText);
    }
    popupOpen = false;
  }
  let popupOpen = $state(false);
</script>

<div class="flex relative flex-col bg-card p-2 rounded-md border">
    <p class="truncate text-left font-bold">{product.name}</p>
    <div class="relative overflow-hidden rounded-xl">
      <Dialog.Root bind:open={popupOpen}>
       <Dialog.Trigger class="bg-background shadow-xs hover:bg-accent dark:bg-input/30 dark:border-input dark:hover:bg-input/50 border
                              absolute rounded-md right-0 top-0 m-1 hover:text-red-500 p-1"> 
          <FlagIcon/>
       </Dialog.Trigger>
       <Dialog.Content>
        <Dialog.Header>
         <Dialog.Title>Rapportera att {product.name} är slut</Dialog.Title>
         <Dialog.Description>
          Genom att rapportera att produkten är slut kan godisskåpsansvariga snabbare se
          att produkten behöver fyllas på.
         </Dialog.Description>
        </Dialog.Header>
        <Button variant="destructive" onclick={() => markSoldOut()}>Rapportera slutsåld</Button>
       </Dialog.Content>
      </Dialog.Root>
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
