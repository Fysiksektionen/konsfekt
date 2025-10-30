import type { ColumnDef } from "@tanstack/table-core";
import { renderSnippet } from "../ui/data-table";
import { createRawSnippet } from "svelte";
 
// This type is used to define the shape of our data.
// You can use a Zod schema here if you want.
export type Transaction = {
 id: string;
 amount: number;
 type: "deposit" | "payment";
 description: string;
};
 
export const columns: ColumnDef<Transaction>[] = [
 {
  accessorKey: "amount",
  header: "Summa",
  cell: ({ row }) => {
      const formatter = new Intl.NumberFormat("sv-SE", {
        style: "currency",
        currency: "SEK",
      });
 
      const amountCellSnippet = createRawSnippet<[{ amount: number; type: "deposit" | "payment" }]>(
        (getAmount) => {
          const { amount, type } = getAmount();
          let formatted = formatter.format(amount);
          let textColor;
          if (type==="deposit") {
              formatted = "+" + formatted
              textColor = "text-secondary";
          } else {
              formatted = "-" + formatted
              textColor = "text-primary";
          }
          return {
            render: () =>
              `<div class="text-right ${textColor} text-shadow-2xs text-shadow-accent font-medium w-1/2">${formatted}</div>`,
          };
        }
      );
 
      return renderSnippet(amountCellSnippet, {
        amount: row.original.amount,
        type: row.original.type
      });
    },
 },
 {
  accessorKey: "type",
  header: "Transaktionstyp",
  cell: ({ row }) => {
      const typeCellSnippet = createRawSnippet<[{ type: "deposit" | "payment" }]>(
        (getAmount) => {
          const { type } = getAmount();
          return {
            render: () =>
              `${type == "deposit" ? "Insättning" : "Köp"}`,
          };
        }
      );
 
      return renderSnippet(typeCellSnippet, {
        type: row.original.type,
      });
    },
 },
 {
  accessorKey: "description",
  header: "Beskrivning",
 },
];
