<script lang="ts">
    import Pencil from "@lucide/svelte/icons/pencil";
    import { X, Save } from "lucide-svelte";
    import { updateAccount, type Account } from "$lib/services/accounts";

    let { account, onUpdated }: { account: Account; onUpdated?: () => void } =
        $props();

    let isOpen = $state(false);
    let saving = $state(false);
    let error = $state<string | null>(null);

    let name = $state("");
    let description = $state("");
    let owner = $state("");
    let color = $state("#a1a1aa");

    function menu() {
        isOpen = !isOpen;
        if (isOpen) {
            loadFromAccount();
        } else {
            error = null;
            saving = false;
        }
    }

    function loadFromAccount() {
        name = account.name;
        description = account.description ?? "";
        owner = account.owner;
        color = account.color ?? "#a1a1aa";
        error = null;
        saving = false;
    }

    async function handleSave() {
        error = null;

        if (!name.trim() || !owner.trim()) {
            error = "Nombre y propietario son obligatorios";
            return;
        }

        saving = true;
        try {
            await updateAccount({
                id: account.id,
                number: account.number,
                name: name.trim(),
                description: description.trim() || null,
                owner: owner.trim(),
                currency: account.currency,
                color,
            });
            onUpdated?.();
            isOpen = false;
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            saving = false;
        }
    }
</script>

<button
    onclick={menu}
    class="w-full text-left px-3 py-2 rounded hover:bg-red-600/10 text-red-400"
>
    Editar tesorería
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
                <h3 class="text-2xl font-bold">Editar tesorería</h3>
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
                    class="p-1 w-full bg-zinc-800/50 rounded-md text-zinc-400 cursor-not-allowed"
                    type="text"
                    value={account.number}
                    disabled
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
                <input
                    class="p-1 w-full bg-zinc-800/50 rounded-md text-zinc-400 cursor-not-allowed"
                    type="text"
                    value={account.currency}
                    disabled
                />
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
