<script lang="ts">
import * as Select from "$lib/components/ui/select/index.js";
import { Input } from "$lib/components/ui/input";
import { Label } from "$lib/components/ui/label";
import { Separator } from "$lib/components/ui/separator/index.js";

import UserTable from "./UserTable.svelte";
    
import { Role, search_filter_label, SearchFilter, type User, type Filter } from "$lib/components/schemas";
    import UserDialog from "$lib/components/UserDialog.svelte";

let { data } = $props();
let admins = $derived(data.admins);
let maintainers = $derived(data.maintainers);
let users = $derived(data.users);

let currentUser: User = $state({id: 0, email: "", name: "", role: Role.User, balance: 0})
let showDialog = $state(false);

let search_filter: Filter = $state({ search_term: "", search_filter: SearchFilter.Name });

function open_dialog(user: User): void {
    showDialog = true;
    currentUser = user;
}

</script>

<UserDialog iAmAdmin={data.user.role == Role.Admin} bind:user={currentUser} bind:show={showDialog}/>

<div>    
    <h2>Administratörer</h2>
    <Separator />
    <div class="pt-2">
        <UserTable data={admins} onclick={open_dialog} />
    </div>
</div>

<div class="mt-5 mb-5">
    <h2>Underhållare</h2>
    <Separator />
    <div class="pt-2">
        <UserTable data={maintainers} onclick={open_dialog} />
    </div>
</div>

<div>
    <h2>Användare</h2>
    <Separator />

    <div class="p-2 pl-0 flex flex-row">
        <Input placeholder="Sök efter användare..." class="max-w-sm mr-1" bind:value={search_filter.search_term}/>
        <Label class="ml-2 mr-2">filter:</Label>

        <Select.Root type="single" bind:value={search_filter.search_filter}>
        <Select.Trigger class="w-[100px]">
            {search_filter_label(search_filter.search_filter)}
        </Select.Trigger>
            <Select.Content>
            <Select.Group>
                <!-- <Select.Label>Filter</Select.Label> -->
                <Select.Item value="name" label="Namn">Namn</Select.Item>
                <Select.Item value="email" label="Email">Email</Select.Item>
                <Select.Item value="id" label="Id">Id</Select.Item>
            </Select.Group>
            </Select.Content>
        </Select.Root>
    </div>

    <UserTable data={users} onclick={open_dialog} filter={search_filter}/>
</div>
