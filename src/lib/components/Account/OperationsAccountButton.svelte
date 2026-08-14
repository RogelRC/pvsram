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

<button
    type="button"
    onclick={menu}
    class="btn-icon"
    title="Nueva operación"
    aria-label="Nueva operación"
>
    <ArrowLeftRight size={18} />
</button>
{#if isOpen}
    <div class="modal-backdrop">
        <div class="modal-panel max-w-xl">
            <div
                class="flex items-center justify-between border-b border-zinc-800 pb-3"
            >
                <h3 class="text-lg font-semibold text-zinc-50">
                    Nueva operación
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

            <div class="grid grid-cols-3 gap-2">
                {#each OPERATIONS as op (op.value)}
                    <button
                        type="button"
                        onclick={() => selectOperation(op.value)}
                        disabled={saving}
                        class="rounded-lg border px-2 py-2 text-sm font-medium transition-all {operation ===
                        op.value
                            ? 'border-indigo-400/40 bg-indigo-500/15 text-indigo-100'
                            : 'border-zinc-800 bg-zinc-950/40 text-zinc-400 hover:border-zinc-700 hover:bg-zinc-800/60 hover:text-zinc-200'}"
                    >
                        {op.label}
                    </button>
                {/each}
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
                    <span class="label-muted">Cuenta</span>
                    <input
                        class="field w-full cursor-not-allowed opacity-60"
                        type="text"
                        value="{account.name} ({account.number})"
                        disabled
                    />
                </label>

                {#if operation === "transfer"}
                    <div class="flex flex-col gap-1.5">
                        <span class="label-muted">Destino</span>
                        {#if otherAccounts.length === 0}
                            <span class="text-sm text-zinc-400">
                                No hay otras cuentas en {account.currency} disponibles
                                para transferir
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

                <div class="flex flex-col gap-1.5">
                    <span class="label-muted">Concepto</span>
                    <div class="flex items-center gap-2">
                        {#if isCustomConcept}
                            {#if availableConcepts.length > 0}
                                <button
                                    type="button"
                                    onclick={backToConceptSelect}
                                    disabled={saving}
                                    class="btn-icon shrink-0"
                                    title="Volver a la lista"
                                >
                                    <ArrowLeft size={16} />
                                </button>
                            {/if}
                            <input
                                class="field w-full"
                                type="text"
                                placeholder="Nombre del concepto"
                                bind:value={customConcept}
                                disabled={saving}
                            />
                        {:else}
                            <select
                                class="field w-full"
                                value={selectedConceptId ?? ""}
                                onchange={onConceptSelect}
                                disabled={saving}
                            >
                                {#each availableConcepts as concept (concept.id)}
                                    <option value={concept.id}
                                        >{concept.name}</option
                                    >
                                {/each}
                                <option value={CUSTOM_OPTION}
                                    >Agregar otro…</option
                                >
                            </select>
                        {/if}
                    </div>
                </div>

                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Monto</span>
                    <div class="flex items-center gap-2">
                        <input
                            class="field w-full"
                            type="number"
                            min="0"
                            step="0.01"
                            placeholder="0.00"
                            bind:value={amount}
                            disabled={saving}
                        />
                        <span
                            class="shrink-0 rounded-md border border-zinc-800 bg-zinc-950/50 px-2 py-2 text-sm text-zinc-400"
                        >
                            {account.currency}
                        </span>
                    </div>
                </label>

                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Comentario</span>
                    <input
                        class="field w-full"
                        type="text"
                        bind:value={comment}
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
                        (operation === "transfer" &&
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
