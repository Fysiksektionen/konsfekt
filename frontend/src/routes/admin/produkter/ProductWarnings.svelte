<script lang="ts">
  import WarningCircleIcon from "@lucide/svelte/icons/circle-alert"
  import WarningTriangleIcon from "@lucide/svelte/icons/triangle-alert"
  import ArchiveXIcon from "@lucide/svelte/icons/archive-x"

  let { product, short }: { product: any, short: boolean } = $props();
  
  let undefinedStock = $derived(product.stock == null || product.stock == undefined);
  let negativeStock = $derived(!undefinedStock && product.stock <= 0);
</script>

{#if short}
  <div class="flex gap-1">
    {#if undefinedStock}
      <ArchiveXIcon class="text-yellow-300"/>
    {:else if negativeStock}
      <WarningCircleIcon class="text-yellow-300"/>
    {/if}
    {#if product.flags.marked_sold_out}
      <WarningTriangleIcon class="text-red-500"/>
    {/if}
  </div>
{:else}
  <div class="flex flex-col gap-2">
    <div class="flex gap-2">
      {#if undefinedStock}
        <ArchiveXIcon class="text-yellow-300"/> <p>Produkten finns inte med i sortimentet</p>
      {:else if negativeStock}
        <WarningCircleIcon class="text-yellow-300"/> <p>Produktens lagerstatus är inte positivt</p>
      {/if}
    </div>
    <div class="flex gap-2">
      {#if product.flags.marked_sold_out}
        <WarningTriangleIcon class="text-red-500"/> <p>Produkten har rapporterats vara slut</p>
      {/if}
    </div>
  </div>
{/if}
