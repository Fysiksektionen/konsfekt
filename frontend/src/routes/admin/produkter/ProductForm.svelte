<script lang="ts">
 import * as Form from "$lib/components/ui/form/index.js";
 import WarningCircleIcon from "@lucide/svelte/icons/circle-alert"
 import OutOfStockIcon from "@lucide/svelte/icons/archive-x"
 import UploadIcon from "@lucide/svelte/icons/upload"
 import { Input } from "$lib/components/ui/input/index.js";
    import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import { backendPOST } from "$lib/utils";
 import { productFormSchema, type ProductFormSchema } from "./product-schema";
 import {
  type SuperValidated,
  type Infer,
  superForm,
  fileProxy,
 } from "sveltekit-superforms";
 import { zod4Client } from "sveltekit-superforms/adapters";
    import Button from "$lib/components/ui/button/button.svelte";
    import Switch from "$lib/components/ui/switch/switch.svelte";
 
 let { validatedForm, onFormSubmit, isCreateForm }: { 
   validatedForm: SuperValidated<Infer<ProductFormSchema>>;
   onFormSubmit: (products: any[], changedProduct: number | undefined) => void;
   isCreateForm: boolean;
  } = $props();
 
 const form = superForm(validatedForm, {
  SPA: true,
  dataType: "json",
  validators: zod4Client(productFormSchema),
  async onUpdate({ form }) {
    if (!form.valid) {
      return;
    }
    
    const formData = new FormData();

    let { image, ...product } = form.data;
    formData.append("product", new Blob([JSON.stringify(product)], { type: "application/json" }))
    
    if (image) {
      formData.append("image", image);
    }

    let response;
    if (isCreateForm) {
      response = await backendPOST("/create_product", formData, false);
    } else {
      response = await backendPOST("/update_product", formData, false);
    }

    let products = await response.json();

    onFormSubmit(products, product.id);
  }
 });

 async function removeProduct() {
   let id = validatedForm.data.id;
   let response = await backendPOST("/delete_product", { id }, true)
   let products = await response.json();
   onFormSubmit(products, id)
 }
 
 const { form: formData, enhance } = form;
 const file = fileProxy(form, "image");

 let imgURL = $derived.by(() => {
   if ($file.item(0)) {
     return URL.createObjectURL($file.item(0)!);
   } else if ($formData.id != undefined) {
     return "/uploads/images/product/" + $formData.id + ".webp";
   }
 });
  
 let showRemoveButton = $state(false);

</script>
 
<form method="POST" enctype="multipart/form-data" use:enhance class="flex flex-col justify-between h-full">
  <div class="flex flex-col gap-2 justify-between">
    <Form.Field {form} name="name">
     <Form.Control>
      {#snippet children({ props })}
       <Form.Label>Produkt namn</Form.Label>
       <Input {...props} bind:value={$formData.name} />
      {/snippet}
     </Form.Control>
     <Form.FieldErrors />
    </Form.Field>
    
    <div class="flex gap-2">
      <Form.Field {form} name="price">
       <Form.Control>
        {#snippet children({ props })}
         <Form.Label>Pris</Form.Label>
         <Input {...props} bind:value={$formData.price} type="number" />
        {/snippet}
       </Form.Control>
       <Form.FieldErrors />
      </Form.Field>

      {#if !isCreateForm}
        <Form.Field {form} name="stock">
         <Form.Control>
          {#snippet children({ props })}
           <Form.Label>Lager</Form.Label>
           <Input {...props} bind:value={$formData.stock} type="number" />
          {/snippet}
         </Form.Control>
         <Form.FieldErrors />
        </Form.Field>
      {/if}
    </div>

    <Form.Field {form} name="description">
      <Form.Control>
        {#snippet children({ props })}
          <Form.Label>Produktbeskrivning</Form.Label>
          <Textarea
            {...props}
            class="resize-none"
            bind:value={$formData.description}
          />
        {/snippet}
      </Form.Control>
      <Form.FieldErrors />
    </Form.Field>
    
    <div class="flex gap-2 flex-col md:flex-row">
      <Form.Field {form} name="image">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Produktbild</Form.Label>
            {#if $file.length == 1 || $formData.id != undefined}
              <div class="overflow-hidden rounded-xl border w-fit">
                <img
                 src={imgURL}
                 alt="Produktbild"
                 class="aspect-square h-[80px] object-cover"
                />
              </div>
            {/if}
            <label>
              <input
                {...props}
                type="file"
                class="resize-none hidden"
                bind:files={$file}
              />
              <span class="
                p-1
                focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive inline-flex shrink-0 items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium outline-none transition-all focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none [&_svg]:shrink-0
                bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 border
              ">
                Ladda upp bild
                <UploadIcon/>
              </span>
            </label>
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
      
      <div class="flex pt-3 gap-1 flex-col">
        <Form.Field {form} name="flags.modifiable">
          <Form.Control>
            {#snippet children({ props })}
            <div class="flex gap-2">
              <Form.Label>Modifierbar</Form.Label>
              <Switch {...props} bind:checked={$formData.flags.modifiable} />
            </div>
            {/snippet}
          </Form.Control>
        </Form.Field>

        <Form.Field {form} name="flags.new_product">
          <Form.Control>
            {#snippet children({ props })}
            <div class="flex gap-2">
              <Form.Label>Visa som ny produkt</Form.Label>
              <Switch {...props} bind:checked={$formData.flags.new_product} />
            </div>
            {/snippet}
          </Form.Control>
        </Form.Field>
      </div>
    </div>
    {#if !isCreateForm}
      <div class="flex gap-2">
        {#if $formData.stock == null || $formData.stock == undefined}
          <OutOfStockIcon class="text-yellow-300"/> <p>Produkten finns inte med i sortimentet</p>
        {:else if $formData.stock <= 0}
          <WarningCircleIcon class="text-yellow-300"/> <p>Produkt har negativ lagerstatus</p>
        {/if}
      </div>
    {/if}
  </div>
  <div class="flex w-full md:justify-between md:flex-row gap-3 flex-col-reverse">
    <Form.Button>
      {#if isCreateForm}
        Lägg till produkt
      {:else}
        Uppdatera produkt 
      {/if}
    </Form.Button>
    <div class="flex">
    {#if showRemoveButton && !isCreateForm}
      <div class="flex flex-row-reverse md:flex-row">
        <Button variant="link" class="text-foreground" onclick={() => showRemoveButton = false}>
          Dölj
        </Button>
        <Button variant="destructive" onclick={async () => await removeProduct()}>
          Ta bort produkt
        </Button>
      </div>
    {:else if !isCreateForm}
      <Button variant="link" class="text-foreground" onclick={() => showRemoveButton = true}>
        Visa dolda alternativ
      </Button>
    {/if}
    </div>
  </div>
</form>
