import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    const timeRange = {
    }
    return {
        bestSellingProd: await fetch("/api/stats/get_best_selling_product").then(resp => resp.json()),
    }
};
