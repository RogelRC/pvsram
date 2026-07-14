<script lang="ts">
    import { onMount } from "svelte";
    import {
        listTransactionsReport,
        type TransactionRecord,
    } from "$lib/services/transactions";
    import EditTransactionButton from "$lib/components/Transactions/EditTransactionButton.svelte";
    import DeleteTransactionButton from "$lib/components/Transactions/DeleteTransactionButton.svelte";
    import ExportButtons from "$lib/components/ExportButtons.svelte";
    import type { ExportColumn } from "$lib/utils/export";

    const TYPE_LABELS: Record<string, string> = {
        deposit: "Ingreso",
        withdrawal: "Extracción",
        transfer: "Transferencia",
    };

    let loading = $state(false);
    let error = $state<string | null>(null);
    let allTransactions = $state<TransactionRecord[]>([]);
    let transactions = $state<TransactionRecord[]>([]);
    let filters = $state({
        account: "",
        destination: "",
        description: "",
        type: "",
        currency: "",
        minAmount: "",
        maxAmount: "",
        from: "",
        to: "",
    });
    let exactMatchEnabled = $state(false);

    function normalizeText(value: string | null | undefined): string {
        return (value ?? "")
            .trim()
            .normalize("NFD")
            .replace(/[\u0300-\u036f]/g, "")
            .toLowerCase();
    }

    function matchesDateRange(
        value: string | null | undefined,
        from: string,
        to: string,
    ): boolean {
        if (!value) return true;
        const candidate = new Date(value.replace(" ", "T"));
        if (Number.isNaN(candidate.getTime())) return true;

        const fromDate = from ? new Date(`${from}T00:00:00`) : null;
        const toDate = to ? new Date(`${to}T23:59:59`) : null;

        if (fromDate && candidate < fromDate) return false;
        if (toDate && candidate > toDate) return false;
        return true;
    }

    function matchesText(value: string, filterValue: string): boolean {
        if (!filterValue) return true;
        const normalizedValue = normalizeText(value);
        const normalizedFilter = normalizeText(filterValue);
        if (exactMatchEnabled) {
            return normalizedValue === normalizedFilter;
        }
        return normalizedValue.includes(normalizedFilter);
    }

    function matchesAnyCandidate(
        candidates: Array<string | null | undefined>,
        filterValue: string,
    ): boolean {
        if (!filterValue) return true;
        return candidates.some((candidate) =>
            matchesText(candidate ?? "", filterValue),
        );
    }

    function applyFilters() {
        const normalized = {
            account: normalizeText(filters.account),
            destination: normalizeText(filters.destination),
            description: normalizeText(filters.description),
            currency: normalizeText(filters.currency),
        };

        const minAmount =
            filters.minAmount === "" ? null : Number(filters.minAmount);
        const maxAmount =
            filters.maxAmount === "" ? null : Number(filters.maxAmount);

        transactions = allTransactions.filter((transaction) => {
            const accountCandidates = [
                transaction.account_name,
                transaction.account_number,
                `${transaction.account_name} (${transaction.account_number})`,
                `${transaction.account_name} ${transaction.account_number}`,
            ];
            const destinationCandidates = [
                transaction.related_account_name,
                transaction.related_account_number,
                transaction.related_account_name &&
                transaction.related_account_number
                    ? `${transaction.related_account_name} (${transaction.related_account_number})`
                    : null,
                transaction.related_account_name &&
                transaction.related_account_number
                    ? `${transaction.related_account_name} ${transaction.related_account_number}`
                    : null,
            ];

            if (
                normalized.account &&
                !matchesAnyCandidate(accountCandidates, normalized.account)
            ) {
                return false;
            }
            if (
                normalized.destination &&
                !matchesAnyCandidate(
                    destinationCandidates,
                    normalized.destination,
                )
            ) {
                return false;
            }
            if (
                normalized.description &&
                !matchesText(
                    transaction.description ?? "",
                    normalized.description,
                )
            ) {
                return false;
            }
            if (filters.type && transaction.type !== filters.type) {
                return false;
            }
            if (
                normalized.currency &&
                !matchesText(transaction.currency, normalized.currency)
            ) {
                return false;
            }
            if (minAmount !== null && transaction.amount < minAmount) {
                return false;
            }
            if (maxAmount !== null && transaction.amount > maxAmount) {
                return false;
            }
            if (
                !matchesDateRange(
                    transaction.occurred_at,
                    filters.from,
                    filters.to,
                )
            ) {
                return false;
            }
            return true;
        });
    }

    async function loadData() {
        loading = true;
        error = null;
        try {
            allTransactions = await listTransactionsReport({
                sortBy: "occurred_at",
                sortDir: "desc",
                limit: 4611686018427387904,
            });
            applyFilters();
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

    const exportColumns: ExportColumn<TransactionRecord>[] = [
        {
            header: "Tesorería",
            accessor: (t) => `${t.account_name} (${t.account_number})`,
        },
        {
            header: "Fecha",
            accessor: (t) => formatDate(t.occurred_at),
        },
        {
            header: "Tipo",
            accessor: (t) => TYPE_LABELS[t.type] ?? t.type,
        },
        {
            header: "Destino",
            accessor: (t) =>
                t.related_account_name
                    ? `${t.related_account_name} (${t.related_account_number})`
                    : "—",
        },
        {
            header: "Descripción",
            accessor: (t) => t.description ?? "—",
        },
        {
            header: "Monto",
            accessor: (t) => `${t.amount.toFixed(2)} ${t.currency}`,
        },
    ];

    onMount(loadData);
</script>

<main class="flex flex-col p-4 gap-4">
    <h1 class="text-2xl font-bold">Historial de operaciones</h1>

    {#if error}
        <p class="text-red-400 text-sm">{error}</p>
    {/if}

    <div
        class="flex flex-wrap items-center justify-between gap-3 rounded-lg border border-zinc-800 bg-zinc-900/70 p-3"
    >
        <div class="flex flex-wrap gap-3">
            <button
                type="button"
                class={`rounded-md border px-3 py-1 text-sm transition-colors ${exactMatchEnabled ? "border-amber-500 bg-amber-500/15 text-amber-400" : "border-zinc-700 bg-zinc-800 text-zinc-300"}`}
                onclick={() => {
                    exactMatchEnabled = !exactMatchEnabled;
                    applyFilters();
                }}
            >
                {exactMatchEnabled
                    ? "Coincidencia exacta: ON"
                    : "Coincidencia exacta: OFF"}
            </button>
            <input
                class="min-w-40 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
                placeholder="Tesorería"
                bind:value={filters.account}
                oninput={applyFilters}
            />
            <input
                class="min-w-40 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
                placeholder="Destino"
                bind:value={filters.destination}
                oninput={applyFilters}
            />
            <input
                class="min-w-40 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
                placeholder="Descripción"
                bind:value={filters.description}
                oninput={applyFilters}
            />
            <select
                class="rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
                bind:value={filters.type}
                onchange={applyFilters}
            >
                <option value="">Todos</option>
                <option value="deposit">Ingreso</option>
                <option value="withdrawal">Extracción</option>
                <option value="transfer">Transferencia</option>
            </select>
            <input
                class="min-w-28 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
                placeholder="Moneda"
                bind:value={filters.currency}
                oninput={applyFilters}
            />
            <input
                class="w-28 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
                type="number"
                step="0.01"
                placeholder="Mín."
                bind:value={filters.minAmount}
                oninput={applyFilters}
            />
            <input
                class="w-28 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
                type="number"
                step="0.01"
                placeholder="Máx."
                bind:value={filters.maxAmount}
                oninput={applyFilters}
            />
            <input
                class="rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
                type="date"
                bind:value={filters.from}
                onchange={applyFilters}
            />
            <input
                class="rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
                type="date"
                bind:value={filters.to}
                onchange={applyFilters}
            />
        </div>
        <ExportButtons
            title="Historial de operaciones"
            data={transactions}
            columns={exportColumns}
            filename="historial-operaciones"
        />
    </div>

    <div class="bg-zinc-900 border border-zinc-800 rounded-lg overflow-hidden">
        {#if loading}
            <p class="p-4 text-zinc-400">Cargando...</p>
        {:else if transactions.length === 0}
            <p class="p-4 text-zinc-400">
                No hay operaciones registradas que coincidan con los filtros
            </p>
        {:else}
            <table class="w-full text-sm">
                <thead class="bg-zinc-800/50 text-zinc-400 text-left">
                    <tr>
                        <th class="p-2">Tesorería</th>
                        <th class="p-2">Fecha</th>
                        <th class="p-2">Tipo</th>
                        <th class="p-2">Destino</th>
                        <th class="p-2">Descripción</th>
                        <th class="p-2 text-right">Monto</th>
                        <th class="p-2 text-right">Acciones</th>
                    </tr>
                </thead>
                <tbody>
                    {#each transactions as t (t.id)}
                        <tr class="border-t border-zinc-800">
                            <td class="p-2">
                                <div class="flex items-center gap-2">
                                    <span
                                        class="w-2 h-2 rounded-full shrink-0"
                                        style:background-color={t.color ??
                                            "#71717a"}
                                    ></span>
                                    {t.account_name} ({t.account_number})
                                </div>
                            </td>
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
                            <td class="p-2">
                                <div
                                    class="flex justify-end items-center gap-2"
                                >
                                    <EditTransactionButton
                                        transaction={t}
                                        onUpdated={loadData}
                                    />
                                    <DeleteTransactionButton
                                        id={t.id}
                                        onDeleted={loadData}
                                    />
                                </div>
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
