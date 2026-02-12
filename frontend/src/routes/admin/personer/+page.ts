import type { PageLoad } from './$types';
import { type User, Role } from "./schema.js"
import { fetchJSON } from '$lib/utils';

export const load: PageLoad = async ({ fetch }) => {
    let admins: Array<User> = (await fetchJSON(fetch, "/api/get_users?role=" + Role.Admin)).users;
    let maintainers: Array<User> = (await fetchJSON(fetch, "/api/get_users?role=" + Role.Maintainer)).users;
    let users: Array<User> = (await fetchJSON(fetch, "/api/get_users?role=" + Role.User)).users;

    return { admins, maintainers, users }
};
