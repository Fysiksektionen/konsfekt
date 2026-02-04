<script lang="ts">
    import { Badge } from "$lib/components/ui/badge";


import * as Table from "$lib/components/ui/table/index.js";
    import { get_roles, type Filter, type User } from "./schema";

let { data, onclick, filter = null } = $props()

function filter_data(data: Array<User>, filter: Filter): Array<User> {
    if (filter != null && filter.search_term != "") {
        return data.filter((user: User) => ("" + user[filter.search_filter])
            .toLowerCase()
            .includes(filter.search_term.toLowerCase()) )
    }
    return data
}

</script>

<div class="rounded-md border">
    <Table.Root>
    <Table.Header>
        <Table.Row>
            <Table.Head>
                <div> Mailadress </div>
            </Table.Head>
            <Table.Head>
                <div> Namn </div>
            </Table.Head>
            <Table.Head>
                <div> Roll </div>
            </Table.Head>
            <Table.Head>
                <Badge variant="outline">ID</Badge>
            </Table.Head>
        </Table.Row>
    </Table.Header>
    <Table.Body>
        {#each filter_data(data, filter) as user}
            <Table.Row onclick={() => {onclick(user)}}>
                <Table.Cell>{user.email}</Table.Cell>
                <Table.Cell>{user.name}</Table.Cell>
                <Table.Cell>{get_roles().find((f: {value: string}) => f.value == user.role)?.label}</Table.Cell>
                <Table.Cell>{user.id}</Table.Cell>
            </Table.Row>
        {/each}
        {#if data.length == 0}
            <div class="m-10 w-full">
                <p class="text-center mx-auto w-1/2">Just nu finns det inga underhållare, för att lägga till en underhållare, klicka på en avnändare och välj rollen "Underhållare"</p>
            </div>
        {/if}
    </Table.Body>
    </Table.Root>
</div>