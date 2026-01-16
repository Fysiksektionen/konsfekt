import type { PageLoad } from './$types';
import { superValidate } from "sveltekit-superforms";
import { productFormSchema } from "./product-schema.js";
import { zod4 } from "sveltekit-superforms/adapters";
import { getProducts } from '$lib/utils';

export const load: PageLoad = async ({ fetch }) => {
    let products = await getProducts(fetch, false);
    return {
        ...products,
        form: await superValidate(zod4(productFormSchema)),
    }
};

