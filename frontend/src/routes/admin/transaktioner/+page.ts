import type { PageLoad } from './$types';
import { getTransactions } from '$lib/utils';

export const load: PageLoad = async ({ fetch }) => {
    return {
        transactions: await getTransactions(fetch)
    }
};
