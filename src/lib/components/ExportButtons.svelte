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

<div class="ml-auto flex items-center gap-1.5">
    {#if error}
        <span class="mr-1 text-xs text-red-300">{error}</span>
    {/if}
    <button
        type="button"
        disabled={exporting || data.length === 0}
        onclick={() => handleExport(() => exportToCsv(data, columns, filename))}
        class="btn-icon"
        title="Exportar a CSV"
    >
        <FileText size={16} />
    </button>
    <button
        type="button"
        disabled={exporting || data.length === 0}
        onclick={() =>
            handleExport(() => exportToExcel(data, columns, filename))}
        class="btn-icon"
        title="Exportar a Excel"
    >
        <FileSpreadsheet size={16} />
    </button>
    <button
        type="button"
        disabled={exporting || data.length === 0}
        onclick={() =>
            handleExport(() => exportToPdf(title, data, columns, filename))}
        class="btn-icon"
        title="Exportar a PDF"
    >
        <FileDown size={16} />
    </button>
</div>
