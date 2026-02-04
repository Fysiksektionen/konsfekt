
export enum Role {
    Admin = "admin",
    Maintainer = "maintainer",
    User = "user"
}

export enum SearchFilter {
    Name = "name",
    Email = "email",
    Id = "id",
}

export function search_filter_label(filter: SearchFilter): string {
    switch (filter) {
        case SearchFilter.Name: {
            return "Namn"
        }
        case SearchFilter.Email: {
            return "Email"
        }
        case SearchFilter.Id: {
            return "Id"
        }
        default: return ""

    }
}

export type Filter = {
    search_term: string,
    search_filter: SearchFilter
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