<script lang="ts">
    import { goto } from "$app/navigation";
    import Button from "$lib/components/ui/button/button.svelte";
    import { backendPOST } from "$lib/utils";
    import { toast } from "svelte-sonner";
	  import type { PageProps } from './$types';

	  let { data }: PageProps = $props();
    const transactionCount = data.transactions.length

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
  <div class="bg-primary relative text-background rounded-xl w-[250px] h-[100px] p-2">
    <p class="flex justify-center items-center text-5xl w-[280px] h-[75px] absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-primary text-background rounded-xl text-shadow-2xs text-shadow-accent">
      Konsfekt
    </p>
  </div>
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
</div>
