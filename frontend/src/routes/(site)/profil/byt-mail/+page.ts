import { getUser } from '$lib/utils';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    return {
        user: await getUser(fetch)
    }
};

