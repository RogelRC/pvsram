<script lang="ts">
    import { X, Save } from "lucide-svelte";
    import { updateAccount, type Account } from "$lib/services/accounts";

    let { account, onUpdated }: { account: Account; onUpdated?: () => void } =
        $props();

    let isOpen = $state(false);
    let saving = $state(false);
    let error = $state<string | null>(null);

    let name = $state("");
    let destination = $state("");
    let comment = $state("");
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
        destination = account.destination ?? "";
        comment = account.comment;
        color = account.color ?? "#a1a1aa";
        error = null;
        saving = false;
    }

    async function handleSave() {
        error = null;

        if (!name.trim() || !comment.trim()) {
            error = "Nombre y propietario son obligatorios";
            return;
        }

        saving = true;
        try {
            await updateAccount({
                id: account.id,
                number: account.number,
                name: name.trim(),
                destination: destination.trim() || null,
                comment: comment.trim(),
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
    class="w-full rounded-lg px-3 py-2 text-left text-sm text-zinc-200 transition-colors hover:bg-zinc-800"
>
    Editar tesorería
</button>
{#if isOpen}
    <div class="modal-backdrop">
        <div class="modal-panel max-w-xl">
            <div
                class="flex items-center justify-between border-b border-zinc-800 pb-3"
            >
                <h3 class="text-lg font-semibold text-zinc-50">
                    Editar tesorería
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
                        class="field w-full cursor-not-allowed opacity-60"
                        type="text"
                        value={account.number}
                        disabled
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
                        bind:value={destination}
                        disabled={saving}
                    />
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
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Moneda</span>
                    <input
                        class="field w-full cursor-not-allowed opacity-60"
                        type="text"
                        value={account.currency}
                        disabled
                    />
                </label>
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
