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
        description = transaction.comment ?? "";
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
                comment: description.trim() || null,
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
    class="btn-icon"
    title="Editar"
    aria-label="Editar operación"
>
    <Pencil size={15} />
</button>
{#if isOpen}
    <div class="modal-backdrop">
        <div class="modal-panel max-w-xl">
            <div
                class="flex items-center justify-between border-b border-zinc-800 pb-3"
            >
                <h3 class="text-lg font-semibold text-zinc-50">
                    Editar operación
                </h3>
                <button
                    type="button"
                    onclick={menu}
                    class="btn-icon"
                    aria-label="Cerrar"
                >
                    <X size={16} />
                </button>
            </div>

            {#if error}
                <p
                    class="rounded-lg border border-red-500/20 bg-red-500/10 px-3 py-2 text-sm text-red-300"
                >
                    {error}
                </p>
            {/if}

            <div class="grid gap-3">
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Tipo</span>
                    <input
                        class="field w-full cursor-not-allowed opacity-60"
                        type="text"
                        value={TYPE_LABELS[transaction.type] ?? transaction.type}
                        disabled
                    />
                </label>

                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Tesorería</span>
                    <input
                        class="field w-full cursor-not-allowed opacity-60"
                        type="text"
                        value="{transaction.account_name} ({transaction.account_number})"
                        disabled
                    />
                </label>

                {#if transaction.type === "transfer"}
                    <div class="flex flex-col gap-1.5">
                        <span class="label-muted">Destino</span>
                        {#if otherAccounts.length === 0}
                            <span class="text-sm text-zinc-400">
                                No hay otras cuentas en {transaction.currency} disponibles
                            </span>
                        {:else}
                            <select
                                class="field w-full"
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

                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Monto</span>
                    <div class="flex items-center gap-2">
                        <input
                            class="field w-full"
                            type="number"
                            min="0"
                            step="0.01"
                            bind:value={amount}
                            disabled={saving}
                        />
                        <span
                            class="shrink-0 rounded-md border border-zinc-800 bg-zinc-950/50 px-2 py-2 text-sm text-zinc-400"
                        >
                            {transaction.currency}
                        </span>
                    </div>
                </label>

                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Fecha</span>
                    <input
                        class="field w-full"
                        type="datetime-local"
                        bind:value={occurredAt}
                        disabled={saving}
                    />
                </label>

                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Comentario</span>
                    <input
                        class="field w-full"
                        type="text"
                        bind:value={description}
                        disabled={saving}
                    />
                </label>
            </div>

            <div class="flex justify-end gap-2 border-t border-zinc-800 pt-3">
                <button type="button" onclick={menu} class="btn" disabled={saving}>
                    Cancelar
                </button>
                <button
                    type="button"
                    onclick={handleSave}
                    disabled={saving ||
                        (transaction.type === "transfer" &&
                            otherAccounts.length === 0)}
                    class="btn-primary"
                >
                    <Save size={16} />
                    <span>{saving ? "Guardando…" : "Guardar"}</span>
                </button>
            </div>
        </div>
    </div>
{/if}
