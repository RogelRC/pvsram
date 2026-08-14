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

<div class="flex min-h-screen flex-col text-zinc-100">
    {#if loading}
        <div
            class="flex h-screen flex-col items-center justify-center gap-3 text-zinc-400"
        >
            <div
                class="h-8 w-8 animate-pulse rounded-xl bg-indigo-500/20 ring-1 ring-indigo-400/30"
            ></div>
            <p class="text-sm tracking-wide">Cargando Tesora…</p>
        </div>
    {:else if !authenticated}
        <LoginScreen
            mode={passwordExists ? "login" : "setup"}
            onSuccess={handleAuthenticated}
        />
    {:else}
        <Header />
        <main class="flex-1 pt-14">
            {@render children()}
        </main>
    {/if}
</div>

<style lang="postcss">
    @reference "tailwindcss";
</style>
