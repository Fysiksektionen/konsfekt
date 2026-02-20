import { error, redirect } from "@sveltejs/kit";
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import { cart } from "./storage.svelte";
import { parseAbsoluteToLocal } from "@internationalized/date";
import { invalidateAll } from "$app/navigation";
import { toast } from "svelte-sonner";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any } ? Omit<T, "children"> : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & { ref?: U | null };

type svelteFetch = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

export async function getUser(fetch: svelteFetch) {
    if (import.meta.env.SSR) {
        return {
            id: 0,
            email: "",
            balance: 0
        }
    }
    const response = await fetch("/api/get_user");
    if (response.status == 401) {
        console.log(response)
        console.log(await response.text())
        redirect(302, "/login");
    }
    if (!response.ok) {
        throw error(response.status, response.statusText)
    }
    return await response.json()
}

type TimeRange = {
    start?: number;
    end?: number;
}

export type TransactionQuery = {
    user_ids: number[];
    product_ids: number[];
    time_range?: TimeRange;
    search_term?: string;
    cursor?: number;
    limit: number;
};

export function defaultTransactionQuery(): TransactionQuery {
    return {
        user_ids: [],
        product_ids: [],
        limit: 20,
    };
}

export function transactionQueryFromUserId(ownUserId: number): TransactionQuery {
    return {
        user_ids: [ownUserId],
        product_ids: [],
        limit: 20,
    };
}

export async function getTransactions(fetch: svelteFetch, query?: TransactionQuery) {
    if (import.meta.env.SSR) {
        return [];
    }
    if (query == null) {
        query = defaultTransactionQuery();
    }
    let options: RequestInit = {
        method: "GET",
        credentials: "include",
        body: JSON.stringify(query),
        headers: { "Content-Type": "application/json" }
    };
    let endpoint = "/api/get_transactions";
    let transactionResponse = await fetch(endpoint, options);
    if (!transactionResponse.ok) {
        throw error(transactionResponse.status, transactionResponse.statusText);
    }
    return await transactionResponse.json();
}

export async function undoTransaction(transactionID: number) {
    let response = await backendPOST("/undo_transaction", { transaction_id: transactionID }, true);
    if (response.ok) {
      invalidateAll();
      toast.success("Köp ångrat");
    } else {
      toast.error("kunde inte ångra köp: " + response.statusText);
    }
  }


export async function getProducts(fetch: svelteFetch, onlyAvailable: boolean) {
    if (import.meta.env.SSR) {
        return { products: [] }
    }
    let response = await fetch('/api/get_products');
    if (!response.ok) {
        throw error(response.status, response.statusText);
    }
    let products = await response.json();
    
    // Filter cart so removed products dont appear
    let filtered_products = [];
    let filtered_cart: Record<string, number> = {}; 
    for (const p of products) {
        if (!(onlyAvailable && p.stock == null)) {
            filtered_products.push(p);
            if (cart.products[p.id]) {
                filtered_cart[p.id] = cart.products[p.id];
            }
        }
    }
    cart.products = filtered_cart;

    return {
        products: filtered_products
    } 
}

export function getDateString(dbDateString: string) {
  return parseAbsoluteToLocal(dbDateString).toDate().toLocaleDateString("sv-SE");
}

export function getSeconds(dbDateString: string) {
    return Math.floor(new Date(dbDateString).getTime() / 1000);
}

type SearchStore<T extends object> = {
    data: T[],
    filtered: T[],
    searchBy: (keyof T)[],
}

export function createSearchStore<T extends object>(data: T[], searchBy: (keyof T)[]): SearchStore<T> {
    return {
        data: data,
        filtered: data,
        searchBy,
    }
}

export function searchData<T extends object>(store: SearchStore<T>, searchTerm: string) {
    store.filtered = store.data.filter(item => {
        for (var key of store.searchBy) {
            if (String(item[key]).toLowerCase().includes(searchTerm.toLowerCase())) {
                return true; 
            }
        }
        return false;
    })     
}

export function updateSearchStore<T extends object>(store: SearchStore<T>, newData: T[]) {
    store.data = newData;
    store.filtered = newData;
}

export async function fetchJSON(fetch: svelteFetch, url: string) {
    const resp = await fetch(url);
    if (!resp.ok) {
        if (resp.status == 404) throw error(resp.status, url)
        throw error(resp.status, resp.statusText);
    }
    return resp.json();
}

export async function backendPOST(endpoint: string, payload: any, json: boolean) {
    let options: RequestInit = {
        method: "POST",
        credentials: "include"
    };
    if (json) {
        options.body = JSON.stringify(payload);
        options.headers = { "Content-Type": "application/json" };
    } else {
        options.body = payload;
    }
    return fetch("/api" + endpoint, options)
}

