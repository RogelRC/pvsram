<script lang="ts">
    import { Lock, LogIn } from "lucide-svelte";
    import { login, setInitialPassword } from "$lib/services/auth";

    let {
        mode,
        onSuccess,
    }: { mode: "login" | "setup"; onSuccess: () => void } = $props();

    let password = $state("");
    let confirmPassword = $state("");
    let saving = $state(false);
    let error = $state<string | null>(null);

    async function handleSubmit() {
        error = null;

        if (mode === "setup") {
            if (password.length < 4) {
                error = "La contraseña debe tener al menos 4 caracteres";
                return;
            }
            if (password !== confirmPassword) {
                error = "Las contraseñas no coinciden";
                return;
            }
            saving = true;
            try {
                await setInitialPassword(password);
                onSuccess();
            } catch (e) {
                error = e instanceof Error ? e.message : String(e);
            } finally {
                saving = false;
            }
        } else {
            if (!password) {
                error = "Introduce tu contraseña";
                return;
            }
            saving = true;
            try {
                const ok = await login(password);
                if (ok) {
                    onSuccess();
                } else {
                    error = "Contraseña incorrecta";
                    password = "";
                }
            } catch (e) {
                error = e instanceof Error ? e.message : String(e);
            } finally {
                saving = false;
            }
        }
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Enter") handleSubmit();
    }
</script>

<div class="flex items-center justify-center h-screen bg-zinc-950">
    <div
        class="flex flex-col bg-zinc-900 border border-zinc-700 rounded-2xl shadow-2xl p-6 w-80 gap-4"
    >
        <div class="flex flex-col items-center gap-2">
            <div class="border rounded-full p-3 bg-white/10 border-zinc-800">
                <Lock size={24} />
            </div>
            <h2 class="text-xl font-bold">
                {mode === "setup" ? "Crea tu contraseña" : "Iniciar sesión"}
            </h2>
            {#if mode === "setup"}
                <p class="text-sm text-zinc-400 text-center">
                    Esta contraseña protegerá el acceso a la aplicación
                </p>
            {/if}
        </div>

        {#if error}
            <p class="text-red-400 text-sm text-center">{error}</p>
        {/if}

        <div class="flex flex-col gap-2">
            <input
                class="p-2 w-full bg-zinc-800 rounded-md"
                type="password"
                placeholder="Contraseña"
                bind:value={password}
                onkeydown={handleKeydown}
                disabled={saving}
                autofocus
            />
            {#if mode === "setup"}
                <input
                    class="p-2 w-full bg-zinc-800 rounded-md"
                    type="password"
                    placeholder="Confirmar contraseña"
                    bind:value={confirmPassword}
                    onkeydown={handleKeydown}
                    disabled={saving}
                />
            {/if}
        </div>

        <button
            type="button"
            onclick={handleSubmit}
            disabled={saving}
            class="flex items-center justify-center gap-2 border rounded-lg bg-white/10 hover:bg-white/20 border-zinc-800 p-2 disabled:opacity-50"
        >
            <span>{saving ? "..." : mode === "setup" ? "Crear" : "Entrar"}</span
            >
            <LogIn size={18} />
        </button>
    </div>
</div>
