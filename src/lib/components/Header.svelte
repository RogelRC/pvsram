<script lang="ts">
    import { page } from "$app/stores";
    import { invoke } from "@tauri-apps/api/core";
    import { open, save } from "@tauri-apps/plugin-dialog";
    import ChangePasswordButton from "$lib/components/Auth/ChangePasswordButton.svelte";
    import { Upload, Download, Settings } from "lucide-svelte";

    let isSettingsMenuOpen = $state(false);

    function toggleSettingsMenu() {
        isSettingsMenuOpen = !isSettingsMenuOpen;
    }

    function isActive(path: string): boolean {
        const currentPath = $page.url.pathname;
        return (
            currentPath === path ||
            (path !== "/" && currentPath.startsWith(path))
        );
    }

    async function handleExportDatabase() {
        const selected = await save({
            defaultPath: "pvsr_accounts_backup.db",
            filters: [
                {
                    name: "Base de datos SQLite",
                    extensions: ["db", "sqlite", "sqlite3"],
                },
            ],
        });
        if (!selected) return;

        try {
            await invoke("export_database", { destination: selected });
            window.alert("Base de datos exportada correctamente");
        } catch (error) {
            window.alert(`No se pudo exportar la base de datos: ${error}`);
        }
    }

    async function handleImportDatabase() {
        const selected = await open({
            multiple: false,
            filters: [
                {
                    name: "Base de datos SQLite",
                    extensions: ["db", "sqlite", "sqlite3"],
                },
            ],
        });
        if (!selected) return;

        try {
            const path = Array.isArray(selected) ? selected[0] : selected;
            await invoke("import_database", { sourcePath: path });
            window.alert(
                "Base de datos importada correctamente. Reinicia la app para ver los cambios.",
            );
        } catch (error) {
            window.alert(`No se pudo importar la base de datos: ${error}`);
        }
    }
</script>

<div class="fixed top-0 left-0 right-0 z-50">
    <nav class="flex w-full gap-4 p-4 bg-[#171717] items-center">
        <a
            href="/"
            class="hover:underline {isActive('/')
                ? 'font-bold text-blue-500!'
                : ''}">Tesorerías</a
        >
        <a
            href="/history"
            class="hover:underline {isActive('/history')
                ? 'font-bold text-blue-500!'
                : ''}">Historial</a
        >
        <a
            href="/statistics"
            class="hover:underline {isActive('/statistics')
                ? 'font-bold text-blue-500!'
                : ''}">Estadísticas</a
        >
        <a
            href="/reports"
            class="hover:underline mr-auto {isActive('/reports')
                ? 'font-bold text-blue-500!'
                : ''}">Informes</a
        >
        <div class="relative">
            <button
                type="button"
                class="border rounded-md p-1 hover:bg-zinc-700 border-zinc-800 flex items-center gap-2 px-3"
                onclick={toggleSettingsMenu}
                title="Ajustes"
            >
                <Settings size={16} />
                Ajustes
            </button>
            {#if isSettingsMenuOpen}
                <div
                    class="absolute right-0 mt-2 flex w-56 flex-col gap-2 rounded-lg border border-zinc-800 bg-zinc-900 p-2 shadow-lg"
                >
                    <button
                        type="button"
                        class="text-left border rounded-md p-2 hover:bg-zinc-700 border-zinc-800 flex items-center gap-2"
                        onclick={() => {
                            isSettingsMenuOpen = false;
                            handleExportDatabase();
                        }}
                        title="Exportar base de datos"
                    >
                        <Upload size={16} />
                        Exportar BD
                    </button>
                    <button
                        type="button"
                        class="text-left border rounded-md p-2 hover:bg-zinc-700 border-zinc-800 flex items-center gap-2"
                        onclick={() => {
                            isSettingsMenuOpen = false;
                            handleImportDatabase();
                        }}
                        title="Importar base de datos"
                    >
                        <Download size={16} />
                        Importar BD
                    </button>
                    <ChangePasswordButton />
                </div>
            {/if}
        </div>
    </nav>
</div>

<style lang="postcss">
    @reference "tailwindcss";
</style>
