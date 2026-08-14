<script lang="ts">
    import { Plus, X, Save, ArrowLeft } from "lucide-svelte";
    import { createAccount, listAccounts } from "$lib/services/accounts";

    let { onCreated }: { onCreated?: () => void } = $props();

    const CUSTOM_OPTION = "__custom__";
    const DEFAULT_CURRENCIES = ["USD", "EUR", "CUP", "MLC"];

    let isOpen = $state(false);
    let saving = $state(false);
    let error = $state<string | null>(null);

    let number = $state("");
    let name = $state("");
    let description = $state("");
    let owner = $state("");
    let color = $state("#a1a1aa"); // valor por defecto (zinc-400)

    let availableCurrencies = $state<string[]>(DEFAULT_CURRENCIES);
    let selectedCurrency = $state("USD");
    let isCustomCurrency = $state(false);
    let customCurrency = $state("");

    async function menu() {
        isOpen = !isOpen;
        if (isOpen) {
            await loadCurrencies();
        } else {
            resetForm();
        }
    }

    async function loadCurrencies() {
        try {
            const accounts = await listAccounts();
            const used = accounts.map((a) => a.currency).filter(Boolean);
            const merged = Array.from(
                new Set([...DEFAULT_CURRENCIES, ...used]),
            ).sort();
            availableCurrencies = merged;
        } catch {
            availableCurrencies = DEFAULT_CURRENCIES;
        }
    }

    function resetForm() {
        number = "";
        name = "";
        description = "";
        owner = "";
        color = "#a1a1aa";
        selectedCurrency = "USD";
        isCustomCurrency = false;
        customCurrency = "";
        error = null;
        saving = false;
    }

    function onCurrencySelect(e: Event) {
        const value = (e.target as HTMLSelectElement).value;
        if (value === CUSTOM_OPTION) {
            isCustomCurrency = true;
            customCurrency = "";
        } else {
            selectedCurrency = value;
        }
    }

    function backToSelect() {
        isCustomCurrency = false;
        customCurrency = "";
    }

    function resolvedCurrency(): string {
        return isCustomCurrency
            ? customCurrency.trim().toUpperCase()
            : selectedCurrency;
    }

    async function handleSave() {
        error = null;

        const currency = resolvedCurrency();

        if (!number.trim() || !name.trim() || !owner.trim()) {
            error = "Número, nombre y propietario son obligatorios";
            return;
        }
        if (!currency) {
            error = "Debes indicar una moneda";
            return;
        }

        saving = true;
        try {
            await createAccount({
                number: number.trim(),
                name: name.trim(),
                destination: description.trim() || null,
                comment: owner.trim(),
                currency,
                color,
            });
            onCreated?.();
            isOpen = false;
            resetForm();
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            saving = false;
        }
    }
</script>

<button onclick={menu} class="btn-primary">
    <Plus size={16} />
    <span>Nueva tesorería</span>
</button>
{#if isOpen}
    <div class="modal-backdrop">
        <div class="modal-panel max-w-xl">
            <div
                class="flex items-center justify-between border-b border-zinc-800 pb-3"
            >
                <h3 class="text-lg font-semibold text-zinc-50">
                    Nueva tesorería
                </h3>
                <button onclick={menu} class="btn-icon" aria-label="Cerrar">
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
                    <span class="label-muted">Número</span>
                    <input
                        class="field w-full"
                        type="text"
                        bind:value={number}
                        disabled={saving}
                    />
                </label>
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Nombre</span>
                    <input
                        class="field w-full"
                        type="text"
                        bind:value={name}
                        disabled={saving}
                    />
                </label>
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Destino</span>
                    <input
                        class="field w-full"
                        type="text"
                        bind:value={description}
                        disabled={saving}
                    />
                </label>
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Comentario</span>
                    <input
                        class="field w-full"
                        type="text"
                        bind:value={owner}
                        disabled={saving}
                    />
                </label>
                <div class="flex flex-col gap-1.5">
                    <span class="label-muted">Moneda</span>
                    <div class="flex items-center gap-2">
                        {#if isCustomCurrency}
                            <button
                                onclick={backToSelect}
                                disabled={saving}
                                class="btn-icon shrink-0"
                                title="Volver a la lista"
                            >
                                <ArrowLeft size={16} />
                            </button>
                            <input
                                class="field w-full uppercase"
                                type="text"
                                maxlength="10"
                                placeholder="Ej: MXN"
                                bind:value={customCurrency}
                                disabled={saving}
                            />
                        {:else}
                            <select
                                class="field w-full"
                                value={selectedCurrency}
                                onchange={onCurrencySelect}
                                disabled={saving}
                            >
                                {#each availableCurrencies as cur (cur)}
                                    <option value={cur}>{cur}</option>
                                {/each}
                                <option value={CUSTOM_OPTION}>Otra…</option>
                            </select>
                        {/if}
                    </div>
                </div>
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Color</span>
                    <div class="flex items-center gap-3">
                        <input
                            class="h-10 w-14 cursor-pointer rounded-lg border border-zinc-700 bg-zinc-950"
                            type="color"
                            bind:value={color}
                            disabled={saving}
                        />
                        <span class="font-mono text-sm text-zinc-400"
                            >{color}</span
                        >
                    </div>
                </label>
            </div>

            <div class="flex justify-end gap-2 border-t border-zinc-800 pt-3">
                <button onclick={menu} class="btn" disabled={saving}>
                    Cancelar
                </button>
                <button
                    onclick={handleSave}
                    disabled={saving}
                    class="btn-primary"
                >
                    <Save size={16} />
                    <span>{saving ? "Guardando…" : "Guardar"}</span>
                </button>
            </div>
        </div>
    </div>
{/if}
