<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import { backendPOST, fetchJSON } from "$lib/utils";
    import { toast } from "svelte-sonner";
    import type { PageProps } from "./$types";
    import Input from "$lib/components/ui/input/input.svelte";
    import { onMount } from "svelte";
    import swishLogoLight from "$lib/assets/swish/swish-logo-light-bg.png";
    import swishLogoDark from "$lib/assets/swish/swish-logo-dark-bg.png";

    const MIN_AMOUNT = 30;

    const STATUS_LABELS: Record<string, string> = {
        pending: "Väntar",
        paid: "Betald",
        declined: "Nekad",
        error: "Fel",
        cancelled: "Avbruten",
    };

    let { data }: PageProps = $props();

    let amount = $state(30);
    let paymentStatus = $state("");
    let paidAmount = $state<number | null>(null);
    let balance = $state<number | null>(null);
    let submitting = $state(false);

    let amountError = $derived(
        amount === undefined || amount === null || Number.isNaN(amount)
            ? "Ange ett belopp"
            : amount < MIN_AMOUNT
                ? `Beloppet måste vara minst ${MIN_AMOUNT} kr`
                : ""
    );

    onMount(() => {
        if (!data.payment_id) return;
        const paymentId = data.payment_id;
        let interval: ReturnType<typeof setInterval>;
        const poll = async () => {
            await getPaymentStatus(paymentId);
            if (paymentStatus && paymentStatus !== "pending") {
                clearInterval(interval);
            }
        };
        poll();
        interval = setInterval(poll, 5000);
        return () => clearInterval(interval);
    });

    async function initiatePayment(amount: number) {
        if (amountError) return;
        submitting = true;
        try {
            let response = await backendPOST(`/payment/swish/create_payment_request?amount=${amount}`, {}, true);
            if (!response.ok) {
                toast.error(`${response.statusText} (${await response.text()})`);
                return;
            }
            const paymentRequestRespone = await response.json();
            const callbackUrl = `${window.location.origin}/swish?payment_id=${paymentRequestRespone.payment_id}`;
            window.location.href = `swish://paymentrequest?token=${paymentRequestRespone.token}&callbackurl=${callbackUrl}`;
        } finally {
            submitting = false;
        }
    }

    async function getPaymentStatus(payment_id: string | null) {
        if (!payment_id) return;
        let response = await fetchJSON(fetch, `/api/payment/status/${payment_id}`);
        paymentStatus = response.status;
        paidAmount = response.amount;
        balance = response.balance;
    }
</script>

<div class="mx-auto flex w-full max-w-sm flex-col gap-6 py-10">
    <Card.Root>
        <Card.Header class="flex items-center justify-center py-2">
            <img
                src={swishLogoLight}
                alt="Swish"
                class="block h-10 w-auto dark:hidden"
            />
            <img
                src={swishLogoDark}
                alt="Swish"
                class="hidden h-10 w-auto dark:block"
            />
        </Card.Header>

        <Card.Content>
            {#if data.payment_id}
                <div class="flex flex-col items-center gap-2 text-center">
                    <p class="text-muted-foreground text-sm">
                        Betalningsreferens {data.payment_id}
                    </p>
                    <p class="text-2xl font-semibold tracking-tight">
                        {STATUS_LABELS[paymentStatus] ?? "Väntar..."}
                    </p>
                    {#if paidAmount !== null}
                        <p class="font-mono text-lg">{paidAmount.toFixed(2)} kr</p>
                    {/if}
                    {#if balance !== null && paymentStatus === "paid"}
                        <p class="text-muted-foreground text-xs">Nytt saldo: {balance.toFixed(2)} kr</p>
                    {/if}
                </div>
            {:else}
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-1.5">
                        <label for="amount" class="text-sm font-medium">Belopp</label>
                        <div class="relative">
                            <Input
                                id="amount"
                                type="number"
                                min={MIN_AMOUNT}
                                step="1"
                                placeholder="30"
                                aria-invalid={!!amountError}
                                class="h-14 pr-14 text-right font-mono text-3xl md:text-3xl"
                                bind:value={amount}
                                onfocus={(e) => e.currentTarget.select()}
                            />
                            <span
                                class="text-muted-foreground pointer-events-none absolute inset-y-0 right-4 flex items-center font-mono text-3xl md:text-3xl"
                            >
                                kr
                            </span>
                        </div>
                        {#if amountError}
                            <p class="text-destructive text-xs">{amountError}</p>
                        {/if}
                    </div>

                    <Button
                        onclick={() => initiatePayment(amount)}
                        disabled={!!amountError || submitting}
                        class="w-full text-base"
                        size="lg"
                    >
                        Betala med Swish
                    </Button>
                </div>
            {/if}
        </Card.Content>
    </Card.Root>
</div>
