<script lang="ts">
    import { onMount } from "svelte";
    import {
        getStatsByAccount,
        getStatsByCurrency,
        type AccountStats,
        type CurrencyStats,
    } from "$lib/services/statistics";

    let loading = $state(false);
    let error = $state<string | null>(null);

    let allByCurrency = $state<CurrencyStats[]>([]);
    let allByAccount = $state<AccountStats[]>([]);
    let byCurrency = $state<CurrencyStats[]>([]);
    let byAccount = $state<AccountStats[]>([]);
    let filters = $state({
        account: "",
        currency: "",
    });

    function normalizeText(value: string | null | undefined): string {
        return (value ?? "")
            .normalize("NFD")
            .replace(/[\u0300-\u036f]/g, "")
            .toLowerCase();
    }

    function buildCurrencyStats(accounts: AccountStats[]): CurrencyStats[] {
        const groups = new Map<string, CurrencyStats>();

        for (const account of accounts) {
            const existing = groups.get(account.currency) ?? {
                currency: account.currency,
                account_count: 0,
                total_balance: 0,
                total_deposits: 0,
                total_withdrawals: 0,
                total_transferred: 0,
                transaction_count: 0,
            };

            existing.account_count += 1;
            existing.total_balance += account.balance;
            existing.total_deposits += account.total_deposits;
            existing.total_withdrawals += account.total_withdrawals;
            existing.total_transferred +=
                account.total_transfers_in + account.total_transfers_out;
            existing.transaction_count += account.transaction_count;
            groups.set(account.currency, existing);
        }

        return Array.from(groups.values()).sort((a, b) =>
            a.currency.localeCompare(b.currency),
        );
    }

    function applyFilters() {
        const normalizedAccount = normalizeText(filters.account);
        const normalizedCurrency = normalizeText(filters.currency);

        const filteredAccounts = allByAccount.filter((account) => {
            const accountText = `${account.account_name} ${account.account_number}`;
            const matchesAccount =
                !normalizedAccount ||
                normalizeText(accountText).includes(normalizedAccount);
            const matchesCurrency =
                !normalizedCurrency ||
                normalizeText(account.currency).includes(normalizedCurrency);
            return matchesAccount && matchesCurrency;
        });

        byAccount = filteredAccounts;
        byCurrency = buildCurrencyStats(filteredAccounts);
    }

    async function loadData() {
        loading = true;
        error = null;
        try {
            const [currencyStats, accountStats] = await Promise.all([
                getStatsByCurrency(),
                getStatsByAccount(),
            ]);
            allByCurrency = currencyStats;
            allByAccount = accountStats;
            applyFilters();
        } catch (e) {
            console.error("Error cargando estadísticas:", e);
            if (e instanceof Error) {
                error = e.message;
            } else if (typeof e === "string") {
                error = e;
            } else {
                error = JSON.stringify(e);
            }
        } finally {
            loading = false;
        }
    }

    function fmt(amount: number): string {
        return amount.toLocaleString(undefined, {
            minimumFractionDigits: 2,
            maximumFractionDigits: 2,
        });
    }

    onMount(loadData);
</script>

<main class="page">
    <div class="page-header">
        <div>
            <h1 class="page-title">Estadísticas</h1>
            <p class="page-subtitle">
                Resumen de saldos y flujos por moneda y tesorería
            </p>
        </div>
    </div>

    {#if error}
        <p
            class="rounded-lg border border-red-500/20 bg-red-500/10 px-3 py-2 text-sm text-red-300"
        >
            {error}
        </p>
    {/if}

    {#if loading}
        <p class="text-sm text-zinc-400">Cargando estadísticas…</p>
    {:else}
        <div class="filter-bar">
            <input
                class="field min-w-44"
                placeholder="Tesorería"
                bind:value={filters.account}
                oninput={applyFilters}
            />
            <input
                class="field min-w-28"
                placeholder="Moneda"
                bind:value={filters.currency}
                oninput={applyFilters}
            />
        </div>

        <section class="flex flex-col gap-3">
            <h2 class="text-sm font-semibold tracking-wide text-zinc-300 uppercase">
                Por moneda
            </h2>
            {#if byCurrency.length === 0}
                <div class="empty-state">No hay datos</div>
            {:else}
                <div class="grid grid-cols-1 gap-4 lg:grid-cols-3">
                    {#each byCurrency as c (c.currency)}
                        <div class="card flex flex-col gap-3">
                            <div class="flex items-center justify-between">
                                <h3 class="text-xl font-semibold text-zinc-50">
                                    {c.currency}
                                </h3>
                                <span class="badge bg-zinc-800 text-zinc-400">
                                    {c.account_count} tesorería{c.account_count ===
                                    1
                                        ? ""
                                        : "s"}
                                </span>
                            </div>
                            <div>
                                <div class="label-muted">Balance total</div>
                                <div class="mt-0.5 text-2xl font-semibold tracking-tight">
                                    {fmt(c.total_balance)}
                                </div>
                            </div>
                            <div class="grid grid-cols-3 gap-2 border-t border-zinc-800/80 pt-3 text-sm">
                                <div>
                                    <div class="text-zinc-500">Ingresos</div>
                                    <div class="font-medium text-emerald-400">
                                        {fmt(c.total_deposits)}
                                    </div>
                                </div>
                                <div>
                                    <div class="text-zinc-500">Extracciones</div>
                                    <div class="font-medium text-red-400">
                                        {fmt(c.total_withdrawals)}
                                    </div>
                                </div>
                                <div>
                                    <div class="text-zinc-500">Transferido</div>
                                    <div class="font-medium text-sky-400">
                                        {fmt(c.total_transferred)}
                                    </div>
                                </div>
                            </div>
                            <span class="text-xs text-zinc-500">
                                {c.transaction_count} operaciones
                            </span>
                        </div>
                    {/each}
                </div>
            {/if}
        </section>

        <section class="flex flex-col gap-3">
            <h2 class="text-sm font-semibold tracking-wide text-zinc-300 uppercase">
                Por tesorería
            </h2>
            {#if byAccount.length === 0}
                <div class="empty-state">No hay datos</div>
            {:else}
                <div class="table-wrap overflow-x-auto">
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th>Tesorería</th>
                                <th>Moneda</th>
                                <th class="text-right">Ingresos</th>
                                <th class="text-right">Extracciones</th>
                                <th class="text-right">Transf. enviadas</th>
                                <th class="text-right">Transf. recibidas</th>
                                <th class="text-right">Balance</th>
                                <th class="text-right">Operaciones</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each byAccount as a (a.account_id)}
                                <tr>
                                    <td>
                                        <div class="flex items-center gap-2">
                                            <span
                                                class="h-2 w-2 shrink-0 rounded-full"
                                                style:background-color={a.color ??
                                                    "#71717a"}
                                            ></span>
                                            ({a.account_number}) {a.account_name}
                                        </div>
                                    </td>
                                    <td>{a.currency}</td>
                                    <td class="text-right text-emerald-400"
                                        >{fmt(a.total_deposits)}</td
                                    >
                                    <td class="text-right text-red-400"
                                        >{fmt(a.total_withdrawals)}</td
                                    >
                                    <td class="text-right text-zinc-300"
                                        >{fmt(a.total_transfers_out)}</td
                                    >
                                    <td class="text-right text-zinc-300"
                                        >{fmt(a.total_transfers_in)}</td
                                    >
                                    <td class="text-right font-semibold"
                                        >{fmt(a.balance)}</td
                                    >
                                    <td class="text-right text-zinc-400"
                                        >{a.transaction_count}</td
                                    >
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            {/if}
        </section>
    {/if}
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
