import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ url }) => {
    return {
        payment_id: url.searchParams.get("payment_id")
    }
};
