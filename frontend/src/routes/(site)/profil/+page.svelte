<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { Input } from '$lib/components/ui/input';
	import type { PageProps } from './$types';
  import * as Item from "$lib/components/ui/item/index.js";
  import DataTable from '$lib/components/transactions/data-table.svelte';
  import type { Transaction } from '$lib/components/transactions/columns';
  import { columns } from '$lib/components/transactions/columns';

	let { data }: PageProps = $props();
  
  export const transactions: Transaction[] = [
   {
    id: "728ed52f",
    amount: 100,
    type: "deposit",
    description: "Swish instättning",
   },
   {
    id: "489e1d42",
    amount: 18,
    type: "payment",
    description: "Billys Orginal",
   },
  ];

</script>

<div class="w-full md:pl-10 md:pr-10 lg:pl-30 lg:pr-30 gap-3 flex flex-col items-start">
  <h1 class="scroll-m-20 text-4xl mb-3 lg:mt-5 font-extrabold tracking-tight lg:text-5xl">
    Min profil
  </h1>
  
  <form class="flex w-full max-w-sm items-center space-x-2">
   <Input type="name" placeholder="Ditt namn" />
   <Button type="submit" class="text-card-foreground" variant="secondary">Byt namn</Button>
  </form>

  <Item.Root variant="outline">
    <Item.Content>
      <Item.Title>Byte av inloggningsmail</Item.Title>
      <Item.Description
        >Tryck här för att byta till en annan Gmail address.</Item.Description
      >
    </Item.Content>
    <Item.Actions>
      <Button variant="outline" class="hover:bg-primary" size="sm">Logga in med ny Gmail</Button>
    </Item.Actions>
  </Item.Root>

  <Button class="text-card-foreground">
    Logga ut
  </Button> 

  <div class="w-full">
   <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">Köp- och insättningshistorik</h3> 
    <DataTable data={transactions} {columns}/>
  </div>

</div>
 

