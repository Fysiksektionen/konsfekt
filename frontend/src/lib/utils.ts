import { error, redirect } from "@sveltejs/kit";
import { clsx, type ClassValue } from "clsx";
import { onMount } from "svelte";
import { writable } from "svelte/store";
import { twMerge } from "tailwind-merge";
import { cart, type Cart } from "./storage.svelte";

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
            user: {
                id: -1,
                email: "",
                balance: 0
            }
        }
    }
    const response = await fetch("/api/get_user");
    if (response.status == 401) {
        console.log(response)
        console.log(await response.text())
        redirect(302, "/login");
    }
    if (!response.ok) {
        throw error(response.status)
    }
    return {
        user: await response.json()
    }
}

export async function getProducts(fetch: svelteFetch, onlyAvailable: boolean) {
    if (import.meta.env.SSR) {
        return { products: [] }
    }
    let response = await fetch('/api/get_products');
    let products = await response.json();
    
    // Filter cart so removed products dont appear
    let filtered_products = [];
    let filtered_cart_products: Record<string, number> = {}; 
    for (const p of products) {
        if (!(onlyAvailable && p.stock == null)) {
            filtered_products.push(p);
            filtered_cart_products[p.id] = cart.products[p.id];
        }
    }
    cart.products = filtered_cart_products;

    return {
        products: filtered_products
    } 
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

