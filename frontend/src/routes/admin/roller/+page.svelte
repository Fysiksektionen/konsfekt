<script lang="ts">
    
    // import Dialog from "$lib/components/ui/dialog/dialog.svelte";
    
import { Button, buttonVariants } from "$lib/components/ui/button";
import * as Dialog from "$lib/components/ui/dialog/index.js";
import * as Select from "$lib/components/ui/select/index.js";
import { Input } from "$lib/components/ui/input";
import { Label } from "$lib/components/ui/label";
import { Separator } from "$lib/components/ui/separator/index.js";

import UserTable from "./UserTable.svelte";
    
import { get_roles, Role, type User } from "./schema";
    import { Badge } from "$lib/components/ui/badge";
    import { backendPOST } from "$lib/utils";


let { data } = $props();
let admins = $state(data.admins);
let maintainers = $state(data.maintainers);
let users = $state(data.users);


let current_user: User = $state({id: 0, email: "", name: "", role: Role.User, balance: 0})
let show_user_dialog = $state(false)
let user_dialog_data = $state({name: "", balance: "", role: Role.User})

function open_dialog(user: User): void {
    show_user_dialog = true;
    current_user = user;

    user_dialog_data.name = user.name ?? "";
    user_dialog_data.balance = user.balance + ""
    user_dialog_data.role = user.role
}

function save_user_dialog(): void {
    show_user_dialog = false

    let user_list = users;// = current_user.role == Role.User ? users: maintainers;
    if (current_user.role == Role.Admin) {
        user_list = admins;
    }
    else if (current_user.role == Role.Maintainer) {
        user_list = maintainers;
    }
    
    let i = user_list.findIndex(item => item.id == current_user.id)
    let user = user_list[i]
    
    // Updatera frontend
    user.name = user_dialog_data.name;
    user.balance = parseFloat(user_dialog_data.balance);

    if (user.role != user_dialog_data.role) {
        let target = current_user.role != Role.User ? users: maintainers;
        user.role = user_dialog_data.role
        target.push(user_list.splice(i, 1)[0])
    }

    // Updatera backend
    backendPOST("/update_user", {
        id: user.id,
        name: user.name == "" ? null: user.name,
        balance: user.balance,
        role: user.role,
      }, true);
}


</script>

<div>    
    <h2>Admins</h2>
    <Separator />
    <UserTable data={admins} onclick={open_dialog} />
</div>

<div class="p-10">
    <h2>Maintaines</h2>
    <Separator />
    <UserTable data={maintainers} onclick={open_dialog} />
</div>

<div>
    <h2>Users</h2>
    <Separator />
    <UserTable data={users} onclick={open_dialog} />
</div>

<Dialog.Root bind:open={show_user_dialog}>

    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>
                {current_user.email}
                <Badge variant="outline">ID#{current_user.id}</Badge>
            </Dialog.Title>
        <Dialog.Description>
            Ändra användarens uppgifter.
        </Dialog.Description>
        </Dialog.Header>

        <div class="grid gap-4">
            <div class="grid gap-3">
                <Label>Användarnamn</Label>
                <Input bind:value={user_dialog_data.name} defaultValue={current_user.name} />
            </div>

            <div class="grid gap-3">
                <Label>Saldo</Label>
                <Input bind:value={user_dialog_data.balance} defaultValue={current_user.balance} />
            </div>

            {#if data.user.role === Role.Admin}

            <Separator />

            <div class="grid grid-cols-4">
                <Label>Roll:</Label>

                <div class="grid col-span-3">
                    <Select.Root type="single" bind:value={user_dialog_data.role}>
                        <Select.Trigger class="w-[200px]">
                            {get_roles().find((f) => f.value == user_dialog_data.role)?.label ?? "Välj roll"}
                        </Select.Trigger>
                        <Select.Content>
                        <Select.Group>
                            <Select.Label>Roller</Select.Label>
                            {#each get_roles() as role (role.value)}
                                <Select.Item
                                    value={role.value}
                                    label={role.label}
                                    disabled={(role.value != Role.Admin && current_user.role == Role.Admin) || (current_user.role != Role.Admin && role.value == Role.Admin)}
                                >{role.label}
                                </Select.Item>
                            {/each}
                        </Select.Group>
                        </Select.Content>
                    </Select.Root>
                </div>
            </div>
            <Button variant="destructive" class="max-w-35">Ta bort användare</Button>
            { /if }
        </div>
        <Dialog.Footer>
            <Button class={buttonVariants({ variant: "outline" })} onclick={() => show_user_dialog = false}>Avbryt</Button>
            <Button onclick={save_user_dialog}>Updatera Användare</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
