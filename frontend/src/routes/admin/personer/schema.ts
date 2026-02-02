
export enum Role {
    Admin = "admin",
    Maintainer = "maintainer",
    User = "user"
}

export function get_roles(): Array<{label: string, value: Role}> {
    return [
        {label: "Underhållare", value: Role.Maintainer},
        {label: "Användare", value: Role.User},
        {label: "Administratör", value: Role.Admin},
    ]
}

export type User = {
    id: number,
    name: string,
    email: string,
    role: Role,
    balance: number,
}