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
    class="btn-ghost w-full justify-start"
>
    <KeyRound size={16} />
    <span>Cambiar contraseña</span>
</button>
{#if isOpen}
    <div class="modal-backdrop">
        <div class="modal-panel max-w-md">
            <div
                class="flex items-center justify-between border-b border-zinc-800 pb-3"
            >
                <h3 class="text-lg font-semibold text-zinc-50">
                    Cambiar contraseña
                </h3>
                <button
                    type="button"
                    onclick={menu}
                    class="btn-icon"
                    aria-label="Cerrar"
                >
                    <X size={16} />
                </button>
            </div>

            {#if error}
                <p
                    class="rounded-lg border border-red-500/20 bg-red-500/10 px-3 py-2 text-sm text-red-300"
                >
                    {error}
                </p>
            {/if}
            {#if success}
                <p
                    class="rounded-lg border border-emerald-500/20 bg-emerald-500/10 px-3 py-2 text-sm text-emerald-300"
                >
                    Contraseña actualizada correctamente
                </p>
            {/if}

            <div class="grid gap-3">
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Contraseña actual</span>
                    <input
                        class="field w-full"
                        type="password"
                        bind:value={currentPassword}
                        disabled={saving}
                    />
                </label>
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Nueva contraseña</span>
                    <input
                        class="field w-full"
                        type="password"
                        bind:value={newPassword}
                        disabled={saving}
                    />
                </label>
                <label class="flex flex-col gap-1.5">
                    <span class="label-muted">Confirmar nueva</span>
                    <input
                        class="field w-full"
                        type="password"
                        bind:value={confirmPassword}
                        disabled={saving}
                    />
                </label>
            </div>

            <div class="flex justify-end gap-2 border-t border-zinc-800 pt-3">
                <button
                    type="button"
                    onclick={menu}
                    class="btn"
                    disabled={saving}
                >
                    Cancelar
                </button>
                <button
                    type="button"
                    onclick={handleSave}
                    disabled={saving}
                    class="btn-primary"
                >
                    <Save size={16} />
                    <span>{saving ? "Guardando…" : "Guardar"}</span>
                </button>
            </div>
        </div>
    </div>
{/if}
