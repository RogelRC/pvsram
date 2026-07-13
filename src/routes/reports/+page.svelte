<script lang="ts">
    import { onMount } from "svelte";
    import { getStatsByAccount, type AccountStats } from "$lib/services/statistics";
    import {
        getMovementsReport,
        getMonthlyBalanceReport,
        type MonthlyBalanceReport,
    } from "$lib/services/reports";
    import ExportButtons from "$lib/components/ExportButtons.svelte";
    import type { ExportColumn } from "$lib/utils/export";

    type Tab = "balances" | "movements" | "monthly";

    const TABS: { value: Tab; label: string }[] = [
        { value: "balances", label: "Saldos actuales" },
        { value: "movements", label: "Movimientos" },
        { value: "monthly", label: "Saldo mensual" },
    ];

    const MONTH_NAMES = [
        "Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio",
        "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre",
    ];

    let tab = $state<Tab>("balances");
    let loading = $state(false);
    let error = $state<string | null>(null);

    // Informe 1: saldos actuales
    let balances = $state<AccountStats[]>([]);

    // Informe 2: movimientos
    const now = new Date();
    const monthStart = new Date(now.getFullYear(), now.getMonth(), 1);
    const monthEnd = new Date(now.getFullYear(), now.getMonth() + 1, 0);
    function toDateInput(d: Date): string {
        return d.toISOString().slice(0, 10);
    }
    let movementsFrom = $state(toDateInput(monthStart));
    let movementsTo = $state(toDateInput(monthEnd));
    let movements = $state<AccountStats[]>([]);

    // Informe 3: saldo inicial/final del mes
    let selectedYear = $state(now.getFullYear());
    let selectedMonth = $state(now.getMonth() + 1);
    let monthly = $state<MonthlyBalanceReport[]>([]);

    function fmt(n: number): string {
        return n.toLocaleString(undefined, {
            minimumFractionDigits: 2,
            maximumFractionDigits: 2,
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
            balances = await getStatsByAccount();
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
            movements = await getMovementsReport({
                from: movementsFrom,
                to: movementsTo,
            });
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
            monthly = await getMonthlyBalanceReport({
                year: selectedYear,
                month: selectedMonth,
            });
        } catch (e) {
            handleError(e);
        } finally {
            loading = false;
        }
    }

    function selectTab(t: Tab) {
        tab = t;
        error = null;
        if (t === "balances" && balances.length === 0) loadBalances();
        if (t === "movements" && movements.length === 0) loadMovements();
        if (t === "monthly" && monthly.length === 0) loadMonthly();
    }

    onMount(loadBalances);

    // Columnas de exportación (mismo orden/formato que las tablas)
    const balancesColumns: ExportColumn<AccountStats>[] = [
        { header: "Cuenta", accessor: (a) => `${a.account_name} (${a.account_number})` },
        { header: "Moneda", accessor: (a) => a.currency },
        { header: "Saldo actual", accessor: (a) => a.balance.toFixed(2) },
    ];

    const movementsColumns: ExportColumn<AccountStats>[] = [
        { header: "Cuenta", accessor: (a) => `${a.account_name} (${a.account_number})` },
        { header: "Moneda", accessor: (a) => a.currency },
        {
            header: "Entradas",
            accessor: (a) => (a.total_deposits + a.total_transfers_in).toFixed(2),
        },
        {
            header: "Salidas",
            accessor: (a) => (a.total_withdrawals + a.total_transfers_out).toFixed(2),
        },
        { header: "Variación neta", accessor: (a) => a.balance.toFixed(2) },
        { header: "Operaciones", accessor: (a) => a.transaction_count },
    ];

    const monthlyColumns: ExportColumn<MonthlyBalanceReport>[] = [
        { header: "Cuenta", accessor: (m) => `${m.account_name} (${m.account_number})` },
        { header: "Moneda", accessor: (m) => m.currency },
        { header: "Saldo inicial", accessor: (m) => m.opening_balance.toFixed(2) },
        { header: "Saldo final", accessor: (m) => m.closing_balance.toFixed(2) },
        { header: "Variación", accessor: (m) => m.net_change.toFixed(2) },
    ];
</script>

<main class="flex flex-col p-4 gap-4">
    <h1 class="text-2xl font-bold">Informes</h1>

    <div class="flex gap-2">
        {#each TABS as t (t.value)}
            <button
                type="button"
                onclick={() => selectTab(t.value)}
                class="border rounded-lg p-2 transition-colors {tab === t.value
                    ? 'bg-white/20 border-white/30'
                    : 'bg-white/5 border-zinc-800 hover:bg-white/10'}"
            >
                {t.label}
            </button>
        {/each}
    </div>

    {#if error}
        <p class="text-red-400 text-sm">{error}</p>
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
        <div class="bg-zinc-900 border border-zinc-800 rounded-lg overflow-hidden">
            {#if loading}
                <p class="p-4 text-zinc-400">Cargando...</p>
            {:else if balances.length === 0}
                <p class="p-4 text-zinc-400">No hay cuentas</p>
            {:else}
                <table class="w-full text-sm">
                    <thead class="bg-zinc-800/50 text-zinc-400 text-left">
                        <tr>
                            <th class="p-2">Cuenta</th>
                            <th class="p-2">Moneda</th>
                            <th class="p-2 text-right">Saldo actual</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each balances as b (b.account_id)}
                            <tr class="border-t border-zinc-800">
                                <td class="p-2">
                                    <div class="flex items-center gap-2">
                                        <span
                                            class="w-2 h-2 rounded-full shrink-0"
                                            style:background-color={b.color ??
                                                "#71717a"}
                                        ></span>
                                        {b.account_name} ({b.account_number})
                                    </div>
                                </td>
                                <td class="p-2">{b.currency}</td>
                                <td class="p-2 text-right font-bold"
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
        <div class="flex items-end justify-between gap-3 flex-wrap">
            <div class="flex items-end gap-3">
                <div class="flex flex-col gap-1">
                    <span class="text-sm text-zinc-400">Desde</span>
                    <input
                        class="p-1 bg-zinc-800 rounded-md"
                        type="date"
                        bind:value={movementsFrom}
                    />
                </div>
                <div class="flex flex-col gap-1">
                    <span class="text-sm text-zinc-400">Hasta</span>
                    <input
                        class="p-1 bg-zinc-800 rounded-md"
                        type="date"
                        bind:value={movementsTo}
                    />
                </div>
                <button
                    type="button"
                    onclick={loadMovements}
                    class="border rounded-lg bg-white/10 hover:bg-white/20 border-zinc-800 px-4 py-1"
                >
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

        <div class="bg-zinc-900 border border-zinc-800 rounded-lg overflow-hidden">
            {#if loading}
                <p class="p-4 text-zinc-400">Cargando...</p>
            {:else if movements.length === 0}
                <p class="p-4 text-zinc-400">No hay datos en este rango</p>
            {:else}
                <table class="w-full text-sm">
                    <thead class="bg-zinc-800/50 text-zinc-400 text-left">
                        <tr>
                            <th class="p-2">Cuenta</th>
                            <th class="p-2">Moneda</th>
                            <th class="p-2 text-right">Entradas</th>
                            <th class="p-2 text-right">Salidas</th>
                            <th class="p-2 text-right">Variación neta</th>
                            <th class="p-2 text-right">Operaciones</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each movements as m (m.account_id)}
                            <tr class="border-t border-zinc-800">
                                <td class="p-2">
                                    {m.account_name} ({m.account_number})
                                </td>
                                <td class="p-2">{m.currency}</td>
                                <td class="p-2 text-right text-green-400">
                                    {fmt(m.total_deposits + m.total_transfers_in)}
                                </td>
                                <td class="p-2 text-right text-red-400">
                                    {fmt(
                                        m.total_withdrawals + m.total_transfers_out,
                                    )}
                                </td>
                                <td
                                    class="p-2 text-right font-bold {m.balance >= 0
                                        ? 'text-green-400'
                                        : 'text-red-400'}"
                                >
                                    {fmt(m.balance)}
                                </td>
                                <td class="p-2 text-right text-zinc-400"
                                    >{m.transaction_count}</td
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
        <div class="flex items-end justify-between gap-3 flex-wrap">
            <div class="flex items-end gap-3">
                <div class="flex flex-col gap-1">
                    <span class="text-sm text-zinc-400">Mes</span>
                    <select
                        class="p-1 bg-zinc-800 rounded-md"
                        bind:value={selectedMonth}
                    >
                        {#each MONTH_NAMES as name, i (i)}
                            <option value={i + 1}>{name}</option>
                        {/each}
                    </select>
                </div>
                <div class="flex flex-col gap-1">
                    <span class="text-sm text-zinc-400">Año</span>
                    <input
                        class="p-1 w-24 bg-zinc-800 rounded-md"
                        type="number"
                        bind:value={selectedYear}
                    />
                </div>
                <button
                    type="button"
                    onclick={loadMonthly}
                    class="border rounded-lg bg-white/10 hover:bg-white/20 border-zinc-800 px-4 py-1"
                >
                    Aplicar
                </button>
            </div>
            <ExportButtons
                title="Saldo mensual {MONTH_NAMES[selectedMonth - 1]} {selectedYear}"
                data={monthly}
                columns={monthlyColumns}
                filename="saldo-mensual-{selectedYear}-{String(selectedMonth).padStart(2, '0')}"
            />
        </div>

        <div class="bg-zinc-900 border border-zinc-800 rounded-lg overflow-hidden">
            {#if loading}
                <p class="p-4 text-zinc-400">Cargando...</p>
            {:else if monthly.length === 0}
                <p class="p-4 text-zinc-400">No hay cuentas</p>
            {:else}
                {#if !monthly[0].month_finished}
                    <p class="p-2 text-xs text-zinc-500 border-b border-zinc-800">
                        El mes aún no ha finalizado — el saldo final corresponde
                        al día de hoy ({monthly[0].period_end}).
                    </p>
                {/if}
                <table class="w-full text-sm">
                    <thead class="bg-zinc-800/50 text-zinc-400 text-left">
                        <tr>
                            <th class="p-2">Cuenta</th>
                            <th class="p-2">Moneda</th>
                            <th class="p-2 text-right">Saldo inicial</th>
                            <th class="p-2 text-right">Saldo final</th>
                            <th class="p-2 text-right">Variación</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each monthly as m (m.account_id)}
                            <tr class="border-t border-zinc-800">
                                <td class="p-2">
                                    <div class="flex items-center gap-2">
                                        <span
                                            class="w-2 h-2 rounded-full shrink-0"
                                            style:background-color={m.color ??
                                                "#71717a"}
                                        ></span>
                                        {m.account_name} ({m.account_number})
                                    </div>
                                </td>
                                <td class="p-2">{m.currency}</td>
                                <td class="p-2 text-right"
                                    >{fmt(m.opening_balance)}</td
                                >
                                <td class="p-2 text-right font-bold"
                                    >{fmt(m.closing_balance)}</td
                                >
                                <td
                                    class="p-2 text-right {m.net_change >= 0
                                        ? 'text-green-400'
                                        : 'text-red-400'}"
                                >
                                    {m.net_change >= 0 ? "+" : ""}{fmt(m.net_change)}
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
