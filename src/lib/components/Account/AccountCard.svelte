<script lang="ts">
    import type { Account } from "$lib/services/accounts";
    import { MoreHorizontal } from "@lucide/svelte";
    import EditAccountButton from "$lib/components/Account/EditAccountButton.svelte";
    import TransactionButton from "$lib/components/Account/OperationsAccountButton.svelte";
    import { deleteAccount } from "$lib/services/accounts";
    import { formatMoney } from "$lib/utils/money";

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

<div class="card group relative flex flex-col overflow-hidden">
    <div
        class="absolute inset-x-0 top-0 h-1 opacity-90"
        style:background-color={account.color}
    ></div>

    <div class="mb-3 flex items-start justify-between gap-2 pt-1">
        <div class="min-w-0">
            <h3 class="truncate text-base font-semibold text-zinc-50">
                {account.name}
            </h3>
            {#if account.destination}
                <p class="mt-0.5 truncate text-sm text-zinc-400">
                    {account.destination}
                </p>
            {/if}
        </div>
        <span
            class="shrink-0 rounded-md border border-zinc-800 bg-zinc-950/50 px-2 py-0.5 text-xs font-medium text-zinc-400"
        >
            {account.currency}
        </span>
    </div>

    <div class="mb-4 space-y-2.5">
        <div>
            <div class="label-muted">Número de cuenta</div>
            <div class="mt-0.5 font-mono text-sm text-zinc-200">
                {account.number}
            </div>
        </div>
        {#if account.comment}
            <div>
                <div class="label-muted">Comentario</div>
                <div class="mt-0.5 text-sm text-zinc-300">{account.comment}</div>
            </div>
        {/if}
    </div>

    <div class="mt-auto flex items-end justify-between gap-3 border-t border-zinc-800/80 pt-3">
        <div>
            <div class="label-muted">Saldo actual</div>
            <div class="mt-0.5 flex items-baseline gap-1.5">
                <span class="text-2xl font-semibold tracking-tight text-zinc-50">
                    {formatMoney(balance)}
                </span>
                <span class="text-sm text-zinc-500">{account.currency}</span>
            </div>
        </div>
        <div class="flex items-end gap-1.5 relative">
            <TransactionButton {account} onCreated={onUpdated} />

            <div class="relative">
                <button
                    class="btn-icon"
                    aria-label="Más opciones"
                    onclick={() => (showMenu = !showMenu)}
                >
                    <MoreHorizontal size={18} />
                </button>

                {#if showMenu}
                    <button
                        type="button"
                        class="fixed inset-0 z-30 cursor-default"
                        aria-label="Cerrar menú"
                        onclick={() => (showMenu = false)}
                    ></button>
                    <div
                        class="absolute right-0 z-40 mt-2 w-48 rounded-xl border border-zinc-800 bg-zinc-900/95 p-1.5 shadow-xl shadow-black/40 backdrop-blur-md"
                    >
                        <EditAccountButton {account} {onUpdated} />
                        <button
                            class="w-full rounded-lg px-3 py-2 text-left text-sm text-red-300 transition-colors hover:bg-red-500/10"
                            onclick={() => openDeleteConfirm()}
                        >
                            Eliminar cuenta
                        </button>
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>

{#if showDeleteConfirm}
    <div class="modal-backdrop">
        <div class="modal-panel">
            <h3 class="text-lg font-semibold text-zinc-50">
                Confirmar eliminación
            </h3>
            <p class="text-sm leading-relaxed text-zinc-400">
                Esta acción eliminará la cuenta y toda la información
                relacionada (transacciones, informes, etc.). No se puede
                deshacer.
            </p>
            <p class="text-sm text-zinc-300">
                Para confirmar, escribe el nombre de la cuenta:
                <span class="font-semibold text-zinc-100">{account.name}</span>
            </p>
            <input
                class="field w-full"
                placeholder="Nombre de la cuenta"
                bind:value={confirmText}
            />
            <div class="flex justify-end gap-2">
                <button
                    class="btn"
                    onclick={() => cancelDelete()}
                    disabled={deleting}
                >
                    Cancelar
                </button>
                <button
                    class="btn-danger"
                    onclick={() => confirmDelete()}
                    disabled={confirmText !== account.name || deleting}
                >
                    {#if deleting}
                        Eliminando…
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
        class="fixed bottom-4 right-4 z-50 rounded-xl border border-zinc-700 bg-zinc-900 px-4 py-2.5 text-sm text-zinc-100 shadow-xl shadow-black/40"
    >
        {toast}
    </div>
{/if}

<style lang="postcss">
    @reference "tailwindcss";
</style>
