import { fetchJSON } from '$lib/utils';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    return {
        bestSellingProd: await fetchJSON(fetch, "/api/stats/best_selling_product"),
        productTransactions: await fetchJSON(fetch, "/api/stats/product_transactions")
    }
};
