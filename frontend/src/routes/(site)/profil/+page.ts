import type { PageLoad } from './$types';
import { getUser } from '$lib/utils';

export const load: PageLoad = async ({ fetch }) => {
    return getUser(fetch)
};
