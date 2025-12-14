import type { PageLoad } from './$types';
import { superValidate } from "sveltekit-superforms";
import { productFormSchema } from "./product-schema.js";
import { zod4 } from "sveltekit-superforms/adapters";

export const load: PageLoad = async ({ fetch }) => {
    let products = await fetch('/api/get_products')
            .then(res => res.json());
    return {
        products,
        form: await superValidate(zod4(productFormSchema)),
    }
};

