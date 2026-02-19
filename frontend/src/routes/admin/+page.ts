import { fetchJSON } from '$lib/utils';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    return {
        bestSellingProd: await fetchJSON(fetch, "/api/stats/best_selling_product"),
        purchasesInfo: await fetchJSON(fetch, "/api/stats/purchases"),
        customerInfo: await fetchJSON(fetch, "/api/stats/customers"),
        depositsInfo: await fetchJSON(fetch, "/api/stats/deposits")
    }
};
