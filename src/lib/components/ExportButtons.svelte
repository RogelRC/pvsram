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
        fetchAllData,
        canExport,
    }: {
        title: string;
        data: T[];
        columns: ExportColumn<T>[];
        filename: string;
        fetchAllData?: () => Promise<T[]>;
        canExport?: boolean;
    } = $props();

    let exporting = $state(false);
    let error = $state<string | null>(null);

    const enabled = $derived(canExport ?? data.length > 0);

    async function getRows(): Promise<T[]> {
        return fetchAllData ? await fetchAllData() : data;
    }

    async function handleExport(kind: "csv" | "excel" | "pdf") {
        error = null;
        exporting = true;
        try {
            const rows = await getRows();
            if (kind === "csv") {
                await exportToCsv(rows, columns, filename);
            } else if (kind === "excel") {
                await exportToExcel(rows, columns, filename);
            } else {
                await exportToPdf(title, rows, columns, filename);
            }
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
        disabled={exporting || !enabled}
        onclick={() => handleExport("csv")}
        class="btn-icon"
        title="Exportar a CSV"
    >
        <FileText size={16} />
    </button>
    <button
        type="button"
        disabled={exporting || !enabled}
        onclick={() => handleExport("excel")}
        class="btn-icon"
        title="Exportar a Excel"
    >
        <FileSpreadsheet size={16} />
    </button>
    <button
        type="button"
        disabled={exporting || !enabled}
        onclick={() => handleExport("pdf")}
        class="btn-icon"
        title="Exportar a PDF"
    >
        <FileDown size={16} />
    </button>
</div>
