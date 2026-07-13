<script lang="ts">
    import Trash2 from "@lucide/svelte/icons/trash-2";
    import { deleteTransaction } from "$lib/services/transactions";

    let { id, onDeleted }: { id: number; onDeleted?: () => void } = $props();

    let confirming = $state(false);
    let deleting = $state(false);
    let error = $state<string | null>(null);

    function askConfirm() {
        confirming = true;
        error = null;
    }

    function cancel() {
        confirming = false;
    }

    async function confirmDelete() {
        deleting = true;
        error = null;
        try {
            await deleteTransaction(id);
            onDeleted?.();
            confirming = false;
        } catch (e) {
            console.error("Error al eliminar la operación:", e);
            error = e instanceof Error ? e.message : String(e);
        } finally {
            deleting = false;
        }
    }
</script>

{#if confirming}
    <div class="flex items-center gap-1">
        {#if error}
            <span class="text-red-400 text-xs">{error}</span>
        {/if}
        <button
            type="button"
            onclick={confirmDelete}
            disabled={deleting}
            class="border rounded-md p-1 bg-red-500/20 hover:bg-red-500/30 border-red-800 text-red-400 text-xs px-2 disabled:opacity-50"
        >
            {deleting ? "..." : "Confirmar"}
        </button>
        <button
            type="button"
            onclick={cancel}
            disabled={deleting}
            class="border rounded-md p-1 hover:bg-zinc-700 border-zinc-800 text-xs px-2"
        >
            Cancelar
        </button>
    </div>
{:else}
    <button
        type="button"
        onclick={askConfirm}
        class="border rounded-md p-1 hover:bg-red-500/20 hover:border-red-800 hover:text-red-400 border-zinc-800"
    >
        <Trash2 size={16} />
    </button>
{/if}
