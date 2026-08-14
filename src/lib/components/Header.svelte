<script lang="ts">
    import { page } from "$app/stores";
    import { invoke } from "@tauri-apps/api/core";
    import { open, save } from "@tauri-apps/plugin-dialog";
    import ChangePasswordButton from "$lib/components/Auth/ChangePasswordButton.svelte";
    import { Upload, Download, Settings } from "lucide-svelte";
    import logo from "$lib/assets/logo-small.svg";

    let isSettingsMenuOpen = $state(false);

    function toggleSettingsMenu() {
        isSettingsMenuOpen = !isSettingsMenuOpen;
    }

    function closeSettingsMenu() {
        isSettingsMenuOpen = false;
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
            defaultPath: "tesora_backup.db",
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

    const navItems = [
        { href: "/", label: "Tesorerías" },
        { href: "/history", label: "Historial" },
        { href: "/statistics", label: "Estadísticas" },
        { href: "/reports", label: "Informes" },
    ];
</script>

<header
    class="fixed top-0 right-0 left-0 z-50 border-b border-zinc-800/80 bg-zinc-950/80 backdrop-blur-xl"
>
    <nav class="mx-auto flex h-14 max-w-[1400px] items-center gap-3 px-4 md:px-6">
        <a
            href="/"
            class="group mr-2 flex shrink-0 items-center gap-2.5 rounded-lg py-1 pr-2 transition-opacity hover:opacity-90"
            aria-label="Tesora — inicio"
        >
            <img
                src={logo}
                alt=""
                class="h-8 w-8 rounded-lg shadow-sm shadow-indigo-500/20 ring-1 ring-white/5"
            />
            <div class="hidden leading-tight sm:block">
                <div
                    class="text-sm font-semibold tracking-tight text-zinc-50 group-hover:text-white"
                >
                    Tesora
                </div>
                <div class="text-[10px] font-medium tracking-wide text-zinc-500 uppercase">
                    Tesorerías
                </div>
            </div>
        </a>

        <div
            class="flex min-w-0 flex-1 items-center gap-1 overflow-x-auto py-1 [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
        >
            {#each navItems as item (item.href)}
                <a
                    href={item.href}
                    class="nav-link whitespace-nowrap {isActive(item.href)
                        ? 'nav-link-active'
                        : ''}"
                >
                    {item.label}
                </a>
            {/each}
        </div>

        <div class="relative shrink-0">
            <button
                type="button"
                class="btn"
                onclick={toggleSettingsMenu}
                title="Ajustes"
                aria-expanded={isSettingsMenuOpen}
                aria-haspopup="menu"
            >
                <Settings size={16} />
                <span class="hidden sm:inline">Ajustes</span>
            </button>
            {#if isSettingsMenuOpen}
                <button
                    type="button"
                    class="fixed inset-0 z-40 cursor-default"
                    aria-label="Cerrar menú de ajustes"
                    onclick={closeSettingsMenu}
                ></button>
                <div
                    class="absolute right-0 z-50 mt-2 flex w-60 flex-col gap-1 rounded-xl border border-zinc-800 bg-zinc-900/95 p-2 shadow-xl shadow-black/40 backdrop-blur-md"
                    role="menu"
                >
                    <button
                        type="button"
                        class="btn-ghost w-full justify-start"
                        role="menuitem"
                        onclick={() => {
                            closeSettingsMenu();
                            handleExportDatabase();
                        }}
                        title="Exportar base de datos"
                    >
                        <Upload size={16} />
                        Exportar BD
                    </button>
                    <button
                        type="button"
                        class="btn-ghost w-full justify-start"
                        role="menuitem"
                        onclick={() => {
                            closeSettingsMenu();
                            handleImportDatabase();
                        }}
                        title="Importar base de datos"
                    >
                        <Download size={16} />
                        Importar BD
                    </button>
                    <div class="my-1 border-t border-zinc-800"></div>
                    <ChangePasswordButton />
                </div>
            {/if}
        </div>
    </nav>
</header>

<style lang="postcss">
    @reference "tailwindcss";
</style>
