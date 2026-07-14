<script lang="ts">
    import { onMount } from "svelte";
    import Card from "$lib/components/Account/AccountCard.svelte";
    import { listAccounts, type Account } from "$lib/services/accounts";
    import { getAccountBalance } from "$lib/services/transactions";
    import CreateAccount from "$lib/components/Account/CreateAccount.svelte";

    let accounts = $state<Account[]>([]);
    let filteredAccounts = $state<Account[]>([]);
    let balances = $state<Record<number, number>>({});
    let filters = $state({
        number: "",
        name: "",
        owner: "",
        currency: "",
        description: "",
        createdFrom: "",
        createdTo: "",
        updatedFrom: "",
        updatedTo: "",
    });

    function normalizeText(value: string | null | undefined): string {
        return (value ?? "")
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

    function applyFilters() {
        const normalized = {
            number: normalizeText(filters.number),
            name: normalizeText(filters.name),
            owner: normalizeText(filters.owner),
            currency: normalizeText(filters.currency),
            description: normalizeText(filters.description),
        };

        filteredAccounts = accounts.filter((account) => {
            if (
                normalized.number &&
                !normalizeText(account.number).includes(normalized.number)
            ) {
                return false;
            }
            if (
                normalized.name &&
                !normalizeText(account.name).includes(normalized.name)
            ) {
                return false;
            }
            if (
                normalized.owner &&
                !normalizeText(account.owner).includes(normalized.owner)
            ) {
                return false;
            }
            if (
                normalized.currency &&
                !normalizeText(account.currency).includes(normalized.currency)
            ) {
                return false;
            }
            if (
                normalized.description &&
                !normalizeText(account.description).includes(
                    normalized.description,
                )
            ) {
                return false;
            }
            if (
                !matchesDateRange(
                    account.created_at,
                    filters.createdFrom,
                    filters.createdTo,
                )
            ) {
                return false;
            }
            if (
                !matchesDateRange(
                    account.updated_at,
                    filters.updatedFrom,
                    filters.updatedTo,
                )
            ) {
                return false;
            }
            return true;
        });
    }

    async function loadAccounts() {
        accounts = await listAccounts();
        applyFilters();
        const balancePromises = accounts.map(async (account) => {
            balances[account.id] = await getAccountBalance(account.id);
        });
        await Promise.all(balancePromises);
    }

    onMount(loadAccounts);
</script>

<main class="flex flex-col p-4 gap-4">
    <div class="flex">
        <h1 class="text-2xl font-bold mr-auto">Tesorerías</h1>
        <CreateAccount onCreated={loadAccounts} />
    </div>

    <div
        class="flex flex-wrap gap-3 rounded-lg border border-zinc-800 bg-zinc-900/70 p-3"
    >
        <input
            class="min-w-36 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
            placeholder="Número"
            bind:value={filters.number}
            oninput={applyFilters}
        />
        <input
            class="min-w-36 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
            placeholder="Nombre"
            bind:value={filters.name}
            oninput={applyFilters}
        />
        <input
            class="min-w-36 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
            placeholder="Propietario"
            bind:value={filters.owner}
            oninput={applyFilters}
        />
        <input
            class="min-w-28 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
            placeholder="Moneda"
            bind:value={filters.currency}
            oninput={applyFilters}
        />
        <input
            class="min-w-40 rounded-md border border-zinc-800 bg-zinc-800 px-2 py-1 text-sm"
            placeholder="Descripción"
            bind:value={filters.description}
            oninput={applyFilters}
        />
    </div>

    <div class="grid lg:grid-cols-4 grid-cols-2 gap-4">
        {#each filteredAccounts as account (account.id)}
            <Card
                {account}
                balance={balances[account.id] ?? 0}
                onUpdated={loadAccounts}
            />
        {/each}
    </div>
    {#if filteredAccounts.length === 0}
        <p>No hay tesorerías que coincidan con los filtros</p>
    {/if}
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
