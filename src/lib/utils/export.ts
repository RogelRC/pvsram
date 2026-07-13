import { save } from "@tauri-apps/plugin-dialog";
import { writeFile, writeTextFile } from "@tauri-apps/plugin-fs";
import * as XLSX from "xlsx";
import jsPDF from "jspdf";
import autoTable from "jspdf-autotable";

export interface ExportColumn<T> {
    header: string;
    accessor: (row: T) => string | number;
}

function buildRows<T>(data: T[], columns: ExportColumn<T>[]): (string | number)[][] {
    return data.map((row) => columns.map((col) => col.accessor(row)));
}

function escapeCsv(value: string): string {
    if (/[",\n]/.test(value)) {
        return `"${value.replace(/"/g, '""')}"`;
    }
    return value;
}

export async function exportToCsv<T>(
    data: T[],
    columns: ExportColumn<T>[],
    filename: string,
): Promise<void> {
    const headerRow = columns.map((c) => escapeCsv(c.header)).join(",");
    const rows = buildRows(data, columns).map((r) =>
        r.map((v) => escapeCsv(String(v))).join(","),
    );
    const content = [headerRow, ...rows].join("\n");

    const path = await save({
        defaultPath: `${filename}.csv`,
        filters: [{ name: "CSV", extensions: ["csv"] }],
    });
    if (!path) return;

    // BOM al inicio para que Excel abra bien los acentos en UTF-8
    await writeTextFile(path, "\uFEFF" + content);
}

export async function exportToExcel<T>(
    data: T[],
    columns: ExportColumn<T>[],
    filename: string,
    sheetName = "Datos",
): Promise<void> {
    const headerRow = columns.map((c) => c.header);
    const rows = buildRows(data, columns);
    const sheetData = [headerRow, ...rows];

    const worksheet = XLSX.utils.aoa_to_sheet(sheetData);
    const workbook = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(workbook, worksheet, sheetName);

    const arrayBuffer = XLSX.write(workbook, {
        type: "array",
        bookType: "xlsx",
    }) as ArrayBuffer;

    const path = await save({
        defaultPath: `${filename}.xlsx`,
        filters: [{ name: "Excel", extensions: ["xlsx"] }],
    });
    if (!path) return;

    await writeFile(path, new Uint8Array(arrayBuffer));
}

export async function exportToPdf<T>(
    title: string,
    data: T[],
    columns: ExportColumn<T>[],
    filename: string,
): Promise<void> {
    const doc = new jsPDF();

    doc.setFontSize(14);
    doc.text(title, 14, 15);
    doc.setFontSize(9);
    doc.setTextColor(120);
    doc.text(new Date().toLocaleString(), 14, 21);

    autoTable(doc, {
        startY: 26,
        head: [columns.map((c) => c.header)],
        body: buildRows(data, columns).map((r) => r.map((v) => String(v))),
        styles: { fontSize: 8 },
        headStyles: { fillColor: [39, 39, 42] },
    });

    const arrayBuffer = doc.output("arraybuffer") as ArrayBuffer;

    const path = await save({
        defaultPath: `${filename}.pdf`,
        filters: [{ name: "PDF", extensions: ["pdf"] }],
    });
    if (!path) return;

    await writeFile(path, new Uint8Array(arrayBuffer));
}
