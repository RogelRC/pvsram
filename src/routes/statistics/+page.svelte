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

<main class="flex flex-col p-4 gap-6">
    <h1 class="text-2xl font-bold">Estadísticas</h1>

    {#if error}
        <p class="text-red-400 text-sm">{error}</p>
    {/if}

    {#if loading}
        <p class="text-zinc-400">Cargando...</p>
    {:else}
        <div
            class="flex flex-wrap gap-3 rounded-lg border border-zinc-800 bg-zinc-900/70 p-3"
        >
            <input
                class="min-w-44 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
                placeholder="Tesorería"
                bind:value={filters.account}
                oninput={applyFilters}
            />
            <input
                class="min-w-28 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
                placeholder="Moneda"
                bind:value={filters.currency}
                oninput={applyFilters}
            />
        </div>

        <!-- Por moneda -->
        <section class="flex flex-col gap-3">
            <h2 class="text-lg font-semibold text-zinc-300">Por moneda</h2>
            {#if byCurrency.length === 0}
                <p class="text-zinc-400 text-sm">No hay datos</p>
            {:else}
                <div class="grid lg:grid-cols-3 grid-cols-1 gap-4">
                    {#each byCurrency as c (c.currency)}
                        <div
                            class="flex flex-col bg-[#171717] rounded-md p-4 border border-white/10 gap-2"
                        >
                            <div class="flex justify-between items-center">
                                <h3 class="text-xl font-bold">{c.currency}</h3>
                                <span class="text-sm text-zinc-400"
                                    >{c.account_count} tesorería{c.account_count ===
                                    1
                                        ? ""
                                        : "s"}</span
                                >
                            </div>
                            <div>
                                <span class="text-sm text-zinc-400"
                                    >Balance total</span
                                >
                                <div class="text-2xl font-bold">
                                    {fmt(c.total_balance)}
                                </div>
                            </div>
                            <div class="grid grid-cols-3 gap-2 text-sm mt-2">
                                <div>
                                    <span class="text-zinc-400">Ingresos</span>
                                    <div class="text-green-400 font-medium">
                                        {fmt(c.total_deposits)}
                                    </div>
                                </div>
                                <div>
                                    <span class="text-zinc-400"
                                        >Extracciones</span
                                    >
                                    <div class="text-red-400 font-medium">
                                        {fmt(c.total_withdrawals)}
                                    </div>
                                </div>
                                <div>
                                    <span class="text-zinc-400"
                                        >Transferido</span
                                    >
                                    <div class="text-blue-400 font-medium">
                                        {fmt(c.total_transferred)}
                                    </div>
                                </div>
                            </div>
                            <span class="text-xs text-zinc-500"
                                >{c.transaction_count} operaciones</span
                            >
                        </div>
                    {/each}
                </div>
            {/if}
        </section>

        <!-- Por tesorería -->
        <section class="flex flex-col gap-3">
            <h2 class="text-lg font-semibold text-zinc-300">Por tesorería</h2>
            {#if byAccount.length === 0}
                <p class="text-zinc-400 text-sm">No hay datos</p>
            {:else}
                <div
                    class="bg-zinc-900 border border-zinc-800 rounded-lg overflow-hidden"
                >
                    <table class="w-full text-sm">
                        <thead class="bg-zinc-800/50 text-zinc-400 text-left">
                            <tr>
                                <th class="p-2">Tesorería</th>
                                <th class="p-2">Moneda</th>
                                <th class="p-2 text-right">Ingresos</th>
                                <th class="p-2 text-right">Extracciones</th>
                                <th class="p-2 text-right">Transf. enviadas</th>
                                <th class="p-2 text-right">Transf. recibidas</th
                                >
                                <th class="p-2 text-right">Balance</th>
                                <th class="p-2 text-right">Operaciones</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each byAccount as a (a.account_id)}
                                <tr
                                    class="border-t border-zinc-800 hover:bg-zinc-800"
                                >
                                    <td class="p-2">
                                        <div class="flex items-center gap-2">
                                            <span
                                                class="w-2 h-2 rounded-full shrink-0"
                                                style:background-color={a.color ??
                                                    "#71717a"}
                                            ></span>
                                            {a.account_name} ({a.account_number})
                                        </div>
                                    </td>
                                    <td class="p-2">{a.currency}</td>
                                    <td class="p-2 text-right text-green-400"
                                        >{fmt(a.total_deposits)}</td
                                    >
                                    <td class="p-2 text-right text-red-400"
                                        >{fmt(a.total_withdrawals)}</td
                                    >
                                    <td class="p-2 text-right text-zinc-300"
                                        >{fmt(a.total_transfers_out)}</td
                                    >
                                    <td class="p-2 text-right text-zinc-300"
                                        >{fmt(a.total_transfers_in)}</td
                                    >
                                    <td class="p-2 text-right font-bold"
                                        >{fmt(a.balance)}</td
                                    >
                                    <td class="p-2 text-right text-zinc-400"
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
