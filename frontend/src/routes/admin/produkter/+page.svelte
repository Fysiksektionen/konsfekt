<script lang="ts">
    import Input from '$lib/components/ui/input/input.svelte';
    import { createSearchStore, searchData, updateSearchStore } from '$lib/utils';
    import * as Sheet from "$lib/components/ui/sheet/index.js";
    import Button from '$lib/components/ui/button/button.svelte';
    import ProductForm from './ProductForm.svelte';
    import type { PageData } from './$types.js';
    import { superValidate } from 'sveltekit-superforms';
    import { zod4 } from 'sveltekit-superforms/adapters';
    import { productFormSchema } from './product-schema';
    import { onMount } from 'svelte';

	let { data }: { data: PageData } = $props();
  let searchTerm = $state("");
  const searchableTerms = ["name", "description"];
  let searchStore = $state(createSearchStore(data.products, searchableTerms));
  $effect(() => {
    searchData(searchStore, searchTerm);
  });
  let addProductSheetOpen = $state(true);

  let updateProductForm = $state(data.form);
  let updateProductSheetOpen = $state(false);

  function onFormSubmit(newProducts: any[]) {
    updateSearchStore(searchStore, newProducts);
    addProductSheetOpen = false;
    updateProductSheetOpen = false;
  }
  
  async function openUpdateProductSheet(product: any) {
    updateProductForm = await superValidate(product, zod4(productFormSchema));
    console.log(updateProductForm)
    updateProductSheetOpen = true;
  }
  onMount(() => {
    addProductSheetOpen = false;
  })
</script>


<Input bind:value={searchTerm} placeholder="Sök efter produkt..."/>

<Sheet.Root bind:open={addProductSheetOpen}>
  <Sheet.Trigger>
    <Button class="mt-3">Lägg till en produkt</Button>
  </Sheet.Trigger>
  <Sheet.Content>
    <Sheet.Header class="h-full">
      <Sheet.Title>Lägg till en ny produkt</Sheet.Title>
      <ProductForm validatedForm={data.form} {onFormSubmit}/>
    </Sheet.Header>
  </Sheet.Content>
</Sheet.Root>

<Sheet.Root bind:open={updateProductSheetOpen}> 
  <Sheet.Content>
    <Sheet.Header class="h-full">
      <Sheet.Title>
        {updateProductForm.data.name}
      </Sheet.Title>
      <Sheet.Description>
        <p>
          Här kan du ändra information och pris för denna produkt.
        </p>
      </Sheet.Description>
      <ProductForm validatedForm={updateProductForm} {onFormSubmit}/>
    </Sheet.Header>
  </Sheet.Content>
</Sheet.Root>

<div class="grid grid-cols-4 gap-3 mt-3">
  {#each searchStore.filtered as product}
     <button onclick={() => openUpdateProductSheet(product)} class="flex flex-col p-2 bg-card rounded-md border">
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
     </button>
  {/each}
</div>


