<script lang="ts">
    import { goto } from "$app/navigation";
    import Button from "$lib/components/ui/button/button.svelte";
    import Logo from "$lib/components/Logo.svelte";
    import { backendPOST } from "$lib/utils";
    import { toast } from "svelte-sonner";
	  import type { PageProps } from './$types';

	  let { data }: PageProps = $props();
    const transactionCount = data.transactions.filter(tx => !tx.admin_issued).length
    const adminIssuedCount = data.transactions.length - transactionCount;

    async function unlinkTransactions() {
      const response = await backendPOST("/unlink_transactions", {}, true);
      if (response.ok) {
        goto("/profil")
        toast.success("Transaktioner dissocierade")
      } else {
        toast.warning("Något gick fel. Transaktioner kan fortfarande vara kopplade.")
      }
    }
</script>

<div class="mt-20 gap-5 flex flex-col w-4/5 md:w-2/5 items-center">
  <Logo class="w-[250px] h-auto shrink-0"/>
  <div class="w-full flex flex-col gap-3">
    <p>
      Vill du inte att några av dina befintliga transaktioner ska vara kopplade till detta konto? 
      Klicka då på knappen nedan för att dissociera all information kopplat till ditt konto från befintliga transaktioner.
    </p>
    <p>
      Notera att transaktionerna inte försvinner från systemet.
    </p>
  </div>
  <div class="flex gap-3">
    <Button variant="outline" href="/profil">Tillbaka</Button>
    {#if transactionCount == 0}
      <Button variant="destructive" disabled>Inga transaktioner att dissociera</Button>
    {:else}
      <Button variant="destructive" onclick={() => unlinkTransactions()}>Dissociera {transactionCount} transaktioner</Button>
    {/if}
  </div>
  <div class="w-full flex flex-col gap-3">
    {#if adminIssuedCount > 0}
    <p class="text-amber-600"><em>
      En administratör har ändrat ditt saldo. Dessa transaktioner kan du inte koppla bort från ditt konto.
    </em></p>
    {/if}
  </div>
</div>
