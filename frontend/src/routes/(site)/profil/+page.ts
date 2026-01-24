import type { PageLoad } from './$types';
import { getUser } from '$lib/utils';

export const load: PageLoad = async ({ fetch }) => {
    let user = await getUser(fetch);
    let transactionResponse = await fetch("/api/get_transactions?user_id=" + user.id);
    return {
        user,
        transactions: await transactionResponse.json()
    }
};
