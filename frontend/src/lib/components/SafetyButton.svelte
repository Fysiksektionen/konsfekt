
<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import * as Dialog from "$lib/components/ui/dialog/index.js";

    let { children, action, class: class_name = "" } = $props();
    let show_dialog = $state(false);

</script>

<Button variant="destructive" class={`card ${class_name}`} onclick={() => show_dialog = true}>
    {@render children?.()}
</Button>

<Dialog.Root bind:open={show_dialog}>
    <Dialog.Content>
        <Dialog.Header>
            <Dialog.Title>Är du säker?</Dialog.Title>
            <Dialog.Description>
                Denna handling är <strong>permanent</strong>! Är du säker på att du vill fortsätta?
            </Dialog.Description>
        </Dialog.Header>
        <div>
            <Button variant="outline" onclick={() => { show_dialog = false; action() }}>Ja</Button>
            <Button onclick={() => { show_dialog = false }}>Nej</Button>
        </div>
    </Dialog.Content>
</Dialog.Root>