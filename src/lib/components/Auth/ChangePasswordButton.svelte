<script lang="ts">
    import { KeyRound, X, Save } from "lucide-svelte";
    import { changePassword } from "$lib/services/auth";

    let isOpen = $state(false);
    let saving = $state(false);
    let error = $state<string | null>(null);
    let success = $state(false);

    let currentPassword = $state("");
    let newPassword = $state("");
    let confirmPassword = $state("");

    function menu() {
        isOpen = !isOpen;
        if (isOpen) resetForm();
    }

    function resetForm() {
        currentPassword = "";
        newPassword = "";
        confirmPassword = "";
        error = null;
        success = false;
        saving = false;
    }

    async function handleSave() {
        error = null;
        success = false;

        if (newPassword.length < 4) {
            error = "La nueva contraseña debe tener al menos 4 caracteres";
            return;
        }
        if (newPassword !== confirmPassword) {
            error = "Las contraseñas nuevas no coinciden";
            return;
        }

        saving = true;
        try {
            await changePassword({ currentPassword, newPassword });
            success = true;
            currentPassword = "";
            newPassword = "";
            confirmPassword = "";
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
        } finally {
            saving = false;
        }
    }
</script>

<button
    type="button"
    onclick={menu}
    class="border rounded-md p-1 hover:bg-zinc-700 border-zinc-800 flex items-center gap-2 px-3"
>
    <KeyRound size={16} />
    <span>Cambiar contraseña</span>
</button>
{#if isOpen}
    <div
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-xs"
    >
        <div
            class="flex flex-col bg-zinc-900 border border-zinc-700 rounded-2xl shadow-2xl p-4 min-w-96 gap-4"
        >
            <div class="flex justify-between border-b items-center pb-2">
                <h3 class="text-2xl font-bold">Cambiar contraseña</h3>
                <button
                    type="button"
                    onclick={menu}
                    class="flex items-center justify-items-center border rounded-lg bg-white/10 hover:bg-white/20 p-2"
                >
                    <X />
                </button>
            </div>

            {#if error}
                <p class="text-red-400 text-sm">{error}</p>
            {/if}
            {#if success}
                <p class="text-green-400 text-sm">
                    Contraseña actualizada correctamente
                </p>
            {/if}

            <div class="flex items-center gap-2">
                <span>Actual:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="password"
                    bind:value={currentPassword}
                    disabled={saving}
                />
            </div>
            <div class="flex items-center gap-2">
                <span>Nueva:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="password"
                    bind:value={newPassword}
                    disabled={saving}
                />
            </div>
            <div class="flex items-center gap-2">
                <span>Confirmar:</span>
                <input
                    class="p-1 w-full bg-zinc-800 rounded-md"
                    type="password"
                    bind:value={confirmPassword}
                    disabled={saving}
                />
            </div>

            <div class="flex justify-end">
                <button
                    type="button"
                    onclick={handleSave}
                    disabled={saving}
                    class="flex items-center justify-items-center border rounded-lg bg-white/10 hover:bg-white/20 p-2 disabled:opacity-50"
                >
                    <span class="mr-2"
                        >{saving ? "Guardando..." : "Guardar"}</span
                    >
                    <Save />
                </button>
            </div>
        </div>
    </div>
{/if}
