<script lang="ts">
  import ShoppingCartIcon from "@lucide/svelte/icons/shopping-basket";
  import FlagIcon from "@lucide/svelte/icons/flag";
  import Button from "$lib/components/ui/button/button.svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { backendPOST, undoTransaction } from "$lib/utils";
    import { toast } from "svelte-sonner";
    import { invalidateAll } from "$app/navigation";

  let { user, product, addedToCart = $bindable(0) } = $props();

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
      toast.success(product.name + " köpt.", user.private_transactions ? {} : {
        action: {
          label: "Ångra",
          onClick: () => undoTransaction(transactionID.transaction_id)
        }
      });
    } else if (response.status == 402) {
      toast.error("Du har inte tillräckligt saldo");
    } else {
      toast.error("Kunde inte köpa produkten: " + response.statusText);
    }
  }

  let showSoldOutPage = $state(false);
  let productPopup = $state(false);
</script>

<Dialog.Root bind:open={productPopup}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title class="flex items-center gap-2">
        {product.name}
        {#if product.flags.new_product}
          <span class="rounded-full bg-accent px-2 py-0.5 text-xs font-semibold text-accent-foreground">NYHET</span>
        {/if}
      </Dialog.Title>
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
              <span class="text-3xl font-mono font-medium">{product.price}kr</span>
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

<div class="group flex flex-col overflow-hidden rounded-2xl border bg-card shadow-sm transition-all duration-200 hover:-translate-y-0.5 hover:shadow-lg">
    <button class="flex flex-col text-left" onclick={() => productPopup = true}>
      <div class="relative overflow-hidden bg-muted">
        {#if product.flags.new_product}
          <span class="absolute top-2 right-2 z-10 rounded-full bg-accent px-2 py-0.5 text-xs font-semibold text-accent-foreground">NYHET</span>
        {/if}
        <img
         src="/uploads/images/product/{product.id}.webp"
         alt={product.description}
         class="aspect-square w-full object-contain transition-transform duration-300 group-hover:scale-105"
        />
      </div>
      <p class="truncate px-3 pt-2 font-semibold">{product.name}</p>
    </button>
    <div class="flex items-center justify-between gap-1 px-2 pb-3 pt-1 sm:gap-2 sm:px-3">
      <span class="truncate font-mono text-lg font-medium sm:text-2xl">{product.price}kr</span>
      <div class="relative shrink-0">
        {#if addedToCart > 0}
          <div class="absolute -top-2 -right-2 flex items-center gap-1 rounded-full bg-accent px-2 py-0.5 text-xs font-semibold text-accent-foreground">
            <ShoppingCartIcon class="size-3.5"/>
            <span>{addedToCart}</span>
          </div>
        {/if}
        <Button class="rounded-xl px-3 sm:px-4" onclick={() => buyProduct()}>Köp</Button>
      </div>
    </div>
</div>
