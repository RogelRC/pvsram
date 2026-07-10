<script>
    let { children } = $props();

    import Header from "$lib/components/Header.svelte";
    import "../app.css";
</script>

<div class="min-h-screen bg-black text-white flex flex-col">
    <Header />

    <main class="flex-1">
        {@render children()}
    </main>
</div>

<style lang="postcss">
    @reference "tailwindcss";
</style>
