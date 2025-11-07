<script lang="ts">
    import Input from '$lib/components/ui/input/input.svelte';
    import { createSearchStore, searchData } from '$lib/utils';
    import * as Sheet from "$lib/components/ui/sheet/index.js";
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
  let searchTerm = $state("");
  let searchStore = $state(createSearchStore(data.products, ["title", "description", "category"]));
  $effect(() => {
    searchData(searchStore, searchTerm);
  });
</script>


<Input bind:value={searchTerm} placeholder="Sök efter produkter..."/>
<div class="grid grid-cols-4 gap-3 mt-3">
  {#each searchStore.filtered as product}
    <Sheet.Root>
      <Sheet.Trigger>
        <div class="flex flex-col p-2 bg-card rounded-md border">
            <p class="truncate text-left font-bold">{product.title}</p> 
            <div class="flex justify-between">
              <div class="overflow-hidden rounded-xl">
                <img
                 src="/uploads/images/product/0.webp"
                 alt={product.title}
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
          <Sheet.Title>{product.title}</Sheet.Title>
          <Sheet.Description>
            Här kan du ändra information och pris för denna produkt.
          </Sheet.Description>
        </Sheet.Header>
        <div class="overflow-hidden rounded-xl">
          <img
           src="/uploads/images/product/0.webp"
           alt={product.title}
           class="aspect-square h-[80px] object-cover"
          />
        </div>
      </Sheet.Content>
    </Sheet.Root>
  {/each}
</div>
