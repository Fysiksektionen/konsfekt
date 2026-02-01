import type { PageLoad } from './$types';
import { type User, Role } from "./schema.js"

export const load: PageLoad = async ({ fetch }) => {
    
    let admins: Array<User> = (await (await fetch("/api/get_users?role=" + Role.Admin)).json()).users
    let maintainers: Array<User> = (await (await fetch("/api/get_users?role=" + Role.Maintainer)).json()).users
    let users: Array<User> = (await (await fetch("/api/get_users?role=" + Role.User)).json()).users

    return { admins: admins, maintainers: maintainers, users: users }
};
