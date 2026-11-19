<script lang="ts">
    import { onMount } from "svelte";
    import {
        getStatsByAccount,
        type AccountStats,
    } from "$lib/services/statistics";
    import {
        getMovementsReport,
        getMonthlyBalanceReport,
        type MonthlyBalanceReport,
    } from "$lib/services/reports";
    import ExportButtons from "$lib/components/ExportButtons.svelte";
    import type { ExportColumn } from "$lib/utils/export";
    import { formatCount, formatMoney } from "$lib/utils/money";

    type Tab = "balances" | "movements" | "monthly";

    const TABS: { value: Tab; label: string }[] = [
        { value: "balances", label: "Saldos actuales" },
        { value: "movements", label: "Movimientos" },
        { value: "monthly", label: "Saldo mensual" },
    ];

    const MONTH_NAMES = [
        "Enero",
        "Febrero",
        "Marzo",
        "Abril",
        "Mayo",
        "Junio",
        "Julio",
        "Agosto",
        "Septiembre",
        "Octubre",
        "Noviembre",
        "Diciembre",
    ];

    let tab = $state<Tab>("balances");
    let loading = $state(false);
    let error = $state<string | null>(null);

    let allBalances = $state<AccountStats[]>([]);
    let balances = $state<AccountStats[]>([]);
    let balanceFilters = $state({ account: "", currency: "" });

    const now = new Date();
    const monthStart = new Date(now.getFullYear(), now.getMonth(), 1);
    const monthEnd = new Date(now.getFullYear(), now.getMonth() + 1, 0);
    function toDateInput(d: Date): string {
        return d.toISOString().slice(0, 10);
    }
    let movementsFrom = $state(toDateInput(monthStart));
    let movementsTo = $state(toDateInput(monthEnd));
    let allMovements = $state<AccountStats[]>([]);
    let movements = $state<AccountStats[]>([]);
    let movementFilters = $state({ account: "", currency: "" });

    let selectedYear = $state(now.getFullYear());
    let selectedMonth = $state(now.getMonth() + 1);
    let allMonthly = $state<MonthlyBalanceReport[]>([]);
    let monthly = $state<MonthlyBalanceReport[]>([]);
    let monthlyFilters = $state({ account: "", currency: "" });
    let exactMatchEnabled = $state(false);

    function fmt(n: number): string {
        return formatMoney(n);
    }

    function normalizeValue(value: string | null | undefined): string {
        return (value ?? "").trim();
    }

    function matchesText(value: string, filterValue: string): boolean {
        if (!filterValue) return true;
        const normalizedValue = value.toLowerCase();
        const normalizedFilter = filterValue.toLowerCase();
        if (exactMatchEnabled) {
            return normalizedValue === normalizedFilter;
        }
        return normalizedValue.includes(normalizedFilter);
    }

    function matchesAccountFilter(
        accountName: string,
        accountNumber: string,
        filterValue: string,
    ): boolean {
        if (!filterValue) return true;
        const candidates = [
            accountName,
            accountNumber,
            `${accountName} (${accountNumber})`,
            `${accountName} ${accountNumber}`,
        ];
        return candidates.some((candidate) =>
            matchesText(candidate, filterValue),
        );
    }

    function applyBalanceFilters() {
        const accountFilter = normalizeValue(balanceFilters.account);
        const currencyFilter = normalizeValue(balanceFilters.currency);
        balances = allBalances.filter((account) => {
            const matchesAccount = matchesAccountFilter(
                account.account_name,
                account.account_number,
                accountFilter,
            );
            const matchesCurrency = matchesText(
                account.currency,
                currencyFilter,
            );
            return matchesAccount && matchesCurrency;
        });
    }

    function applyMovementFilters() {
        const accountFilter = normalizeValue(movementFilters.account);
        const currencyFilter = normalizeValue(movementFilters.currency);
        movements = allMovements.filter((account) => {
            const matchesAccount = matchesAccountFilter(
                account.account_name,
                account.account_number,
                accountFilter,
            );
            const matchesCurrency = matchesText(
                account.currency,
                currencyFilter,
            );
            return matchesAccount && matchesCurrency;
        });
    }

    function applyMonthlyFilters() {
        const accountFilter = normalizeValue(monthlyFilters.account);
        const currencyFilter = normalizeValue(monthlyFilters.currency);
        monthly = allMonthly.filter((item) => {
            const matchesAccount = matchesAccountFilter(
                item.account_name,
                item.account_number,
                accountFilter,
            );
            const matchesCurrency = matchesText(item.currency, currencyFilter);
            return matchesAccount && matchesCurrency;
        });
    }

    function handleError(e: unknown) {
        console.error(e);
        if (e instanceof Error) error = e.message;
        else if (typeof e === "string") error = e;
        else error = JSON.stringify(e);
    }

    async function loadBalances() {
        loading = true;
        error = null;
        try {
            allBalances = await getStatsByAccount();
            applyBalanceFilters();
        } catch (e) {
            handleError(e);
        } finally {
            loading = false;
        }
    }

    async function loadMovements() {
        loading = true;
        error = null;
        try {
            allMovements = await getMovementsReport({
                from: movementsFrom,
                to: movementsTo,
            });
            applyMovementFilters();
        } catch (e) {
            handleError(e);
        } finally {
            loading = false;
        }
    }

    async function loadMonthly() {
        loading = true;
        error = null;
        try {
            allMonthly = await getMonthlyBalanceReport({
                year: selectedYear,
                month: selectedMonth,
            });
            applyMonthlyFilters();
        } catch (e) {
            handleError(e);
        } finally {
            loading = false;
        }
    }

    function selectTab(t: Tab) {
        tab = t;
        error = null;
        if (t === "balances" && allBalances.length === 0) loadBalances();
        if (t === "movements" && allMovements.length === 0) loadMovements();
        if (t === "monthly" && allMonthly.length === 0) loadMonthly();
    }

    onMount(loadBalances);

    const balancesColumns: ExportColumn<AccountStats>[] = [
        {
            header: "Tesorería",
            accessor: (a) => `(${a.account_number}) ${a.account_name}`,
        },
        { header: "Moneda", accessor: (a) => a.currency },
        { header: "Saldo actual", accessor: (a) => fmt(a.balance) },
    ];

    const movementsColumns: ExportColumn<AccountStats>[] = [
        {
            header: "Tesorería",
            accessor: (a) => `(${a.account_number}) ${a.account_name}`,
        },
        { header: "Moneda", accessor: (a) => a.currency },
        {
            header: "Entradas",
            accessor: (a) => fmt(a.total_deposits + a.total_transfers_in),
        },
        {
            header: "Salidas",
            accessor: (a) => fmt(a.total_withdrawals + a.total_transfers_out),
        },
        { header: "Variación neta", accessor: (a) => fmt(a.balance) },
        { header: "Operaciones", accessor: (a) => formatCount(a.transaction_count) },
    ];

    const monthlyColumns: ExportColumn<MonthlyBalanceReport>[] = [
        {
            header: "Tesorería",
            accessor: (m) => `(${m.account_number}) ${m.account_name}`,
        },
        { header: "Moneda", accessor: (m) => m.currency },
        { header: "Saldo inicial", accessor: (m) => fmt(m.opening_balance) },
        { header: "Saldo final", accessor: (m) => fmt(m.closing_balance) },
        { header: "Variación", accessor: (m) => fmt(m.net_change) },
    ];
</script>

<main class="page">
    <div class="page-header">
        <div>
            <h1 class="page-title">Informes</h1>
            <p class="page-subtitle">
                Saldos, movimientos y balances mensuales listos para exportar
            </p>
        </div>
    </div>

    <div class="flex flex-wrap gap-2">
        {#each TABS as t (t.value)}
            <button
                type="button"
                onclick={() => selectTab(t.value)}
                class="rounded-lg border px-3 py-2 text-sm font-medium transition-all {tab ===
                t.value
                    ? 'border-indigo-400/40 bg-indigo-500/15 text-indigo-100'
                    : 'border-zinc-800 bg-zinc-900/50 text-zinc-400 hover:border-zinc-700 hover:bg-zinc-800/70 hover:text-zinc-200'}"
            >
                {t.label}
            </button>
        {/each}
    </div>

    {#if error}
        <p
            class="rounded-lg border border-red-500/20 bg-red-500/10 px-3 py-2 text-sm text-red-300"
        >
            {error}
        </p>
    {/if}

    <!-- Informe 1: saldos actuales -->
    {#if tab === "balances"}
        <div class="flex justify-end">
            <ExportButtons
                title="Saldos actuales"
                data={balances}
                columns={balancesColumns}
                filename="saldos-actuales"
            />
        </div>
        <div class="filter-bar">
            <button
                type="button"
                class={`btn text-sm ${exactMatchEnabled ? "border-amber-500/40 bg-amber-500/10 text-amber-300" : ""}`}
                onclick={() => {
                    exactMatchEnabled = !exactMatchEnabled;
                    applyBalanceFilters();
                }}
            >
                {exactMatchEnabled
                    ? "Coincidencia exacta: ON"
                    : "Coincidencia exacta: OFF"}
            </button>
            <input
                class="field min-w-44"
                placeholder="Tesorería"
                bind:value={balanceFilters.account}
                oninput={applyBalanceFilters}
            />
            <input
                class="field min-w-28"
                placeholder="Moneda"
                bind:value={balanceFilters.currency}
                oninput={applyBalanceFilters}
            />
        </div>

        <div class="table-wrap overflow-x-auto">
            {#if loading}
                <p class="p-6 text-sm text-zinc-400">Cargando…</p>
            {:else if balances.length === 0}
                <div class="empty-state m-0 rounded-none border-0 bg-transparent">
                    No hay tesorerías
                </div>
            {:else}
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>Tesorería</th>
                            <th>Moneda</th>
                            <th class="text-right">Saldo actual</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each balances as b (b.account_id)}
                            <tr>
                                <td>
                                    <div class="flex items-center gap-2">
                                        <span
                                            class="h-2 w-2 shrink-0 rounded-full"
                                            style:background-color={b.color ??
                                                "#71717a"}
                                        ></span>
                                        ({b.account_number}) {b.account_name}
                                    </div>
                                </td>
                                <td>{b.currency}</td>
                                <td class="text-right font-semibold"
                                    >{fmt(b.balance)}</td
                                >
                            </tr>
                        {/each}
                    </tbody>
                </table>
            {/if}
        </div>
    {/if}

    <!-- Informe 2: movimientos en un rango -->
    {#if tab === "movements"}
        <div class="flex flex-wrap items-end justify-between gap-3">
            <div class="flex flex-wrap items-end gap-3">
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Desde</span>
                    <input
                        class="field"
                        type="date"
                        bind:value={movementsFrom}
                    />
                </label>
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Hasta</span>
                    <input
                        class="field"
                        type="date"
                        bind:value={movementsTo}
                    />
                </label>
                <button type="button" onclick={loadMovements} class="btn-primary">
                    Aplicar
                </button>
            </div>
            <ExportButtons
                title="Movimientos ({movementsFrom} a {movementsTo})"
                data={movements}
                columns={movementsColumns}
                filename="movimientos-{movementsFrom}_a_{movementsTo}"
            />
        </div>

        <div class="filter-bar">
            <button
                type="button"
                class={`btn text-sm ${exactMatchEnabled ? "border-amber-500/40 bg-amber-500/10 text-amber-300" : ""}`}
                onclick={() => {
                    exactMatchEnabled = !exactMatchEnabled;
                    applyMovementFilters();
                }}
            >
                {exactMatchEnabled
                    ? "Coincidencia exacta: ON"
                    : "Coincidencia exacta: OFF"}
            </button>
            <input
                class="field min-w-44"
                placeholder="Tesorería"
                bind:value={movementFilters.account}
                oninput={applyMovementFilters}
            />
            <input
                class="field min-w-28"
                placeholder="Moneda"
                bind:value={movementFilters.currency}
                oninput={applyMovementFilters}
            />
        </div>

        <div class="table-wrap overflow-x-auto">
            {#if loading}
                <p class="p-6 text-sm text-zinc-400">Cargando…</p>
            {:else if movements.length === 0}
                <div class="empty-state m-0 rounded-none border-0 bg-transparent">
                    No hay datos en este rango
                </div>
            {:else}
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>Tesorería</th>
                            <th>Moneda</th>
                            <th class="text-right">Entradas</th>
                            <th class="text-right">Salidas</th>
                            <th class="text-right">Variación neta</th>
                            <th class="text-right">Operaciones</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each movements as m (m.account_id)}
                            <tr>
                                <td>
                                    <div class="flex items-center gap-2">
                                        <span
                                            class="h-2 w-2 shrink-0 rounded-full"
                                            style:background-color={m.color ??
                                                "#71717a"}
                                        ></span>
                                        ({m.account_number}) {m.account_name}
                                    </div>
                                </td>
                                <td>{m.currency}</td>
                                <td class="text-right text-emerald-400">
                                    {fmt(
                                        m.total_deposits + m.total_transfers_in,
                                    )}
                                </td>
                                <td class="text-right text-red-400">
                                    {fmt(
                                        m.total_withdrawals +
                                            m.total_transfers_out,
                                    )}
                                </td>
                                <td
                                    class="text-right font-semibold {m.balance >=
                                    0
                                        ? 'text-emerald-400'
                                        : 'text-red-400'}"
                                >
                                    {fmt(m.balance)}
                                </td>
                                <td class="text-right text-zinc-400"
                                    >{formatCount(m.transaction_count)}</td
                                >
                            </tr>
                        {/each}
                    </tbody>
                </table>
            {/if}
        </div>
    {/if}

    <!-- Informe 3: saldo inicial y final del mes -->
    {#if tab === "monthly"}
        <div class="flex flex-wrap items-end justify-between gap-3">
            <div class="flex flex-wrap items-end gap-3">
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Mes</span>
                    <select class="field" bind:value={selectedMonth}>
                        {#each MONTH_NAMES as name, i (i)}
                            <option value={i + 1}>{name}</option>
                        {/each}
                    </select>
                </label>
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Año</span>
                    <input
                        class="field w-28"
                        type="number"
                        bind:value={selectedYear}
                    />
                </label>
                <button type="button" onclick={loadMonthly} class="btn-primary">
                    Aplicar
                </button>
            </div>
            <ExportButtons
                title="Saldo mensual {MONTH_NAMES[
                    selectedMonth - 1
                ]} {selectedYear}"
                data={monthly}
                columns={monthlyColumns}
                filename="saldo-mensual-{selectedYear}-{String(
                    selectedMonth,
                ).padStart(2, '0')}"
            />
        </div>

        <div class="filter-bar">
            <button
                type="button"
                class={`btn text-sm ${exactMatchEnabled ? "border-amber-500/40 bg-amber-500/10 text-amber-300" : ""}`}
                onclick={() => {
                    exactMatchEnabled = !exactMatchEnabled;
                    applyMonthlyFilters();
                }}
            >
                {exactMatchEnabled
                    ? "Coincidencia exacta: ON"
                    : "Coincidencia exacta: OFF"}
            </button>
            <input
                class="field min-w-44"
                placeholder="Tesorería"
                bind:value={monthlyFilters.account}
                oninput={applyMonthlyFilters}
            />
            <input
                class="field min-w-28"
                placeholder="Moneda"
                bind:value={monthlyFilters.currency}
                oninput={applyMonthlyFilters}
            />
        </div>

        <div class="table-wrap overflow-x-auto">
            {#if loading}
                <p class="p-6 text-sm text-zinc-400">Cargando…</p>
            {:else if monthly.length === 0}
                <div class="empty-state m-0 rounded-none border-0 bg-transparent">
                    No hay tesorerías
                </div>
            {:else}
                {#if !monthly[0].month_finished}
                    <p
                        class="border-b border-zinc-800 px-3 py-2 text-xs text-zinc-500"
                    >
                        El mes aún no ha finalizado — el saldo final corresponde
                        al día de hoy ({monthly[0].period_end}).
                    </p>
                {/if}
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>Tesorería</th>
                            <th>Moneda</th>
                            <th class="text-right">Saldo inicial</th>
                            <th class="text-right">Saldo final</th>
                            <th class="text-right">Variación</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each monthly as m (m.account_id)}
                            <tr>
                                <td>
                                    <div class="flex items-center gap-2">
                                        <span
                                            class="h-2 w-2 shrink-0 rounded-full"
                                            style:background-color={m.color ??
                                                "#71717a"}
                                        ></span>
                                        ({m.account_number}) {m.account_name}
                                    </div>
                                </td>
                                <td>{m.currency}</td>
                                <td class="text-right"
                                    >{fmt(m.opening_balance)}</td
                                >
                                <td class="text-right font-semibold"
                                    >{fmt(m.closing_balance)}</td
                                >
                                <td
                                    class="text-right {m.net_change >= 0
                                        ? 'text-emerald-400'
                                        : 'text-red-400'}"
                                >
                                    {m.net_change >= 0 ? "+" : ""}{fmt(
                                        m.net_change,
                                    )}
                                </td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            {/if}
        </div>
    {/if}
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
