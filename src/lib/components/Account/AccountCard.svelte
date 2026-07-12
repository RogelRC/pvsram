<script lang="ts">
    import type { Account } from "$lib/services/accounts";
    import { ArrowLeftRight } from "@lucide/svelte";
    import EditAccountButton from "$lib/components/Account/EditAccountButton.svelte";
    import TransactionButton from "$lib/components/Account/OperationsAccountButton.svelte";
    interface Props {
        account: Account;
        balance?: number;
        onUpdated?: () => void;
    }
    let { account, balance = 0, onUpdated }: Props = $props();
</script>
<div
    class="flex flex-col bg-[#171717] rounded-md w-full p-4 border border-white/10"
>
    <div
        class="w-16 h-3 mb-3 rounded-full"
        style:background-color={account.color}
    ></div>
    <h3 class="font-bold text-2xl">{account.name}</h3>
    <span class="mb-3">{account.description}</span>
    <span>Número de cuenta</span>
    <span class="mb-3">{account.number}</span>
    <span>Propietario</span>
    <span class="mb-3">{account.owner}</span>
    <div class="flex justify-between">
        <div>
            <span>Saldo actual</span>
            <div>
                <span class="text-2xl font-bold">{balance}</span>
                <span>{account.currency}</span>
            </div>
        </div>
        <div class="flex items-end gap-2">
            <EditAccountButton {account} {onUpdated} />
            <TransactionButton {account} onCreated={onUpdated} />
        </div>
    </div>
</div>
<style lang="postcss">
    @reference "tailwindcss";
</style>
