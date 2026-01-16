import { getProducts } from '$lib/utils';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    return getProducts(fetch, true);
};
