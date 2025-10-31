import type { PageLoad } from './$types';
import { get_user } from '$lib/utils';

export const load: PageLoad = async ({ fetch }) => {
    return get_user(fetch)
};
