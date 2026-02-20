import type { PageLoad } from './$types';
import { getTransactions, getUser } from '$lib/utils';

export const load: PageLoad = async ({ fetch }) => {
    let user = await getUser(fetch);
    return {
        user,
        // transactions: await getTransactions(fetch, user.id)
        transactions: [] 
    }
};
