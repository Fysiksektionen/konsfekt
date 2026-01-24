import type { ColumnDef } from "@tanstack/table-core";
import { renderSnippet } from "../ui/data-table";
import { createRawSnippet } from "svelte";
import { getDateString } from "$lib/utils";
 
// This type is used to define the shape of our data.
// You can use a Zod schema here if you want.
//
export type TransactionItem = {
    name: string;
    price: number;
    quantity: number;
    product_id: number;
}

export type Transaction = {
 id: number;
 amount: number;
 datetime: string;
 search_term: string;
 items: TransactionItem[];
}

export const columns: ColumnDef<Transaction>[] = [
 {
  accessorKey: "amount",
  header: "Belopp",
  cell: ({ row }) => {
      const formatter = new Intl.NumberFormat("sv-SE", {
        style: "currency",
        currency: "SEK",
      });
 
      const totalCellSnippet = createRawSnippet<[{ amount: number; }]>(
        (getAmount) => {
          const { amount } = getAmount();
          let formatted = formatter.format(amount);
          let textColor;
          if (amount > 0) {
              formatted = "+" + formatted
              textColor = "text-blue-500";
          } else {
              formatted = formatted
              textColor = "text-red-500";
          }
          return {
            render: () =>
              `<div class="text-right ${textColor} font-medium w-1/2">${formatted}</div>`,
          };
        }
      );
 
      return renderSnippet(totalCellSnippet, {
        amount: row.original.amount,
      });
    },
 },
 {
  accessorKey: "type",
  header: "Transaktionstyp",
  cell: ({ row }) => {
      const typeCellSnippet = createRawSnippet<[{ amount: number }]>(
        (getAmount) => {
          const { amount } = getAmount();
          return {
            render: () =>
              `${amount > 0 ? "Insättning" : "Köp"}`,
          };
        }
      );
 
      return renderSnippet(typeCellSnippet, {
        amount: row.original.amount,
      });
    },
 },
 {
  accessorKey: "date",
  header: "Datum",
  cell: ({ row }) => {
      const typeCellSnippet = createRawSnippet<[{ datetime: string }]>(
        (getDatetime) => {
          const { datetime } = getDatetime();
          return {
            render: () =>
                getDateString(datetime)
          };
        }
      );
 
      return renderSnippet(typeCellSnippet, {
        datetime: row.original.datetime,
      });
  }
 },
 {
  accessorKey: "search_term",
 },
];
