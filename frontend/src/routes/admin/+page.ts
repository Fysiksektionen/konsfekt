import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    const timeRange = {
    }
    return {
        bestSellingProd: await fetch("/api/stats/best_selling_product").then(resp => resp.json()),
        productTransactions: await fetch("/api/stats/product_transactions").then(resp => resp.json())
    }
};
