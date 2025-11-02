import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    return fetch('https://dummyjson.com/products')
        .then(res => res.json())
};
