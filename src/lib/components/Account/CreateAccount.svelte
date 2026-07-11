<script lang="ts">
    import { Plus, X, Save } from "lucide-svelte";
    import { createAccount } from "$lib/services/accounts";

    let { onCreated }: { onCreated?: () => void } = $props();

    let isOpen = $state(false);
    let saving = $state(false);
    let error = $state<string | null>(null);

    let number = $state("");
    let name = $state("");
    let description = $state("");
    let owner = $state("");
    let color = $state("#a1a1aa"); // valor por defecto (zinc-400)

    function menu() {
        isOpen = !isOpen;
        if (!isOpen) resetForm();
    }

    function resetForm() {
        number = "";
        name = "";
        description = "";
        owner = "";
        color = "#a1a1aa";
        error = null;
        saving = false;
    }

    async function handleSave() {
        error = null;

        if (!number.trim() || !name.trim() || !owner.trim()) {
            error = "Número, nombre y propietario son obligatorios";
            return;
        }

        saving = true;
        try {
            await createAccount({
                number: number.trim(),
                name: name.trim(),
                description: description.trim() || null,
                owner: owner.trim(),
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
    class="flex items-center justify-items-center border rounded-lg bg-white/10 hover:bg-white/20 p-2"
>
    <span class="mr-2">Nueva cuenta</span>
    <Plus />
</button>
{#if isOpen}
    <div
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-xs"
    >
        <div
            class="flex flex-col bg-zinc-900 border border-zinc-700 rounded-2xl shadow-2xl p-4 min-w-120 gap-4"
        >
            <div class="flex justify-between border-b items-center pb-2">
                <h3 class="text-2xl font-bold">Nueva cuenta</h3>
                <button
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
                <span>Descripción:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="text"
                    bind:value={description}
                    disabled={saving}
                />
            </div>
            <div class="flex items-center gap-2">
                <span>Propietario:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="text"
                    bind:value={owner}
                    disabled={saving}
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
