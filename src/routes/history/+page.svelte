<script lang="ts">
    import { onMount } from "svelte";
    import {
        listTransactionsReport,
        type TransactionRecord,
    } from "$lib/services/transactions";

    const TYPE_LABELS: Record<string, string> = {
        deposit: "Ingreso",
        withdrawal: "Extracción",
        transfer: "Transferencia",
    };

    let loading = $state(false);
    let error = $state<string | null>(null);
    let transactions = $state<TransactionRecord[]>([]);

    async function loadData() {
        loading = true;
        error = null;
        try {
            transactions = await listTransactionsReport({
                sortBy: "occurred_at",
                sortDir: "desc",
                limit: 500,
            });
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            loading = false;
        }
    }

    function formatAmount(amount: number, curr: string): string {
        return `${amount.toLocaleString(undefined, {
            minimumFractionDigits: 2,
            maximumFractionDigits: 2,
        })} ${curr}`;
    }

    function formatDate(value: string): string {
        const d = new Date(value.replace(" ", "T"));
        if (Number.isNaN(d.getTime())) return value;
        return d.toLocaleString();
    }

    function typeBadgeClass(t: string): string {
        if (t === "deposit") return "bg-green-500/15 text-green-400";
        if (t === "withdrawal") return "bg-red-500/15 text-red-400";
        return "bg-blue-500/15 text-blue-400";
    }

    onMount(loadData);
</script>

<main class="flex flex-col p-4 gap-4">
    <h1 class="text-2xl font-bold">Historial de operaciones</h1>

    {#if error}
        <p class="text-red-400 text-sm">{error}</p>
    {/if}

    <div class="bg-zinc-900 border border-zinc-800 rounded-lg overflow-hidden">
        {#if loading}
            <p class="p-4 text-zinc-400">Cargando...</p>
        {:else if transactions.length === 0}
            <p class="p-4 text-zinc-400">No hay operaciones registradas</p>
        {:else}
            <table class="w-full text-sm">
                <thead class="bg-zinc-800/50 text-zinc-400 text-left">
                    <tr>
                        <th class="p-2">Fecha</th>
                        <th class="p-2">Tipo</th>
                        <th class="p-2">Cuenta</th>
                        <th class="p-2">Destino</th>
                        <th class="p-2">Descripción</th>
                        <th class="p-2 text-right">Monto</th>
                    </tr>
                </thead>
                <tbody>
                    {#each transactions as t (t.id)}
                        <tr class="border-t border-zinc-800">
                            <td class="p-2 whitespace-nowrap"
                                >{formatDate(t.occurred_at)}</td
                            >
                            <td class="p-2">
                                <span
                                    class="px-2 py-0.5 rounded-full text-xs {typeBadgeClass(
                                        t.type,
                                    )}"
                                >
                                    {TYPE_LABELS[t.type] ?? t.type}
                                </span>
                            </td>
                            <td class="p-2">
                                {t.account_name} ({t.account_number})
                            </td>
                            <td class="p-2 text-zinc-400">
                                {t.related_account_name
                                    ? `${t.related_account_name} (${t.related_account_number})`
                                    : "—"}
                            </td>
                            <td class="p-2 text-zinc-400"
                                >{t.description ?? "—"}</td
                            >
                            <td class="p-2 text-right font-medium">
                                {formatAmount(t.amount, t.currency)}
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {/if}
    </div>
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
