import { error, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
    const response = await fetch("/api/get_user");
    if (response.status == 401) {
        console.log(response)
        console.log(await response.text())
        redirect(302, "/login");
    }
    if (!response.ok) {
        throw error(response.status)
    }
    return {
        user: await response.json()
    }
};
