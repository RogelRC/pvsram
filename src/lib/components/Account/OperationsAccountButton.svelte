<script lang="ts">
    import { ArrowLeftRight, X, Save } from "lucide-svelte";
    import {
        createDeposit,
        createWithdrawal,
        createTransfer,
    } from "$lib/services/transactions";
    import { listAccounts, type Account } from "$lib/services/accounts";

    let { account, onCreated }: { account: Account; onCreated?: () => void } =
        $props();

    type OperationType = "deposit" | "withdrawal" | "transfer";

    let isOpen = $state(false);
    let saving = $state(false);
    let error = $state<string | null>(null);

    let operation = $state<OperationType>("deposit");
    let amount = $state<number | string>("");
    let description = $state("");

    let otherAccounts = $state<Account[]>([]);
    let relatedAccountId = $state<number | null>(null);

    const OPERATIONS: { value: OperationType; label: string }[] = [
        { value: "deposit", label: "Ingreso" },
        { value: "withdrawal", label: "Extracción" },
        { value: "transfer", label: "Transferencia" },
    ];

    async function menu() {
        isOpen = !isOpen;
        if (isOpen) {
            await loadOtherAccounts();
            resetForm();
        } else {
            error = null;
            saving = false;
        }
    }

    async function loadOtherAccounts() {
        try {
            const all = await listAccounts();
            // Solo cuentas distintas a la actual Y con la misma moneda
            otherAccounts = all.filter(
                (a) => a.id !== account.id && a.currency === account.currency,
            );
            relatedAccountId = otherAccounts[0]?.id ?? null;
        } catch (e) {
            console.error("Error cargando cuentas:", e);
            otherAccounts = [];
            relatedAccountId = null;
        }
    }

    function resetForm() {
        operation = "deposit";
        amount = "";
        description = "";
        error = null;
        saving = false;
    }

    function selectOperation(op: OperationType) {
        operation = op;
        error = null;
    }

    async function handleSave() {
        error = null;

        const parsedAmount = Number(amount);
        if (amount === "" || Number.isNaN(parsedAmount) || parsedAmount <= 0) {
            error = "Introduce un monto válido mayor que 0";
            return;
        }
        if (operation === "transfer" && !relatedAccountId) {
            error = "Selecciona una cuenta destino";
            return;
        }

        saving = true;
        try {
            const desc = description.trim() || null;

            if (operation === "deposit") {
                await createDeposit({
                    accountId: account.id,
                    amount: parsedAmount,
                    description: desc,
                });
            } else if (operation === "withdrawal") {
                await createWithdrawal({
                    accountId: account.id,
                    amount: parsedAmount,
                    description: desc,
                });
            } else {
                await createTransfer({
                    accountId: account.id,
                    relatedAccountId: relatedAccountId!,
                    amount: parsedAmount,
                    description: desc,
                });
            }

            onCreated?.();
            isOpen = false;
        } catch (e) {
            console.error("Error al guardar la operación:", e);
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
    <ArrowLeftRight />
</button>
{#if isOpen}
    <div
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-xs"
    >
        <div
            class="flex flex-col bg-zinc-900 border border-zinc-700 rounded-2xl shadow-2xl p-4 min-w-120 gap-4"
        >
            <div class="flex justify-between border-b items-center pb-2">
                <h3 class="text-2xl font-bold">Nueva operación</h3>
                <button
                    type="button"
                    onclick={menu}
                    class="flex items-center justify-items-center border rounded-lg bg-white/10 hover:bg-white/20 p-2"
                >
                    <X />
                </button>
            </div>

            <div class="flex gap-2">
                {#each OPERATIONS as op (op.value)}
                    <button
                        type="button"
                        onclick={() => selectOperation(op.value)}
                        disabled={saving}
                        class="flex-1 border rounded-lg p-2 transition-colors {operation ===
                        op.value
                            ? 'bg-white/20 border-white/30'
                            : 'bg-white/5 border-zinc-800 hover:bg-white/10'}"
                    >
                        {op.label}
                    </button>
                {/each}
            </div>

            {#if error}
                <p class="text-red-400 text-sm">{error}</p>
            {/if}

            <div class="flex items-center gap-2">
                <span>Cuenta:</span>
                <input
                    class="p-1 w-full bg-zinc-800/50 rounded-md text-zinc-400 cursor-not-allowed"
                    type="text"
                    value="{account.name} ({account.number})"
                    disabled
                />
            </div>

            {#if operation === "transfer"}
                <div class="flex items-center gap-2">
                    <span>Destino:</span>
                    {#if otherAccounts.length === 0}
                        <span class="text-sm text-zinc-400">
                            No hay otras cuentas en {account.currency} disponibles
                            para transferir
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
                    placeholder="0.00"
                    bind:value={amount}
                    disabled={saving}
                />
                <span class="text-sm text-zinc-400 shrink-0"
                    >{account.currency}</span
                >
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
                        (operation === "transfer" && otherAccounts.length === 0)}
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
