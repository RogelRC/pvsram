<script lang="ts">
    import { page } from "$app/stores";
    import ChangePasswordButton from "$lib/components/Auth/ChangePasswordButton.svelte";

    function isActive(path: string): boolean {
        const currentPath = $page.url.pathname;
        return (
            currentPath === path ||
            (path !== "/" && currentPath.startsWith(path))
        );
    }
</script>

<div class="fixed top-0 left-0 right-0 z-50">
    <nav class="flex w-full gap-4 p-4 bg-[#171717] items-center">
        <a
            href="/"
            class="hover:underline {isActive('/')
                ? 'font-bold text-blue-500!'
                : ''}">Cuentas</a
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
        <ChangePasswordButton />
    </nav>
</div>

<style lang="postcss">
    @reference "tailwindcss";
</style>
