<script lang="ts">
 import * as Form from "$lib/components/ui/form/index.js";
 import { Input } from "$lib/components/ui/input/index.js";
    import Textarea from "$lib/components/ui/textarea/textarea.svelte";
 import { productFormSchema, type ProductFormSchema } from "./product-schema";
 import {
  type SuperValidated,
  type Infer,
  superForm,
  fileProxy,
 } from "sveltekit-superforms";
 import { zod4Client } from "sveltekit-superforms/adapters";
 
 let { data }: { 
   data: { form: SuperValidated<Infer<ProductFormSchema>> } 
  } = $props();
 
 const form = superForm(data.form, {
  SPA: true,
  validators: zod4Client(productFormSchema),
  onUpdate({ form }) {
    if (form.valid) {
      console.log(form);
    }
  }
 });
 
 const { form: formData, enhance } = form;
 const file = fileProxy(form, "image");
</script>
 
<form method="POST" enctype="multipart/form-data" use:enhance>
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
        <Form.Label>Produktbeskrivning</Form.Label>
        <input
          {...props}
          type="file"
          class="resize-none bg-red-500"
          bind:files={$file}
        />
      {/snippet}
    </Form.Control>
    <Form.FieldErrors />
  </Form.Field>

 <Form.Button>Submit</Form.Button>
</form>
