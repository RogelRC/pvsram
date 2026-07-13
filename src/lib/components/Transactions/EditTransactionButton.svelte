<script lang="ts">
    import Pencil from "@lucide/svelte/icons/pencil";
    import { X, Save } from "lucide-svelte";
    import {
        updateTransaction,
        type TransactionRecord,
    } from "$lib/services/transactions";
    import { listAccounts, type Account } from "$lib/services/accounts";

    let {
        transaction,
        onUpdated,
    }: { transaction: TransactionRecord; onUpdated?: () => void } = $props();

    const TYPE_LABELS: Record<string, string> = {
        deposit: "Ingreso",
        withdrawal: "Extracción",
        transfer: "Transferencia",
    };

    let isOpen = $state(false);
    let saving = $state(false);
    let error = $state<string | null>(null);

    let amount = $state<number | string>("");
    let description = $state("");
    let occurredAt = $state("");

    let otherAccounts = $state<Account[]>([]);
    let relatedAccountId = $state<number | null>(null);

    function toDatetimeLocal(value: string): string {
        // "2026-07-13 10:30:00" -> "2026-07-13T10:30"
        return value.replace(" ", "T").slice(0, 16);
    }

    function toOccurredAt(value: string): string {
        // "2026-07-13T10:30" -> "2026-07-13 10:30:00"
        return `${value.replace("T", " ")}:00`;
    }

    async function menu() {
        isOpen = !isOpen;
        if (isOpen) {
            await loadFromTransaction();
        } else {
            error = null;
            saving = false;
        }
    }

    async function loadFromTransaction() {
        amount = transaction.amount;
        description = transaction.description ?? "";
        occurredAt = toDatetimeLocal(transaction.occurred_at);
        relatedAccountId = transaction.related_account_id;
        error = null;
        saving = false;

        if (transaction.type === "transfer") {
            try {
                const all = await listAccounts();
                otherAccounts = all.filter(
                    (a) =>
                        a.id !== transaction.account_id &&
                        a.currency === transaction.currency,
                );
            } catch (e) {
                console.error("Error cargando cuentas:", e);
                otherAccounts = [];
            }
        }
    }

    async function handleSave() {
        error = null;

        const parsedAmount = Number(amount);
        if (amount === "" || Number.isNaN(parsedAmount) || parsedAmount <= 0) {
            error = "Introduce un monto válido mayor que 0";
            return;
        }
        if (!occurredAt) {
            error = "Selecciona una fecha y hora";
            return;
        }
        if (transaction.type === "transfer" && !relatedAccountId) {
            error = "Selecciona una cuenta destino";
            return;
        }

        saving = true;
        try {
            await updateTransaction({
                id: transaction.id,
                relatedAccountId:
                    transaction.type === "transfer" ? relatedAccountId : null,
                amount: parsedAmount,
                description: description.trim() || null,
                occurredAt: toOccurredAt(occurredAt),
            });
            onUpdated?.();
            isOpen = false;
        } catch (e) {
            console.error("Error al actualizar la operación:", e);
            error = e instanceof Error ? e.message : String(e);
        } finally {
            saving = false;
        }
    }
</script>

<button
    type="button"
    onclick={menu}
    class="border rounded-md p-1 hover:bg-zinc-700 border-zinc-800"
>
    <Pencil size={16} />
</button>
{#if isOpen}
    <div
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-xs"
    >
        <div
            class="flex flex-col bg-zinc-900 border border-zinc-700 rounded-2xl shadow-2xl p-4 min-w-120 gap-4"
        >
            <div class="flex justify-between border-b items-center pb-2">
                <h3 class="text-2xl font-bold">Editar operación</h3>
                <button
                    type="button"
                    onclick={menu}
                    class="flex items-center justify-items-center border rounded-lg bg-white/10 hover:bg-white/20 p-2"
                >
                    <X />
                </button>
            </div>

            {#if error}
                <p class="text-red-400 text-sm">{error}</p>
            {/if}

            <div class="flex items-center gap-2">
                <span>Tipo:</span>
                <input
                    class="p-1 w-full bg-zinc-800/50 rounded-md text-zinc-400 cursor-not-allowed"
                    type="text"
                    value={TYPE_LABELS[transaction.type] ?? transaction.type}
                    disabled
                />
            </div>

            <div class="flex items-center gap-2">
                <span>Cuenta:</span>
                <input
                    class="p-1 w-full bg-zinc-800/50 rounded-md text-zinc-400 cursor-not-allowed"
                    type="text"
                    value="{transaction.account_name} ({transaction.account_number})"
                    disabled
                />
            </div>

            {#if transaction.type === "transfer"}
                <div class="flex items-center gap-2">
                    <span>Destino:</span>
                    {#if otherAccounts.length === 0}
                        <span class="text-sm text-zinc-400">
                            No hay otras cuentas en {transaction.currency} disponibles
                        </span>
                    {:else}
                        <select
                            class="p-1 w-full bg-zinc-800 rounded-md"
                            bind:value={relatedAccountId}
                            disabled={saving}
                        >
                            {#each otherAccounts as acc (acc.id)}
                                <option value={acc.id}>
                                    {acc.name} ({acc.number}) · {acc.currency}
                                </option>
                            {/each}
                        </select>
                    {/if}
                </div>
            {/if}

            <div class="flex items-center gap-2">
                <span>Monto:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="number"
                    min="0"
                    step="0.01"
                    bind:value={amount}
                    disabled={saving}
                />
                <span class="text-sm text-zinc-400 shrink-0"
                    >{transaction.currency}</span
                >
            </div>

            <div class="flex items-center gap-2">
                <span>Fecha:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="datetime-local"
                    bind:value={occurredAt}
                    disabled={saving}
                />
            </div>

            <div class="flex items-center gap-2">
                <span>Descripción:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="text"
                    bind:value={description}
                    disabled={saving}
                />
            </div>

            <div class="flex justify-end">
                <button
                    type="button"
                    onclick={handleSave}
                    disabled={saving ||
                        (transaction.type === "transfer" &&
                            otherAccounts.length === 0)}
                    class="flex items-center justify-items-center border rounded-lg bg-white/10 hover:bg-white/20 p-2 disabled:opacity-50"
                >
                    <span class="mr-2"
                        >{saving ? "Guardando..." : "Guardar"}</span
                    >
                    <Save />
                </button>
            </div>
        </div>
    </div>
{/if}
