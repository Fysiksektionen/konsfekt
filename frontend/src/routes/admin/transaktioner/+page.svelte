<script lang="ts">
    import TransactionTable from "$lib/components/TransactionTable.svelte";
    import Input from "$lib/components/ui/input/input.svelte";
    import { defaultTransactionQuery, getTransactions } from "$lib/utils";
  import type { PageProps } from "./$types";
    import Button from "$lib/components/ui/button/button.svelte";
    import { onMount } from "svelte";

  let { data }: PageProps = $props();

  let transactionQuery = $state(defaultTransactionQuery());
  
  let timeOfSearchInputChange = $state(Date.now());
  let hasSearched = $state(true);
  let transactions = $state(data.transactions);

  async function search() {
    if (transactionQuery.search_term == "") {
      transactionQuery.search_term = undefined;
    }
    transactions = await getTransactions(transactionQuery);
  }

  onMount(() => {
  	const interval = setInterval(() => {
  		const currentTime = Date.now();
      if (currentTime - timeOfSearchInputChange > 250 && !hasSearched) {
        search();
      }
  	}, 50);
  
  	return () => {
  		clearInterval(interval);
  	};
  });
</script>

<div class="flex gap-3 flex-col">
  <div class="flex gap-3">
    <Input 
      oninput={() => {
        timeOfSearchInputChange = Date.now();
        hasSearched = false;
      }} 
      placeholder="Sök efter transaktioner..." class="max-w-sm mr-1" bind:value={transactionQuery.search_term}/>
    <!-- <Button onclick={() => search()} variant="secondary">Sök</Button> <!-- TODO: search as user types -->
  </div>

  <TransactionTable transactions={transactions} isAdminTable={true}/>
</div>

