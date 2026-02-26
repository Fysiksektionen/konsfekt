<script lang="ts">
  import { Button, buttonVariants } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator/index.js";
      
  import { get_roles, Role, type User } from "./schemas";
      import { Badge } from "$lib/components/ui/badge";
      import { backendPOST } from "$lib/utils";
      import SafetyButton from "$lib/components/SafetyButton.svelte";
      import { toast } from "svelte-sonner";
    import { invalidateAll } from "$app/navigation";

  
  let { iAmAdmin, show = $bindable(), user = $bindable() }: { 
    iAmAdmin: boolean,
    show: boolean,
    user: User,
  } = $props();

  async function save_user_dialog(): Promise<void> {
      show = false

      // Updatera backend
      let resp = await backendPOST("/update_user", {
          id: user.id,
          name: user.name == "" ? null: user.name,
          balance: user.balance,
          role: user.role,
        }, true);
      if (!resp.ok) {
        toast.error("Kunde inte uppdatera användare: " + resp.statusText);
      }

      invalidateAll();
  }
  
  async function delete_current_user(): Promise<void> {
      show = false;

      let resp = await backendPOST("/delete_user?id=" + user.id, {}, true);
      if (!resp.ok) {
        toast.error("Kunde inte ta bort användare: " + resp.statusText);
        return;
      }

      invalidateAll();
  }

</script>

<Dialog.Root bind:open={show}>
    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>
                {user.email}
                <Badge variant="outline">ID#{user.id}</Badge>
            </Dialog.Title>
        <Dialog.Description>
            Ändra användarens uppgifter.
        </Dialog.Description>
        </Dialog.Header>

        <div class="grid gap-4">
            <div class="grid gap-3">
                <Label>Användarnamn</Label>
                <Input bind:value={user.name} defaultValue={user.name} />
            </div>

            <div class="grid gap-3">
                <Label>Saldo</Label>
                <Input bind:value={user.balance} defaultValue={user.balance} type="number" />
            </div>

            {#if iAmAdmin}

            <Separator />

            <div class="grid grid-cols-4">
                <Label>Roll:</Label>

                <div class="grid col-span-3">
                    <Select.Root type="single" bind:value={user.role}>
                        <Select.Trigger class="w-[200px]">
                            {get_roles().find((f: {value: string}) => f.value == user.role)?.label ?? "Välj roll"}
                        </Select.Trigger>
                        <Select.Content>
                        <Select.Group>
                            <Select.Label>Roller</Select.Label>
                            {#each get_roles() as role (role.value)}
                                <Select.Item
                                    value={role.value}
                                    label={role.label}
                                    disabled={(role.value != Role.Admin && user.role == Role.Admin) || (user.role != Role.Admin && role.value == Role.Admin)}
                                >{role.label}
                                </Select.Item>
                            {/each}
                        </Select.Group>
                        </Select.Content>
                    </Select.Root>
                </div>
            </div>

            {#if user.role != Role.Admin}
                <SafetyButton action={delete_current_user} class="max-w-35">Ta bort användare</SafetyButton>
            { /if }

            { /if }
        </div>
        <Dialog.Footer>
            <Button class={buttonVariants({ variant: "outline" })} onclick={() => show = false }>Avbryt</Button>
            <Button onclick={save_user_dialog}>Updatera Användare</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
