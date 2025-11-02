<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { Input } from '$lib/components/ui/input';
	import type { PageProps } from './$types';
  import * as Item from "$lib/components/ui/item/index.js";
  import DataTable from '$lib/components/transactions/data-table.svelte';
  import type { Transaction } from '$lib/components/transactions/columns';
  import { columns } from '$lib/components/transactions/columns';

	let { data }: PageProps = $props();

  let username = $state(data.user.name);
  
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
  
  <form class="flex w-full flex-col max-w-sm space-x-2">
   <div class="flex gap-3">
    <Input bind:value={username} type="name" placeholder='Ditt namn'/>
    {#if username}
      <Button type="submit" class="text-card-foreground" variant="secondary">{data.user.name ? "Byt namn" : "Lägg till namn"}</Button>
    {:else}
      <Button type="submit" disabled class="text-card-foreground" variant="secondary">{data.user.name ? "Byt namn" : "Lägg till namn"}</Button>
    {/if}
   </div>
   <p class="text-muted-foreground text-sm pl-2 pt-1">Detta namn kan ses av andra</p>
  </form>

  <Item.Root variant="outline">
    <Item.Content>
      <Item.Title>Byte av inloggningsmail</Item.Title>
      <Item.Description>
        Nuvarande gmail:<br>{data.user.email}
      </Item.Description>
    </Item.Content>
    <Item.Actions>
      <Button variant="outline" class="hover:bg-primary" size="sm">Logga in med annan Gmail</Button>
    </Item.Actions>
  </Item.Root>

  <Button class="text-card-foreground">
    Logga ut
  </Button> 

  <div class="w-full">
   <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">Köp- och insättningshistorik</h3> 
    <div class="text-muted-foreground flex-1 pt-3 text-md">
      Totalt saldo: {data.user.balance}kr
    </div>
    <DataTable data={transactions} {columns}/>
  </div>
</div>
