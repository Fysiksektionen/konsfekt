<script lang="ts" generics="TData, TValue">
 import {
  type PaginationState,
  type SortingState,
  type ColumnFiltersState,
  type VisibilityState,
  getCoreRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  getFilteredRowModel,
  type ColumnDef,
 } from "@tanstack/table-core";
    import { createSvelteTable } from "../ui/data-table";
    import Input from "../ui/input/input.svelte";
  import * as Table from "$lib/components/ui/table/index.js"
    import Button from "../ui/button/button.svelte";
    import FlexRender from "../ui/data-table/flex-render.svelte";
import { Badge } from "$lib/components/ui/badge/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { Transaction } from "./columns";
    import { getDateString } from "$lib/utils";

  type DataTableProps<TData, TValue> = {
   data: TData[];
   columns: ColumnDef<TData, TValue>[];
 };
 let { columns, data, balance }: DataTableProps<TData, TValue> & {balance: Number} = $props();
 
 let pagination = $state<PaginationState>({ pageIndex: 0, pageSize: 10 });
 let sorting = $state<SortingState>([]);
 let columnFilters = $state<ColumnFiltersState>([]);
 let columnVisibility = $state<VisibilityState>({search_term: false});
 
 const table = createSvelteTable({
  get data() {
   return data;
  },
  columns,
  getCoreRowModel: getCoreRowModel(),
  getPaginationRowModel: getPaginationRowModel(),
  getSortedRowModel: getSortedRowModel(),
  getFilteredRowModel: getFilteredRowModel(),
  onPaginationChange: (updater) => {
   if (typeof updater === "function") {
    pagination = updater(pagination);
   } else {
    pagination = updater;
   }
  },
  onSortingChange: (updater) => {
   if (typeof updater === "function") {
    sorting = updater(sorting);
   } else {
    sorting = updater;
   }
  },
  onColumnFiltersChange: (updater) => {
   if (typeof updater === "function") {
    columnFilters = updater(columnFilters);
   } else {
    columnFilters = updater;
   }
  },
  onColumnVisibilityChange: (updater) => {
   if (typeof updater === "function") {
    columnVisibility = updater(columnVisibility);
   } else {
    columnVisibility = updater;
   }
  },
  state: {
   get pagination() {
    return pagination;
   },
   get sorting() {
    return sorting;
   },
   get columnFilters() {
    return columnFilters;
   },
   get columnVisibility() {
    return columnVisibility;
   },
  },
 });
 let currentTransaction: Transaction | undefined = $state();
 let transactionViewOpen = $state(false);
</script>

<Dialog.Root bind:open={transactionViewOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>{currentTransaction?.amount > 0 ? "Insättning" : "Köp" }</Dialog.Title>
      <Dialog.Description>
        {getDateString(currentTransaction?.datetime)}
        {#if currentTransaction?.amount <= 0}
          <div class="flex flex-row gap-2">
            {#each currentTransaction?.items as item}
              <p>{item.quantity}x {item.name} ({item.quantity * item.price}kr)</p>
            {/each}
          </div>
        {/if}
      </Dialog.Description>
    </Dialog.Header>
  </Dialog.Content>
</Dialog.Root>
 
<div class="w-full">
  <div class="flex md:flex-row flex-col-reverse items-center py-4 gap-3">
    <Input
      placeholder="Sök efter transaktion..."
      value={(table.getColumn("search_term")?.getFilterValue() as string) ?? ""}
      oninput={(e) =>
        table.getColumn("search_term")?.setFilterValue(e.currentTarget.value)}
      onchange={(e) => {
        table.getColumn("search_term")?.setFilterValue(e.currentTarget.value);
      }}
      class="max-w-sm"
    />
    <div class="text-muted-foreground flex-1 text-md">
      Totalt saldo: {balance}kr
    </div>
  </div>
  <div class="rounded-md border">
    <Table.Root>
      <Table.Header>
        {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
          <Table.Row>
            {#each headerGroup.headers as header (header.id)}
              <Table.Head class="[&:has([role=checkbox])]:pl-3">
                {#if !header.isPlaceholder}
                  <FlexRender
                    content={header.column.columnDef.header}
                    context={header.getContext()}
                  />
                {/if}
              </Table.Head>
            {/each}
          </Table.Row>
        {/each}
      </Table.Header>
      <Table.Body>
        {#each table.getRowModel().rows as row (row.id)}
          <Table.Row onclick={() => { transactionViewOpen = true; currentTransaction = row.original; } } data-state={row.getIsSelected() && "selected"}>
            {#each row.getVisibleCells() as cell (cell.id)}
              <Table.Cell class="[&:has([role=checkbox])]:pl-3">
                <FlexRender
                  content={cell.column.columnDef.cell}
                  context={cell.getContext()}
                />
              </Table.Cell>
            {/each}
          </Table.Row>
        {:else}
          <Table.Row>
            <Table.Cell colspan={columns.length} class="h-24 text-center">
              No results.
            </Table.Cell>
          </Table.Row>
        {/each}
      </Table.Body>
    </Table.Root>
  </div>
  <div class="flex items-center justify-between md:justify-center space-x-2 pt-4">
      <Button
        variant="outline"
        size="sm"
        onclick={() => table.previousPage()}
        disabled={!table.getCanPreviousPage()}
      >
        Previous
      </Button>
      <Button
        variant="outline"
        size="sm"
        onclick={() => table.nextPage()}
        disabled={!table.getCanNextPage()}
      >
        Next
      </Button>
  </div>
</div>
