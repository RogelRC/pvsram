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
            <span class="text-xs text-red-300">{error}</span>
        {/if}
        <button
            type="button"
            onclick={confirmDelete}
            disabled={deleting}
            class="btn-danger px-2 py-1 text-xs"
        >
            {deleting ? "…" : "Confirmar"}
        </button>
        <button
            type="button"
            onclick={cancel}
            disabled={deleting}
            class="btn px-2 py-1 text-xs"
        >
            Cancelar
        </button>
    </div>
{:else}
    <button
        type="button"
        onclick={askConfirm}
        class="btn-icon hover:border-red-500/40 hover:bg-red-500/10 hover:text-red-300"
        title="Eliminar"
        aria-label="Eliminar operación"
    >
        <Trash2 size={15} />
    </button>
{/if}
