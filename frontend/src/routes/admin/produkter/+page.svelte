<script lang="ts">
    import Input from '$lib/components/ui/input/input.svelte';
    import { createSearchStore, searchData, updateSearchStore } from '$lib/utils';
    import * as Sheet from "$lib/components/ui/sheet/index.js";
    import Button from '$lib/components/ui/button/button.svelte';
    import ProductForm from './ProductForm.svelte';
    import type { PageData } from './$types.js';

	let { data }: { data: PageData } = $props();
  let searchTerm = $state("");
  const searchableTerms = ["name", "description"];
  let searchStore = $state(createSearchStore(data.products, searchableTerms));
  $effect(() => {
    searchData(searchStore, searchTerm);
  });
  
  let addProductSheetOpen = $state(false);
  function onFormSubmit(newProducts: any[]) {
    updateSearchStore(searchStore, newProducts);
    addProductSheetOpen = false;
  }
</script>


<Input bind:value={searchTerm} placeholder="Sök efter produkter..."/>

<Sheet.Root bind:open={addProductSheetOpen}>
  <Sheet.Trigger>
    <Button class="mt-3">Lägg till en produkt</Button>
  </Sheet.Trigger>
  <Sheet.Content>
    <Sheet.Header class="h-full">
      <Sheet.Title>Lägg till en ny produkt</Sheet.Title>
      <ProductForm {data} {onFormSubmit}/>
    </Sheet.Header>
  </Sheet.Content>
</Sheet.Root>

<div class="grid grid-cols-4 gap-3 mt-3">
  {#each searchStore.filtered as product}
    <Sheet.Root>
      <Sheet.Trigger>
        <div class="flex flex-col p-2 bg-card rounded-md border">
            <p class="truncate text-left font-bold">{product.name}</p> 
            <div class="flex justify-between">
              <div class="overflow-hidden rounded-xl">
                <img
                 src="/uploads/images/product/{product.id}.webp"
                 alt={product.description}
                 class="aspect-square h-[80px] object-cover"
                />
              </div>
              <div class="flex flex-col justify-end">
                <p class="">{product.price}kr</p>
              </div>
            </div>
        </div>
      </Sheet.Trigger>
      <Sheet.Content>
        <Sheet.Header>
          <Sheet.Title>{product.name}</Sheet.Title>
          <Sheet.Description>
          <p>
            Här kan du ändra information och pris för denna produkt.
          </p>
            <div class="overflow-hidden rounded-xl">
              <img
               src="/uploads/images/product/{product.id}.webp"
               alt={product.name}
               class="aspect-square h-[80px] object-cover"
              />
            </div>
          </Sheet.Description>
        </Sheet.Header>
      </Sheet.Content>
    </Sheet.Root>
  {/each}
</div>
