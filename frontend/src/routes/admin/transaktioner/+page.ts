import type { PageLoad } from './$types';
import { getTransactions } from '$lib/utils';

export const load: PageLoad = async () => {
    return {
        transactions: await getTransactions()
    }
};
