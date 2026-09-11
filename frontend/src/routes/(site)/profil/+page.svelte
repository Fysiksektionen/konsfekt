<script lang="ts">
  import Button from '$lib/components/ui/button/button.svelte';
  import { Input } from '$lib/components/ui/input';
	import type { PageProps } from './$types';
  import * as Item from "$lib/components/ui/item/index.js";
  import DarkModeToggle from '$lib/components/DarkModeToggle.svelte';
  import Footer from '$lib/components/Footer.svelte';
    import { backendPOST, UNDO_PURCHASE_WINDOW_SECONDS, undoPurchase, getTransactions, nextTransactionCursor, transactionQueryFromUserId, type TransactionSummary } from "$lib/utils";
    import { Switch } from '$lib/components/ui/switch';
    import { invalidateAll } from '$app/navigation';
    import TransactionTable from '$lib/components/TransactionTable.svelte';
    import { toast } from 'svelte-sonner';
    import { undoablePurchases } from '$lib/storage.svelte';
    import { fetchJSON, type TransactionDetail } from '$lib/utils';
    import Badge from '$lib/components/ui/badge/badge.svelte';
    import { onMount } from 'svelte';
    import RotateCcwIcon from "@lucide/svelte/icons/rotate-ccw";

	let { data }: PageProps = $props();

  let undoablePurchasesDetail = $state<TransactionDetail[]>([]);
  let currentTime = $state(Math.floor(Date.now()/1000));

  const transactionQuery = transactionQueryFromUserId(data.user.id);

  // Fetched pages, cached so going back doesn't need a refetch.
  let pages = $state<TransactionSummary[][]>([data.transactions]);
  let pageIndex = $state(0);

  let hasPreviousPage = $derived(pageIndex > 0);
  let hasNextPage = $derived(
    pageIndex + 1 < pages.length || (pages[pageIndex]?.length ?? 0) === transactionQuery.limit
  );

  async function nextPage() {
    if (pageIndex + 1 < pages.length) {
      pageIndex++;
      return;
    }
    transactionQuery.cursor = nextTransactionCursor(pages[pageIndex]);
    const next = await getTransactions(transactionQuery);
    pages = [...pages, next];
    pageIndex++;
  }

  function previousPage() {
    if (pageIndex > 0) {
      pageIndex--;
    }
  }

  // Transactions of the current page, minus any still-undoable purchases shown separately above.
  let transactions = $derived(
    (pages[pageIndex] ?? []).filter(tx => !undoablePurchasesDetail.some(u => u.id === tx.id))
  );

  onMount(() => {
   const interval = setInterval(() => {
   	currentTime = Math.floor(Date.now()/1000);
   }, 1000);
  
   return () => {
   	clearInterval(interval);
   };
	});

  $effect(() => {
    const ids = Object.keys(undoablePurchases.purchases).map(Number);
    const currentTime = Math.floor(Date.now() / 1000);

    Promise.all(
      ids.map((txId) =>
        fetchJSON(fetch, "/api/get_detailed_transaction/" + txId)
          .then((tx: TransactionDetail) => tx)
          .catch(() => null)
      )
    ).then((results) => {
      const fresh = results.filter(
        (tx): tx is TransactionDetail => tx !== null && currentTime - tx.datetime < UNDO_PURCHASE_WINDOW_SECONDS
      );
      undoablePurchasesDetail = fresh;
      const keep = fresh.map((tx) => tx.id);
      if (keep.length !== ids.length) {
        // Rebuild the id→token map with only the still-undoable purchases.
        undoablePurchases.purchases = Object.fromEntries(
          keep.map((id) => [id, undoablePurchases.purchases[id]])
        );
      }
    });
  });

  let username = $state(data.user.name);
  
  const isAdmin = ["admin", "maintainer"].includes(data.user.role);

  let privateTransactionsEnabled = $state(data.user.private_transactions)

  async function changeUserFlag(flag: "on_leaderboard" | "private_transactions", value: boolean) {
    let resp = await backendPOST(`/set_user_flags?${flag}=${value}`, {}, true);
    if (!resp.ok) {
      toast.warning("Kunde inte uppdatera: " + flag)
    }
  }

 async function setUsername() {
   let resp = await backendPOST("/set_username", {name: username}, true);
   if (resp.ok) {
     invalidateAll();
   }
 }
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
      <Button onclick={() => setUsername()} type="submit" class="text-card-foreground" variant="secondary">{data.user.name ? "Byt namn" : "Lägg till namn"}</Button>
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
      <Button href="/profil/byt-mail" variant="outline" class="hover:bg-primary" size="sm">Byt Gmail-address</Button>
    </Item.Actions>
  </Item.Root>
  
  <div class="flex">
    <Button href="/api/auth/logout" rel="external" class="text-card-foreground">
      Logga ut
    </Button> 
    <Button href="/profil/radera-konto" variant="link" class="text-foreground">
      Jag vill ta bort mitt konto
    </Button> 
  </div>

  <div class="flex flex-col w-full gap-2">
    <h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">Köp- och insättningshistorik</h3> 
    <Item.Root variant="outline" class="max-w-[500px]">
      <Item.Content>
        <Item.Title><a href="/om#anonyma-transaktioner">Anonyma transaktioner</a></Item.Title>
        <Item.Description>
          Vill du att dina köp och insättningar <u>inte</u> ska kopplas till ditt namn? 
          <a href="/om#anonyma-transaktioner">Läs mer </a>
        </Item.Description>
      </Item.Content>
      <Item.Actions>
        <Switch bind:checked={privateTransactionsEnabled} onclick={() => changeUserFlag("private_transactions", !privateTransactionsEnabled)}/>
      </Item.Actions>
    </Item.Root>
    {#if undoablePurchasesDetail.length > 0}
    <h4 class="scroll-m-20 text-xl font-semibold tracking-tight">
      Senaste köp
    </h4>
    <div class="flex w-full max-w-[500px] flex-col gap-2">
    {#key currentTime}
    {#each undoablePurchasesDetail as transaction (transaction.id)}
      {@const secondsLeft = UNDO_PURCHASE_WINDOW_SECONDS - (currentTime - transaction.datetime)}
      {@const canUndo = secondsLeft > 0}
      <Item.Root variant="outline">
        <Item.Content>
          <Item.Title>
            {transaction.items.map((item) => `${item.quantity}× ${item.name}`).join(", ")}
          </Item.Title>
          <Item.Description>
            <div class="flex gap-3">
              <Badge variant="outline" class="font-mono">T{transaction.id}</Badge>
              <span class="font-mono text-red-500">−{Math.abs(transaction.amount)} kr</span>
            </div>
          </Item.Description>
        </Item.Content>
        <Item.Actions>
          <Button
            size="sm"
            variant="secondary"
            class="text-card-foreground"
            disabled={!canUndo}
            onclick={() => undoPurchase(transaction.id)}>
            <RotateCcwIcon class="size-4" />
            Ångra köp {secondsLeft > 0 ? secondsLeft : ""}
          </Button>
        </Item.Actions>
      </Item.Root>
    {/each}
    {/key}
    </div>
    {/if}
    <TransactionTable
      transactions={transactions}
      isAdminTable={false}
      hasPreviousPage={hasPreviousPage}
      hasNextPage={hasNextPage}
      onPreviousPage={previousPage}
      onNextPage={nextPage}
    />
    {#if data.transactions.length > 0}
      <Button href="/profil/koppla-bort-transaktioner" variant="link" class="text-foreground">
        Dissociera transaktioner från mitt konto
      </Button>
    {/if}
  </div>
</div>

<Footer/>
