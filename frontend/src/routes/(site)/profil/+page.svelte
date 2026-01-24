<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { Input } from '$lib/components/ui/input';
	import type { PageProps } from './$types';
  import * as Item from "$lib/components/ui/item/index.js";
  import DataTable from '$lib/components/transactions/data-table.svelte';
  import type { Transaction } from '$lib/components/transactions/columns';
  import { columns } from '$lib/components/transactions/columns';
  import DarkModeToggle from '$lib/components/DarkModeToggle.svelte';
  import Switch from '$lib/components/ui/switch/switch.svelte';

	let { data }: PageProps = $props();

  let username = $state(data.user.name);
  
  let transactions = $state(data.transactions)

  const isAdmin = ["admin", "maintainer"].includes(data.user.role);
</script>

<div class="w-full md:pl-10 md:pr-10 lg:pl-30 lg:pr-30 gap-3 flex flex-col items-start">
  <h1 class="scroll-m-20 text-4xl mb-3 lg:mt-5 font-extrabold tracking-tight lg:text-5xl">
    Min profil
  </h1>
  
  {#if isAdmin}
    <div class="flex items-center gap-3">
     <p>Du är administratör</p> 
     <Button href="/admin" variant="outline" class="hover:bg-primary">Gå till adminsidan</Button>
    </div>
  {/if}

  <div class="flex items-center gap-3">
   <p>Byt mellan mörkt och ljust läge</p> 
   <DarkModeToggle/>
  </div>

  <Item.Root variant="outline" class="max-w-[500px]">
    <Item.Content>
      <Item.Title>Delta i topplistan</Item.Title>
      <Item.Description>
        Genom att delta i topplistan kan ditt namn visas på skärmen i Konsulatet
      </Item.Description>
    </Item.Content>
    <Item.Actions>
      <Switch/>
    </Item.Actions>
  </Item.Root>

  <form class="flex w-full flex-col max-w-sm space-x-2">
   <div class="flex gap-3">
    <Input bind:value={username} type="name" placeholder='Ditt namn'/>
    {#if username}
      <Button type="submit" class="text-card-foreground" variant="secondary">{data.user.name ? "Byt namn" : "Lägg till namn"}</Button>
    {:else}
      <Button type="submit" disabled class="text-card-foreground" variant="secondary">{data.user.name ? "Byt namn" : "Lägg till namn"}</Button>
    {/if}
   </div>
   <p class="text-muted-foreground text-sm pl-2 pt-1">Detta namn kan ses av andra på topplistan</p>
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

  <div class="flex flex-col w-full gap-2">
    <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">Köp- och insättningshistorik</h3> 
    <Item.Root variant="outline" class="max-w-[500px]">
      <Item.Content>
        <Item.Title>Anonyma köp</Item.Title>
        <Item.Description>
          Vill du att dina köp <u>inte</u> ska kopplas till ditt namn?
        </Item.Description>
      </Item.Content>
      <Item.Actions>
        <Switch/>
      </Item.Actions>
    </Item.Root>

    <DataTable data={transactions} {columns} balance={data.user.balance}/>
  </div>
</div>
