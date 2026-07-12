<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import { backendPOST, fetchJSON } from "$lib/utils";
    import { toast } from "svelte-sonner";
    import type { PageProps } from "../$types";
    import Input from "$lib/components/ui/input/input.svelte";

    let { data }: PageProps = $props();
    
    let amount = $state(30)

    let paymentStatus = $state("")
    
    async function initiatePayment(amount: number) {
      let response = await backendPOST(`/payment/swish/create_payment_request?amount=${amount}`, {}, true);
      if (!response.ok) {
        toast.error(`${response.statusText} (${await response.text()})`);
        return
      }
      const paymentRequestRespone = await response.json();
      const callbackUrl = `${window.location.origin}/swish?payment_id=${paymentRequestRespone.id}`
      window.location.href = `swish://paymentrequest?token=${paymentRequestRespone.token}&callbackurl=${callbackUrl}`;
    }

    async function getPaymentStatus(payment_id: string) {
      let response = await fetchJSON(fetch, `/api/payment/status/${payment_id}`);
      paymentStatus = response 
    }


</script>


{#if data.payment_id}
  <h1>{data.user.name}</h1>
  <p>has the active payment request {data.payment_id} with status {paymentStatus}</p>
  <Button onclick={() => getPaymentStatus(data.payment_id)}></Button>
{:else}
  <Input type="number" placeholder="30" bind:value={amount}/>
  
  <Button onclick={() => initiatePayment(amount)} class="text-2xl text-card-foreground" variant="secondary">Testa swish</Button>
{/if}

