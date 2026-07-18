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

<button
    onclick={menu}
    class="border rounded-md p-1 hover:bg-zinc-700 border-zinc-800 flex items-center gap-2 px-3"
>
    <Plus />
    <span class="mr-2">Nueva tesorería</span>
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
                <h3 class="text-2xl font-bold">Nueva tesorería</h3>
                <button
                    onclick={menu}
                    class="border rounded-md p-1 hover:bg-zinc-700 border-zinc-800 flex items-center gap-2 px-3"
                >
                    <X />
                </button>
            </div>

            {#if error}
                <p class="text-red-400 text-sm">{error}</p>
            {/if}

            <div class="flex items-center gap-2">
                <span>Número:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="text"
                    bind:value={number}
                    disabled={saving}
                />
            </div>
            <div class="flex items-center gap-2">
                <span>Nombre:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="text"
                    bind:value={name}
                    disabled={saving}
                />
            </div>
            <div class="flex items-center gap-2">
                <span>Destino:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="text"
                    bind:value={description}
                    disabled={saving}
                />
            </div>
            <div class="flex items-center gap-2">
                <span>Comentario:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="text"
                    bind:value={owner}
                    disabled={saving}
                />
            </div>
            <div class="flex items-center gap-2">
                <span>Moneda:</span>
                {#if isCustomCurrency}
                    <button
                        onclick={backToSelect}
                        disabled={saving}
                        class="flex items-center justify-center border rounded-md bg-white/10 hover:bg-white/20 p-1 shrink-0"
                        title="Volver a la lista"
                    >
                        <ArrowLeft size={16} />
                    </button>
                    <input
                        class="p-1 w-full bg-zinc-800 rounded-md uppercase"
                        type="text"
                        maxlength="10"
                        placeholder="Ej: MXN"
                        bind:value={customCurrency}
                        disabled={saving}
                    />
                {:else}
                    <select
                        class="p-1 w-full bg-zinc-800 rounded-md"
                        value={selectedCurrency}
                        onchange={onCurrencySelect}
                        disabled={saving}
                    >
                        {#each availableCurrencies as cur (cur)}
                            <option value={cur}>{cur}</option>
                        {/each}
                        <option value={CUSTOM_OPTION}>Otra...</option>
                    </select>
                {/if}
            </div>
            <div class="flex items-center gap-2">
                <span>Color:</span>
                <input
                    class="h-8 w-14 bg-zinc-800 rounded-md border border-zinc-700 cursor-pointer"
                    type="color"
                    bind:value={color}
                    disabled={saving}
                />
                <span class="text-sm text-zinc-400">{color}</span>
            </div>
            <div class="flex justify-end">
                <button
                    onclick={handleSave}
                    disabled={saving}
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
