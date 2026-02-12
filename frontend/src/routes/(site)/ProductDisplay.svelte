<script lang="ts">
  import ShoppingCartIcon from "@lucide/svelte/icons/shopping-basket";
  import FlagIcon from "@lucide/svelte/icons/flag";
  import Button from "$lib/components/ui/button/button.svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { backendPOST } from "$lib/utils";
    import { toast } from "svelte-sonner";
    import { goto, invalidateAll } from "$app/navigation";

  let { product, addedToCart = $bindable(0) } = $props();

  async function markSoldOut() {
    let response = await backendPOST("/mark_sold_out", { id: Number(product.id) }, true);
    if (response.ok) {
      toast.success(product.name + " rapporterad som slutsåld.");
    } else {
      toast.error("Kunde inte rapportera slutsåld: " + response.statusText);
    }
    showSoldOutPage = false;
  }

  async function buyProduct() {
    let response = await backendPOST("/buy_single_product", { id: Number(product.id) }, true);
    if (response.ok) {
      let transactionID = await response.json();
      invalidateAll();
      toast.success(product.name + " köpt.", {
          action: {
            label: "Ångra",
            onClick: () => undoTransaction(transactionID)
          }
        });
    } else if (response.status == 402) {
      toast.error("Du har inte tillräckligt saldo");
    } else {
      toast.error("Kunde inte köpa produkten: " + response.statusText);
    }
  }

  async function undoTransaction(transactionID: { transaction_id: number }) {
    let response = await backendPOST("/undo_transaction", transactionID, true);
    if (response.ok) {
      invalidateAll();
      toast.success("Köp ångrat");
    } else {
      toast.error("kunde inte ångra köp: " + response.statusText);
    }
  }

  let showSoldOutPage = $state(false);
  let productPopup = $state(false);
</script>

<Dialog.Root bind:open={productPopup}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>{product.name}</Dialog.Title>
    </Dialog.Header>
    {#if showSoldOutPage}
      <p>
        Genom att rapportera att produkten är slut kan godisskåpsansvariga snabbare se att produkten behöver fyllas på.
      </p>
      <div class="flex gap-3">
        <Button variant="outline" onclick={() => showSoldOutPage = false}>Tillbaka</Button>
        <Button variant="destructive" onclick={() => markSoldOut()}>Rapportera slutsåld</Button>
      </div>
    {:else}
      <div class="flex items-center w-full gap-3">
        <div class="flex">
          <img
           src="/uploads/images/product/{product.id}.webp"
           alt={product.description}
           class="aspect-square object-contain w-60"
          />
        </div>
        <div class="flex justify-between h-full flex-col">
          <p>{product.description}</p>
          <div class="flex flex-col gap-2">
            <div class="flex justify-between">
              <span class="text-3xl font-mono font-semibold">{product.price}kr</span> 
              <Button variant="outline" onclick={() => showSoldOutPage=true}>
                <FlagIcon/>
              </Button>
            </div>
            <div class="flex justify-between items-center">
              {#if addedToCart == 0}
                <Button disabled>-</Button>
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
    {/if}
  </Dialog.Content>
</Dialog.Root>

<div class="flex flex-col bg-card p-2 rounded-md border">
    <button class="flex flex-col" onclick={() => productPopup = true}>
      <p class="truncate text-left font-bold">{product.name}</p>
      <div class="overflow-hidden rounded-xl">
        <img
         src="/uploads/images/product/{product.id}.webp"
         alt={product.description}
         class="aspect-square object-contain"
        />
      </div>
      <div class="flex justify-center w-full">
        <span class="text-3xl font-mono font-semibold">{product.price}kr</span> 
      </div>
    </button>
    <div class="flex relative justify-center items-center">
      {#if addedToCart > 0}
        <div class="flex absolute gap-1 justify-center right-0">
          <ShoppingCartIcon/>
          <span>{addedToCart}</span>
        </div>
      {/if}
      <Button onclick={() => buyProduct()}>Köp</Button>
    </div>
</div>
