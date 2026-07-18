<script lang="ts">
    import type { Account } from "$lib/services/accounts";
    import { MoreHorizontal } from "@lucide/svelte";
    import EditAccountButton from "$lib/components/Account/EditAccountButton.svelte";
    import TransactionButton from "$lib/components/Account/OperationsAccountButton.svelte";
    import { deleteAccount } from "$lib/services/accounts";
    interface Props {
        account: Account;
        balance?: number;
        onUpdated?: () => void;
    }
    let { account, balance = 0, onUpdated }: Props = $props();
    let showMenu = $state(false);
    let showDeleteConfirm = $state(false);
    let confirmText = $state("");
    let deleting = $state(false);
    let toast = $state<string | null>(null);

    function showToast(message: string) {
        toast = message;
        setTimeout(() => (toast = null), 3500);
    }

    function openDeleteConfirm() {
        showMenu = false;
        showDeleteConfirm = true;
        confirmText = "";
    }

    function cancelDelete() {
        showDeleteConfirm = false;
        confirmText = "";
    }

    async function confirmDelete() {
        if (confirmText !== account.name) return;
        deleting = true;
        try {
            await deleteAccount(account.id);
            deleting = false;
            showDeleteConfirm = false;
            if (onUpdated) onUpdated();
            showToast("Cuenta eliminada correctamente");
        } catch (err) {
            deleting = false;
            console.error(err);
            const msg = (err as any)?.message ?? String(err);
            showToast("Error al eliminar la cuenta: " + msg);
        }
    }
</script>

<div
    class="flex flex-col bg-[#171717] rounded-md w-full p-4 border border-white/10"
>
    <div
        class="w-16 h-3 mb-3 rounded-full"
        style:background-color={account.color}
    ></div>
    <h3 class="font-bold text-l">{account.name}</h3>
    <span class="mb-3">{account.destination}</span>
    <span>Número de cuenta</span>
    <span class="mb-3">{account.number}</span>
    <span>Comentario</span>
    <span class="mb-3">{account.comment}</span>
    <div class="flex justify-between">
        <div>
            <span>Saldo actual</span>
            <div>
                <span class="text-2xl font-bold">{balance}</span>
                <span>{account.currency}</span>
            </div>
        </div>
        <div class="flex items-end gap-2 relative">
            <TransactionButton {account} onCreated={onUpdated} />

            <!-- Dropdown menu trigger -->
            <div class="relative">
                <button
                    class="p-2 rounded-md hover:bg-white/5"
                    aria-label="Más opciones"
                    onclick={() => (showMenu = !showMenu)}
                >
                    <MoreHorizontal />
                </button>

                {#if showMenu}
                    <div
                        class="absolute right-0 mt-2 w-44 bg-[#0f0f0f] border border-white/10 rounded-md shadow-lg z-40"
                    >
                        <div class="p-2">
                            <div class="mb-2">
                                <!-- Reuse edit button inside menu -->
                                <EditAccountButton {account} {onUpdated} />
                            </div>
                            <button
                                class="w-full text-left px-3 py-2 rounded hover:bg-red-600/10 text-red-400"
                                onclick={() => openDeleteConfirm()}
                            >
                                Eliminar cuenta
                            </button>
                        </div>
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>

{#if showDeleteConfirm}
    <div
        class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
    >
        <div
            class="bg-[#171717] p-6 rounded-md w-full max-w-lg border border-white/10"
        >
            <h3 class="text-xl font-bold mb-2">
                Confirmar eliminación de cuenta
            </h3>
            <p class="mb-4 text-sm text-zinc-300">
                Esta acción eliminará la cuenta y toda la información
                relacionada (transacciones, informes, etc.). Esta operación no
                se puede deshacer.
            </p>
            <p class="mb-4 text-sm">
                Para confirmar, escribe el nombre de la cuenta: <span
                    class="font-semibold">{account.name}</span
                >
            </p>
            <input
                class="w-full mb-4 p-2 rounded bg-[#0b0b0b] border border-white/10"
                placeholder="Escribe el nombre de la cuenta para confirmar"
                bind:value={confirmText}
            />
            <div class="flex justify-end gap-2">
                <button
                    class="px-4 py-2 rounded hover:bg-white/5"
                    onclick={() => cancelDelete()}
                    disabled={deleting}
                >
                    Cancelar
                </button>
                <button
                    class="px-4 py-2 rounded bg-red-600 text-white disabled:opacity-50"
                    onclick={() => confirmDelete()}
                    disabled={confirmText !== account.name || deleting}
                >
                    {#if deleting}
                        Eliminando...
                    {:else}
                        Eliminar definitivamente
                    {/if}
                </button>
            </div>
        </div>
    </div>
{/if}

{#if toast}
    <div
        class="fixed bottom-4 right-4 bg-zinc-900 text-white px-4 py-2 rounded shadow z-50"
    >
        {toast}
    </div>
{/if}

<style lang="postcss">
    @reference "tailwindcss";
</style>
