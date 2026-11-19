<script lang="ts">
    import { onMount } from "svelte";
    import {
        countTransactionsReport,
        listTransactionsReport,
        type TransactionFilters,
        type TransactionRecord,
    } from "$lib/services/transactions";
    import EditTransactionButton from "$lib/components/Transactions/EditTransactionButton.svelte";
    import DeleteTransactionButton from "$lib/components/Transactions/DeleteTransactionButton.svelte";
    import ExportButtons from "$lib/components/ExportButtons.svelte";
    import type { ExportColumn } from "$lib/utils/export";
    import { formatCount, formatMoney } from "$lib/utils/money";

    const TYPE_LABELS: Record<string, string> = {
        deposit: "Ingreso",
        withdrawal: "Extracción",
        transfer: "Transferencia",
    };

    const PAGE_SIZE_OPTIONS = [25, 50, 100, 200, 500];

    let loading = $state(false);
    let error = $state<string | null>(null);
    let transactions = $state<TransactionRecord[]>([]);
    let totalCount = $state(0);
    let page = $state(1);
    let pageSize = $state(50);
    let loadSeq = 0;
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

    const totalPages = $derived(Math.max(1, Math.ceil(totalCount / pageSize)));
    const pageStart = $derived(totalCount === 0 ? 0 : (page - 1) * pageSize + 1);
    const pageEnd = $derived(Math.min(page * pageSize, totalCount));

    function buildFilters(
        overrides: Partial<TransactionFilters> = {},
    ): TransactionFilters {
        const f: TransactionFilters = {
            sortBy: "occurred_at",
            sortDir: "desc",
            exactMatch: exactMatchEnabled,
            ...overrides,
        };

        if (filters.account.trim()) f.accountQuery = filters.account.trim();
        if (filters.destination.trim()) {
            f.destinationQuery = filters.destination.trim();
        }
        if (filters.concept.trim()) f.concept = filters.concept.trim();
        if (filters.comment.trim()) f.comment = filters.comment.trim();
        if (filters.type) f.type = filters.type as TransactionFilters["type"];
        if (filters.currency.trim()) f.currency = filters.currency.trim();
        if (filters.minAmount !== "") f.amountMin = Number(filters.minAmount);
        if (filters.maxAmount !== "") f.amountMax = Number(filters.maxAmount);
        if (filters.from) f.from = `${filters.from} 00:00:00`;
        if (filters.to) f.to = `${filters.to} 23:59:59`;

        return f;
    }

    async function loadPage() {
        const seq = ++loadSeq;
        loading = true;
        error = null;
        try {
            const base = buildFilters();
            const count = await countTransactionsReport(base);
            if (seq !== loadSeq) return;

            totalCount = count;
            const maxPage = Math.max(1, Math.ceil(count / pageSize));
            if (page > maxPage) page = maxPage;

            if (count === 0) {
                transactions = [];
            } else {
                const rows = await listTransactionsReport({
                    ...base,
                    limit: pageSize,
                    offset: (page - 1) * pageSize,
                });
                if (seq !== loadSeq) return;
                transactions = rows;
            }
        } catch (e) {
            if (seq !== loadSeq) return;
            error = e instanceof Error ? e.message : String(e);
        } finally {
            if (seq === loadSeq) loading = false;
        }
    }

    function onFiltersChanged() {
        page = 1;
        void loadPage();
    }

    function onPageSizeChange() {
        pageSize = Number(pageSize);
        page = 1;
        void loadPage();
    }

    function goToPage(next: number) {
        const clamped = Math.min(Math.max(1, next), totalPages);
        if (clamped === page) return;
        page = clamped;
        void loadPage();
    }

    async function fetchAllForExport(): Promise<TransactionRecord[]> {
        const base = buildFilters({ sortBy: "occurred_at", sortDir: "desc" });
        const total = await countTransactionsReport(base);
        const chunk = 500;
        const all: TransactionRecord[] = [];
        for (let offset = 0; offset < total; offset += chunk) {
            const part = await listTransactionsReport({
                ...base,
                limit: chunk,
                offset,
            });
            all.push(...part);
            if (part.length < chunk) break;
        }
        return all;
    }

    function formatAmount(amount: number, curr: string): string {
        return `${formatMoney(amount)} ${curr}`;
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
            accessor: (t) => `${formatMoney(t.amount)} ${t.currency}`,
        },
    ];

    onMount(loadPage);
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
            fetchAllData={fetchAllForExport}
            canExport={totalCount > 0}
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
                onFiltersChanged();
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
            oninput={onFiltersChanged}
        />
        <input
            class="field min-w-40"
            placeholder="Destino"
            bind:value={filters.destination}
            oninput={onFiltersChanged}
        />
        <input
            class="field min-w-40"
            placeholder="Concepto"
            bind:value={filters.concept}
            oninput={onFiltersChanged}
        />
        <input
            class="field min-w-40"
            placeholder="Comentario"
            bind:value={filters.comment}
            oninput={onFiltersChanged}
        />
        <select class="field" bind:value={filters.type} onchange={onFiltersChanged}>
            <option value="">Todos los tipos</option>
            <option value="deposit">Ingreso</option>
            <option value="withdrawal">Extracción</option>
            <option value="transfer">Transferencia</option>
        </select>
        <input
            class="field min-w-28"
            placeholder="Moneda"
            bind:value={filters.currency}
            oninput={onFiltersChanged}
        />
        <input
            class="field w-28"
            type="number"
            step="0.01"
            placeholder="Mín."
            bind:value={filters.minAmount}
            oninput={onFiltersChanged}
        />
        <input
            class="field w-28"
            type="number"
            step="0.01"
            placeholder="Máx."
            bind:value={filters.maxAmount}
            oninput={onFiltersChanged}
        />
        <input
            class="field"
            type="date"
            bind:value={filters.from}
            onchange={onFiltersChanged}
        />
        <input
            class="field"
            type="date"
            bind:value={filters.to}
            onchange={onFiltersChanged}
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
                                    <span class="badge {typeBadgeClass(t.type)}">
                                        {TYPE_LABELS[t.type] ?? t.type}
                                    </span>
                                </td>
                                <td class="text-zinc-400">
                                    {t.related_account_name
                                        ? `${t.related_account_name} (${t.related_account_number})`
                                        : "—"}
                                </td>
                                <td class="text-zinc-300">{t.concept_name ?? "—"}</td>
                                <td class="text-zinc-400">{t.comment ?? "—"}</td>
                                <td class="text-right font-medium text-zinc-100">
                                    {formatAmount(t.amount, t.currency)}
                                </td>
                                <td>
                                    <div class="flex items-center justify-end gap-1.5">
                                        <EditTransactionButton
                                            transaction={t}
                                            onUpdated={loadPage}
                                        />
                                        <DeleteTransactionButton
                                            id={t.id}
                                            onDeleted={loadPage}
                                        />
                                    </div>
                                </td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>

            <div
                class="flex flex-wrap items-center justify-between gap-3 border-t border-zinc-800 px-4 py-3 text-sm text-zinc-400"
            >
                <div>
                    Mostrando {formatCount(pageStart)}–{formatCount(pageEnd)} de {formatCount(totalCount)} operaciones
                </div>
                <div class="flex flex-wrap items-center gap-2">
                    <label class="flex items-center gap-2">
                        <span>Por página</span>
                        <select
                            class="field w-24 py-1"
                            bind:value={pageSize}
                            onchange={onPageSizeChange}
                        >
                            {#each PAGE_SIZE_OPTIONS as size}
                                <option value={size}>{size}</option>
                            {/each}
                        </select>
                    </label>
                    <button
                        type="button"
                        class="btn px-3 py-1 text-sm"
                        disabled={page <= 1 || loading}
                        onclick={() => goToPage(page - 1)}
                    >
                        Anterior
                    </button>
                    <span class="px-1 text-zinc-300">
                        Página {formatCount(page)} de {formatCount(totalPages)}
                    </span>
                    <button
                        type="button"
                        class="btn px-3 py-1 text-sm"
                        disabled={page >= totalPages || loading}
                        onclick={() => goToPage(page + 1)}
                    >
                        Siguiente
                    </button>
                </div>
            </div>
        {/if}
    </div>
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
