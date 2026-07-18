<script lang="ts">
    import { ArrowLeftRight, X, Save, ArrowLeft } from "lucide-svelte";
    import {
        createDeposit,
        createWithdrawal,
        createTransfer,
    } from "$lib/services/transactions";
    import { listAccounts, type Account } from "$lib/services/accounts";
    import {
        ensureConcept,
        listConcepts,
        type Concept,
        type ConceptType,
    } from "$lib/services/concepts";

    let { account, onCreated }: { account: Account; onCreated?: () => void } =
        $props();

    type OperationType = ConceptType;

    const CUSTOM_OPTION = "__custom__";

    let isOpen = $state(false);
    let saving = $state(false);
    let error = $state<string | null>(null);

    let operation = $state<OperationType>("deposit");
    let amount = $state<number | string>("");
    let comment = $state("");

    let otherAccounts = $state<Account[]>([]);
    let relatedAccountId = $state<number | null>(null);

    let availableConcepts = $state<Concept[]>([]);
    let selectedConceptId = $state<number | null>(null);
    let isCustomConcept = $state(false);
    let customConcept = $state("");

    const OPERATIONS: { value: OperationType; label: string }[] = [
        { value: "deposit", label: "Ingreso" },
        { value: "withdrawal", label: "Extracción" },
        { value: "transfer", label: "Transferencia" },
    ];

    async function menu() {
        isOpen = !isOpen;
        if (isOpen) {
            resetForm();
            await Promise.all([loadOtherAccounts(), loadConcepts("deposit")]);
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

    async function loadConcepts(op: OperationType) {
        try {
            availableConcepts = await listConcepts({
                accountId: account.id,
                conceptType: op,
                activeOnly: true,
            });
            selectedConceptId = availableConcepts[0]?.id ?? null;
            // Si no hay conceptos, entrar en modo "agregar otro"
            isCustomConcept = availableConcepts.length === 0;
            customConcept = "";
        } catch (e) {
            console.error("Error cargando conceptos:", e);
            availableConcepts = [];
            selectedConceptId = null;
            isCustomConcept = true;
            customConcept = "";
        }
    }

    function resetForm() {
        operation = "deposit";
        amount = "";
        comment = "";
        error = null;
        saving = false;
        availableConcepts = [];
        selectedConceptId = null;
        isCustomConcept = false;
        customConcept = "";
    }

    async function selectOperation(op: OperationType) {
        operation = op;
        error = null;
        await loadConcepts(op);
    }

    function onConceptSelect(e: Event) {
        const value = (e.target as HTMLSelectElement).value;
        if (value === CUSTOM_OPTION) {
            isCustomConcept = true;
            customConcept = "";
        } else {
            selectedConceptId = Number(value);
        }
    }

    function backToConceptSelect() {
        isCustomConcept = false;
        customConcept = "";
        if (availableConcepts.length > 0 && selectedConceptId == null) {
            selectedConceptId = availableConcepts[0].id;
        }
    }

    async function resolveConceptId(): Promise<number | null> {
        if (isCustomConcept) {
            const name = customConcept.trim();
            if (!name) return null;
            const concept = await ensureConcept({
                accountId: account.id,
                conceptType: operation,
                name,
            });
            return concept.id;
        }
        return selectedConceptId;
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
        if (isCustomConcept && !customConcept.trim()) {
            error = "Debes indicar un concepto";
            return;
        }
        if (!isCustomConcept && selectedConceptId == null) {
            error = "Selecciona un concepto";
            return;
        }

        saving = true;
        try {
            const desc = comment.trim() || null;
            const conceptId = await resolveConceptId();
            if (conceptId == null) {
                error = "Debes indicar un concepto";
                return;
            }

            if (operation === "deposit") {
                await createDeposit({
                    accountId: account.id,
                    amount: parsedAmount,
                    comment: desc,
                    conceptId,
                });
            } else if (operation === "withdrawal") {
                await createWithdrawal({
                    accountId: account.id,
                    amount: parsedAmount,
                    comment: desc,
                    conceptId,
                });
            } else {
                await createTransfer({
                    accountId: account.id,
                    relatedAccountId: relatedAccountId!,
                    amount: parsedAmount,
                    comment: desc,
                    conceptId,
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

<button type="button" onclick={menu} class="p-2 rounded-md hover:bg-white/5">
    <ArrowLeftRight />
</button>
{#if isOpen}
    <div
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-xs"
    >
        <div
            class="flex flex-col bg-zinc-900 border border-zinc-700 rounded-2xl shadow-2xl p-4 min-w-120 gap-4"
        >
            <div
                class="flex justify-between border-b border-zinc-700 items-center pb-2"
            >
                <h3 class="text-2xl font-bold">Nueva operación</h3>
                <button
                    type="button"
                    onclick={menu}
                    class="border rounded-md p-1 hover:bg-zinc-700 border-zinc-800 flex items-center gap-2 px-3"
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
                <span>Concepto:</span>
                {#if isCustomConcept}
                    {#if availableConcepts.length > 0}
                        <button
                            type="button"
                            onclick={backToConceptSelect}
                            disabled={saving}
                            class="flex items-center justify-center border rounded-md bg-white/10 hover:bg-white/20 p-1 shrink-0"
                            title="Volver a la lista"
                        >
                            <ArrowLeft size={16} />
                        </button>
                    {/if}
                    <input
                        class="p-1 w-full bg-zinc-800 rounded-md"
                        type="text"
                        placeholder="Nombre del concepto"
                        bind:value={customConcept}
                        disabled={saving}
                    />
                {:else}
                    <select
                        class="p-1 w-full bg-zinc-800 rounded-md"
                        value={selectedConceptId ?? ""}
                        onchange={onConceptSelect}
                        disabled={saving}
                    >
                        {#each availableConcepts as concept (concept.id)}
                            <option value={concept.id}>{concept.name}</option>
                        {/each}
                        <option value={CUSTOM_OPTION}>Agregar otro...</option>
                    </select>
                {/if}
            </div>

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
                <span>Comentario:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="text"
                    bind:value={comment}
                    disabled={saving}
                />
            </div>

            <div class="flex justify-end">
                <button
                    type="button"
                    onclick={handleSave}
                    disabled={saving ||
                        (operation === "transfer" &&
                            otherAccounts.length === 0)}
                    class="border rounded-md p-1 hover:bg-zinc-700 border-zinc-800 flex items-center gap-2 px-3"
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
