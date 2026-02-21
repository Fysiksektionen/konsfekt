<script lang="ts">
    import TransactionTable from "$lib/components/TransactionTable.svelte";
    import Input from "$lib/components/ui/input/input.svelte";
    import { defaultTransactionQuery, getTransactions, type TransactionQuery } from "$lib/utils";
  import type { PageProps } from "./$types";
  import * as Select from "$lib/components/ui/select/index.js";
    import { onMount } from "svelte";

  let { data }: PageProps = $props();

  let transactionQuery = $state(defaultTransactionQuery());
  
  let timeOfSearchInputChange = $state(Date.now());
  let hasSearched = $state(true);
  let transactions = $state(data.transactions);

  async function search() {
    hasSearched = true;
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
  
  type Filter = { value: string, label: string, apply: (q: TransactionQuery) => void };

  const filters: Filter[] = [
    { value: "today", label: "Idag",
      apply: (q) => {
        q.descending = true;
        const now = new Date();
        q.time_range = {
          start: Math.floor(new Date(now.getFullYear(), now.getMonth(), now.getDate()).getTime() / 1000)
        }
      }
    },
    { value: "thisWeek", label: "Den här veckan",
      apply: (q) => {
        q.descending = true;
        const now = new Date();
        const weekday = now.getDay() === 0 ? 6 : now.getDay() - 1;
        q.time_range = {
          start: Math.floor(new Date(now.getFullYear(), now.getMonth(), now.getDate() - weekday).getTime() / 1000)
        }
      }
    },
    { value: "last-30", label: "Senaste 30 dagarna",
      apply: (q) => {
        q.descending = true;
        q.time_range = {
          start: Math.round(Date.now()/1000) - 60 * 60 * 24 * 30
        }
      }
    },
    { value: "oldest", label: "Äldst",
      apply: (q) => {
        q.descending = false;
        q.time_range = undefined;
      }
    }
  ];
  let filterValue = $state("");
  let prevFilterValue = "";
 
  const triggerContent = $derived(
    filters.find((f) => f.value === filterValue)?.label ?? "Filtrera"
  );

  function toggleFilter(filter: Filter) {
    if (filter.value != prevFilterValue) {
      filter.apply(transactionQuery);
    } else {
      filterValue = "";
      transactionQuery.time_range = undefined;
      transactionQuery.descending = true;
    }
    prevFilterValue = filter.value;
    search();
  }
</script>

<div class="flex gap-3 flex-col">
  <div class="flex gap-3">
    <Input 
      oninput={() => {
        timeOfSearchInputChange = Date.now();
        hasSearched = false;
      }} 
      placeholder="Sök efter transaktioner..." class="max-w-sm mr-1" bind:value={transactionQuery.search_term}
    />
    <Select.Root type="single" name="filterSelector" bind:value={filterValue}>
      <Select.Trigger class="w-[180px]">{triggerContent}</Select.Trigger>
      <Select.Content>
        {#each filters as filter (filter.value)}
          <Select.Item
            value={filter.value}
            label={filter.label}
            onclick={() => toggleFilter(filter)}
          >
            {filter.label}
          </Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
  </div>

  <TransactionTable transactions={transactions} isAdminTable={true}/>
</div>

