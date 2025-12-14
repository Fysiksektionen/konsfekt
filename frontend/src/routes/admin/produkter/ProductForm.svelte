<script lang="ts">
 import * as Form from "$lib/components/ui/form/index.js";
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
 
 let { validatedForm, onFormSubmit, isCreateForm }: { 
   validatedForm: SuperValidated<Infer<ProductFormSchema>>;
   onFormSubmit: (products: any[], changedProduct: number) => void;
   isCreateForm: boolean;
  } = $props();
 
 const form = superForm(validatedForm, {
  SPA: true,
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
    let products;
    if (isCreateForm) {
      products = await backendPOST("/create_product", formData)
              .then(res => res.json());
    } else {
      products = await backendPOST("/update_product", formData)
              .then(res => res.json());
      // invalidate("/uploads/images/product/" + product.id + ".webp");
    }
    onFormSubmit(products, product.id);
  }
 });
 
 const { form: formData, enhance } = form;
 const file = fileProxy(form, "image");

 const imgURL = $derived.by(() => {
   if ($file.item(0)) {
     return URL.createObjectURL($file.item(0)!);
   } else if ($formData.id != undefined) {
     return "/uploads/images/product/" + $formData.id + ".webp";
   }
 });
</script>
 
<form method="POST" enctype="multipart/form-data" use:enhance class="flex flex-col justify-between h-full">
  <div class="flex flex-col justify-between">
    <Form.Field {form} name="name">
     <Form.Control>
      {#snippet children({ props })}
       <Form.Label>Produkt namn</Form.Label>
       <Input {...props} bind:value={$formData.name} />
      {/snippet}
     </Form.Control>
     <Form.FieldErrors />
    </Form.Field>

    <Form.Field {form} name="price">
     <Form.Control>
      {#snippet children({ props })}
       <Form.Label>Pris</Form.Label>
       <Input {...props} bind:value={$formData.price} type="number" />
      {/snippet}
     </Form.Control>
     <Form.FieldErrors />
    </Form.Field>

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
  </div>
  <div>
    <Form.Button>
      {#if isCreateForm}
        Lägg till produkt
      {:else}
        Uppdatera produkt 
      {/if}
    </Form.Button>
  </div>
</form>
