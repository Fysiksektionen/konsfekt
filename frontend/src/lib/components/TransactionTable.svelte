<script lang="ts">
  import * as Table from "$lib/components/ui/table/index.js";
    import { fetchJSON, getDateString, undoTransaction } from "$lib/utils";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
    import { onMount } from "svelte";
    import { invalidateAll } from "$app/navigation";
    import Button from "$lib/components/ui/button/button.svelte";
    import Badge from "$lib/components/ui/badge/badge.svelte";

  let { transactions, isAdminTable = false } = $props();
  
  let currentTransaction = $state(null);
  let transactionViewOpen = $state(false);

  async function onTransactionClicked(transactionID: number) {
    currentTransaction = await fetchJSON(fetch, "/api/get_detailed_transaction/" + transactionID);
    transactionViewOpen = true;
  }
 let currentTime = $state(Math.floor(Date.now()/1000));

 let timeSincePurchace = $derived(currentTime - currentTransaction?.datetime ?? 0);

 onMount(() => {
		const interval = setInterval(() => {
			currentTime = Math.floor(Date.now()/1000);
		}, 1000);

		return () => {
			clearInterval(interval);
		};
	});
</script>

<div class="rounded-md border">
  <Table.Root>
    <Table.Header>
      <Table.Row>
        {#if isAdminTable}
          <Table.Head>Användare</Table.Head>
        {/if}
        <Table.Head>Typ</Table.Head>
        <Table.Head>Belopp</Table.Head>
        <Table.Head>Datum</Table.Head>
        <Table.Head class="text-end">Transaktions ID</Table.Head>
      </Table.Row>
    </Table.Header>
    <Table.Body>
      {#each transactions as transaction}
        <Table.Row onclick={() => onTransactionClicked(transaction.id)}>
          {#if isAdminTable}
            <Table.Cell>{transaction.user_email}</Table.Cell>
          {/if}
          <Table.Cell>{transaction.amount > 0 ? 'Insättning' : "Köp"}</Table.Cell>
          <Table.Cell class="font-medium text-blue-500">
            {#if transaction.amount > 0}
              <div class="text-end font-medium text-blue-500">+{transaction.amount} kr</div>
            {:else }
              <div class="text-end font-medium text-red-500">-{Math.abs(transaction.amount)} kr</div>
            {/if}
          </Table.Cell>
          <Table.Cell>{getDateString(transaction.datetime)}</Table.Cell>
          <Table.Cell class="text-end">
            <Badge variant="outline">T{transaction.id}</Badge>
          </Table.Cell>
        </Table.Row>
      {:else}
        <Table.Row>
          <Table.Cell colspan={isAdminTable ? 5 : 4} class="text-center text-muted-foreground">
            Inga transaktioner hittades
          </Table.Cell>
        </Table.Row>
      {/each}
    </Table.Body>
  </Table.Root>
</div>

<Dialog.Root bind:open={transactionViewOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>{currentTransaction?.amount > 0 ? "Insättning" : "Köp" }</Dialog.Title>
      <Dialog.Description>
        {getDateString(currentTransaction?.datetime)}
        <Separator/>
        {#if currentTransaction?.amount <= 0}
          <div class="flex flex-col mt-1">
            {#each currentTransaction?.items as item}
              <div class="flex gap-2">
                <p>{item.quantity}x {item.name} ({item.quantity * item.price}kr)</p>
                <Badge variant="outline">P{item.product_id}</Badge>
              </div>
            {/each}
          </div>
        {/if}
        <span class="text-2xl font-mono font-semibold">{Math.abs(currentTransaction?.amount)}kr</span> 
      </Dialog.Description>
    </Dialog.Header>
    {#if !isAdminTable}
      {#key currentTime}
        {#if timeSincePurchace < 60}
          <Button onclick={() => {
            undoTransaction(currentTransaction?.id)
            transactionViewOpen = false;
            invalidateAll();
          }}>
            Ångra köp ({60 - timeSincePurchace})
          </Button>
        {/if}
      {/key}
    {/if}
  </Dialog.Content>
</Dialog.Root>
