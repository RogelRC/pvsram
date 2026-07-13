<script lang="ts">
    import ChangePasswordButton from "$lib/components/Auth/ChangePasswordButton.svelte";
</script>

<div>
    <nav class="flex w-full gap-4 p-4 bg-[#171717]">
        <a href="/" class="hover:underline">Cuentas</a>
        <a href="/history" class="hover:underline">Historial</a>
        <a href="/statistics" class="hover:underline">Estadísticas</a>
        <a href="/reports" class="hover:underline mr-auto">Informes</a>
        <ChangePasswordButton />
    </nav>
</div>

<style lang="postcss">
    @reference "tailwindcss";
</style>
