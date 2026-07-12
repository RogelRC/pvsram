<script lang="ts">
    import { onMount } from "svelte";
    import Card from "$lib/components/Account/AccountCard.svelte";
    import { listAccounts, type Account } from "$lib/services/accounts";
    import { getAccountBalance } from "$lib/services/transactions";
    import CreateAccount from "$lib/components/Account/CreateAccount.svelte";

    let accounts = $state<Account[]>([]);
    let balances = $state<Record<number, number>>({});

    async function loadAccounts() {
        accounts = await listAccounts();
        const balancePromises = accounts.map(async (account) => {
            balances[account.id] = await getAccountBalance(account.id);
        });
        await Promise.all(balancePromises);
    }

    onMount(loadAccounts);
</script>

<main class="flex flex-col p-4 gap-4">
    <div class="flex">
        <h1 class="text-2xl font-bold mr-auto">Mis Cuentas</h1>
        <CreateAccount onCreated={loadAccounts} />
    </div>
    <div class="grid lg:grid-cols-4 grid-cols-2 gap-4">
        {#each accounts as account (account.id)}
            <Card
                {account}
                balance={balances[account.id] ?? 0}
                onUpdated={loadAccounts}
            />
        {/each}
    </div>
    {#if accounts.length === 0}
        <p>No tienes cuentas aún</p>
    {/if}
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>
