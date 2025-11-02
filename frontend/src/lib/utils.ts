import { error, redirect } from "@sveltejs/kit";
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any } ? Omit<T, "children"> : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & { ref?: U | null };


export async function getUser(fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>) {
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
            if (String(item[key]).toLowerCase().includes(searchTerm)) {
                return true; 
            }
        }
        return false;
    })     
}
