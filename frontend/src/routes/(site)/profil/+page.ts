import type { PageLoad } from './$types';
import { getTransactions, getUser, transactionQueryFromUserId, type TransactionSummary } from '$lib/utils';

export const load: PageLoad = async ({ fetch }) => {
    let user = await getUser(fetch);
    let transactionQuery = transactionQueryFromUserId(user.id)

    let transactions: TransactionSummary[] = await getTransactions(transactionQuery);

    return {
        user,
        transactions
    }
};
