<script lang="ts">
    let { children } = $props();

    import Header from "$lib/components/Header.svelte";
    import "../app.css";

    // Auth imports
    import { onMount } from "svelte";
    import { hasPassword, isAuthenticated } from "$lib/services/auth";
    import LoginScreen from "$lib/components/Auth/LoginScreen.svelte";

    let loading = $state(true);
    let authenticated = $state(false);
    let passwordExists = $state(false);

    async function checkAuth() {
        loading = true;
        passwordExists = await hasPassword();
        authenticated = passwordExists ? await isAuthenticated() : false;
        loading = false;
    }

    function handleAuthenticated() {
        authenticated = true;
    }

    onMount(checkAuth);
</script>

<div class="min-h-screen bg-black text-white flex flex-col">
    {#if loading}
        <div
            class="flex items-center justify-center h-screen text-zinc-400 bg-zinc-950"
        >
            Cargando...
        </div>
    {:else if !authenticated}
        <LoginScreen
            mode={passwordExists ? "login" : "setup"}
            onSuccess={handleAuthenticated}
        />
    {:else}
        <Header />
        <main class="flex-1 pt-16">
            {@render children()}
        </main>
    {/if}
</div>

<style lang="postcss">
    @reference "tailwindcss";
</style>
