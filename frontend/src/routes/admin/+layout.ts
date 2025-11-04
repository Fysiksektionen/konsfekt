import { getUser } from '$lib/utils';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ fetch }) => {
    return getUser(fetch)
};
