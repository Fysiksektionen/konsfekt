<script lang="ts">
    import { Badge } from "$lib/components/ui/badge";


import * as Table from "$lib/components/ui/table/index.js";
    import { get_roles } from "./schema";

let { data, onclick } = $props()

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
        {#each data as row}
            <Table.Row onclick={() => {onclick(row)}}>
                <Table.Cell>{row.email}</Table.Cell>
                <Table.Cell>{row.name}</Table.Cell>
                <Table.Cell>{get_roles().find((f) => f.value == row.role)?.label}</Table.Cell>
                <Table.Cell>{row.id}</Table.Cell>
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