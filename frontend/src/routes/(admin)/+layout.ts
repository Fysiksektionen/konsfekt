import { get_user } from '$lib/utils';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ fetch }) => {
    return get_user(fetch)
};
