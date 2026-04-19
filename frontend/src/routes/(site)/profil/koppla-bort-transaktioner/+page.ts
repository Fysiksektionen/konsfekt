import type { PageLoad } from './$types';
import { getTransactions, getUser, transactionQueryFromUserId } from '$lib/utils';

export const load: PageLoad = async ({ fetch }) => {
    let user = await getUser(fetch);
    let transactionQuery = transactionQueryFromUserId(user.id)
    return {
        user,
        transactions: await getTransactions(transactionQuery)
    }
};
