import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    let products = await fetch('/api/get_products')
            .then(res => res.json());
    return {
        products,
    }
};
