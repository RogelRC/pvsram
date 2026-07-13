<script lang="ts" generics="T">
    import { FileText, FileSpreadsheet, FileDown } from "lucide-svelte";
    import {
        exportToCsv,
        exportToExcel,
        exportToPdf,
        type ExportColumn,
    } from "$lib/utils/export";

    let {
        title,
        data,
        columns,
        filename,
    }: {
        title: string;
        data: T[];
        columns: ExportColumn<T>[];
        filename: string;
    } = $props();

    let exporting = $state(false);
    let error = $state<string | null>(null);

    async function handleExport(fn: () => Promise<void>) {
        error = null;
        exporting = true;
        try {
            await fn();
        } catch (e) {
            console.error(e);
            error = e instanceof Error ? e.message : String(e);
        } finally {
            exporting = false;
        }
    }
</script>

<div class="flex items-center gap-2">
    {#if error}
        <span class="text-red-400 text-xs">{error}</span>
    {/if}
    <button
        type="button"
        disabled={exporting || data.length === 0}
        onclick={() => handleExport(() => exportToCsv(data, columns, filename))}
        class="border rounded-md p-1 hover:bg-zinc-700 border-zinc-800 disabled:opacity-50"
        title="Exportar a CSV"
    >
        <FileText size={18} />
    </button>
    <button
        type="button"
        disabled={exporting || data.length === 0}
        onclick={() =>
            handleExport(() => exportToExcel(data, columns, filename))}
        class="border rounded-md p-1 hover:bg-zinc-700 border-zinc-800 disabled:opacity-50"
        title="Exportar a Excel"
    >
        <FileSpreadsheet size={18} />
    </button>
    <button
        type="button"
        disabled={exporting || data.length === 0}
        onclick={() =>
            handleExport(() => exportToPdf(title, data, columns, filename))}
        class="border rounded-md p-1 hover:bg-zinc-700 border-zinc-800 disabled:opacity-50"
        title="Exportar a PDF"
    >
        <FileDown size={18} />
    </button>
</div>
