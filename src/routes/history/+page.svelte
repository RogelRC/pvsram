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
        concept: "",
        comment: "",
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
            concept: normalizeText(filters.concept),
            comment: normalizeText(filters.comment),
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
                normalized.concept &&
                !matchesText(transaction.concept_name ?? "", normalized.concept)
            ) {
                return false;
            }
            if (
                normalized.comment &&
                !matchesText(transaction.comment ?? "", normalized.comment)
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
            accessor: (t) => `(${t.account_number}) ${t.account_name}`,
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
            header: "Concepto",
            accessor: (t) => t.concept_name ?? "—",
        },
        {
            header: "Comentario",
            accessor: (t) => t.comment ?? "—",
        },
        {
            header: "Monto",
            accessor: (t) => `${t.amount.toFixed(2)} ${t.currency}`,
        },
    ];

    onMount(loadData);
</script>

<main class="page">
    <div class="page-header">
        <div>
            <h1 class="page-title">Historial de operaciones</h1>
            <p class="page-subtitle">
                Consulta, filtra y exporta todos los movimientos
            </p>
        </div>
        <ExportButtons
            title="Historial de operaciones"
            data={transactions}
            columns={exportColumns}
            filename="historial-operaciones"
        />
    </div>

    {#if error}
        <p
            class="rounded-lg border border-red-500/20 bg-red-500/10 px-3 py-2 text-sm text-red-300"
        >
            {error}
        </p>
    {/if}

    <div class="filter-bar">
        <button
            type="button"
            class={`btn text-sm ${exactMatchEnabled ? "border-amber-500/40 bg-amber-500/10 text-amber-300" : ""}`}
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
            class="field min-w-40"
            placeholder="Tesorería"
            bind:value={filters.account}
            oninput={applyFilters}
        />
        <input
            class="field min-w-40"
            placeholder="Destino"
            bind:value={filters.destination}
            oninput={applyFilters}
        />
        <input
            class="field min-w-40"
            placeholder="Concepto"
            bind:value={filters.concept}
            oninput={applyFilters}
        />
        <input
            class="field min-w-40"
            placeholder="Comentario"
            bind:value={filters.comment}
            oninput={applyFilters}
        />
        <select
            class="field"
            bind:value={filters.type}
            onchange={applyFilters}
        >
            <option value="">Todos los tipos</option>
            <option value="deposit">Ingreso</option>
            <option value="withdrawal">Extracción</option>
            <option value="transfer">Transferencia</option>
        </select>
        <input
            class="field min-w-28"
            placeholder="Moneda"
            bind:value={filters.currency}
            oninput={applyFilters}
        />
        <input
            class="field w-28"
            type="number"
            step="0.01"
            placeholder="Mín."
            bind:value={filters.minAmount}
            oninput={applyFilters}
        />
        <input
            class="field w-28"
            type="number"
            step="0.01"
            placeholder="Máx."
            bind:value={filters.maxAmount}
            oninput={applyFilters}
        />
        <input
            class="field"
            type="date"
            bind:value={filters.from}
            onchange={applyFilters}
        />
        <input
            class="field"
            type="date"
            bind:value={filters.to}
            onchange={applyFilters}
        />
    </div>

    <div class="table-wrap">
        {#if loading}
            <p class="p-6 text-sm text-zinc-400">Cargando operaciones…</p>
        {:else if transactions.length === 0}
            <div class="empty-state m-0 rounded-none border-0 bg-transparent">
                No hay operaciones registradas que coincidan con los filtros
            </div>
        {:else}
            <div class="overflow-x-auto">
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>Tesorería</th>
                            <th>Fecha</th>
                            <th>Tipo</th>
                            <th>Destino</th>
                            <th>Concepto</th>
                            <th>Comentario</th>
                            <th class="text-right">Monto</th>
                            <th class="text-right">Acciones</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each transactions as t (t.id)}
                            <tr>
                                <td>
                                    <div class="flex items-center gap-2">
                                        <span
                                            class="h-2 w-2 shrink-0 rounded-full"
                                            style:background-color={t.color ??
                                                "#71717a"}
                                        ></span>
                                        <span class="text-zinc-200">
                                            ({t.account_number}) {t.account_name}
                                        </span>
                                    </div>
                                </td>
                                <td class="whitespace-nowrap text-zinc-300">
                                    {formatDate(t.occurred_at)}
                                </td>
                                <td>
                                    <span
                                        class="badge {typeBadgeClass(t.type)}"
                                    >
                                        {TYPE_LABELS[t.type] ?? t.type}
                                    </span>
                                </td>
                                <td class="text-zinc-400">
                                    {t.related_account_name
                                        ? `${t.related_account_name} (${t.related_account_number})`
                                        : "—"}
                                </td>
                                <td class="text-zinc-300"
                                    >{t.concept_name ?? "—"}</td
                                >
                                <td class="text-zinc-400">{t.comment ?? "—"}</td>
                                <td class="text-right font-medium text-zinc-100">
                                    {formatAmount(t.amount, t.currency)}
                                </td>
                                <td>
                                    <div
                                        class="flex items-center justify-end gap-1.5"
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
            </div>
        {/if}
    </div>
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
