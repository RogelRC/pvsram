<script lang="ts">
    import { Lock, LogIn } from "lucide-svelte";
    import { login, setInitialPassword } from "$lib/services/auth";
    import logo from "$lib/assets/logo.svg";

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

<div class="flex min-h-screen items-center justify-center p-4">
    <div
        class="flex w-full max-w-sm flex-col gap-5 rounded-2xl border border-zinc-800/90 bg-zinc-900/80 p-6 shadow-2xl shadow-black/40 backdrop-blur-md"
    >
        <div class="flex flex-col items-center gap-3 text-center">
            <img
                src={logo}
                alt="Tesora"
                class="h-14 w-14 rounded-2xl shadow-lg shadow-indigo-500/20 ring-1 ring-white/10"
            />
            <div>
                <h1 class="text-xl font-semibold tracking-tight text-zinc-50">
                    Tesora
                </h1>
                <p class="mt-0.5 text-xs font-medium tracking-wide text-zinc-500 uppercase">
                    Gestión de tesorerías
                </p>
            </div>
            <div
                class="mt-1 flex items-center gap-2 rounded-full border border-zinc-800 bg-zinc-950/50 px-3 py-1 text-xs text-zinc-400"
            >
                <Lock size={12} class="text-indigo-300" />
                {mode === "setup" ? "Configura tu acceso" : "Iniciar sesión"}
            </div>
        </div>

        {#if mode === "setup"}
            <p class="text-center text-sm leading-relaxed text-zinc-400">
                Crea una contraseña para proteger el acceso a tus cuentas y
                operaciones.
            </p>
        {/if}

        {#if error}
            <p
                class="rounded-lg border border-red-500/20 bg-red-500/10 px-3 py-2 text-center text-sm text-red-300"
            >
                {error}
            </p>
        {/if}

        <div class="flex flex-col gap-2.5">
            <label class="flex flex-col gap-1.5">
                <span class="label-muted">Contraseña</span>
                <input
                    class="field"
                    type="password"
                    placeholder="••••••••"
                    bind:value={password}
                    onkeydown={handleKeydown}
                    disabled={saving}
                    autocomplete={mode === "setup"
                        ? "new-password"
                        : "current-password"}
                />
            </label>

            {#if mode === "setup"}
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Confirmar contraseña</span>
                    <input
                        class="field"
                        type="password"
                        placeholder="••••••••"
                        bind:value={confirmPassword}
                        onkeydown={handleKeydown}
                        disabled={saving}
                        autocomplete="new-password"
                    />
                </label>
            {/if}
        </div>

        <button
            type="button"
            onclick={handleSubmit}
            disabled={saving}
            class="btn-primary w-full py-2.5"
        >
            <span>
                {saving
                    ? "…"
                    : mode === "setup"
                      ? "Crear contraseña"
                      : "Entrar"}
            </span>
            <LogIn size={16} />
        </button>
    </div>
</div>
