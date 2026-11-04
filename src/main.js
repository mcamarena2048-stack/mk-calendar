import { invoke } from '@tauri-apps/api/core';

window.addEventListener("DOMContentLoaded", () => {
    console.log(">>> DOM completamente cargado");

    const toggleBtn = document.getElementById('toggle-btn');
    console.log(">>> Buscando botón #toggle-btn:", toggleBtn);

    if (toggleBtn) {
        toggleBtn.addEventListener('click', async () => {
            console.log(">>> ¡Clic detectado en el botón! Llamando a Rust...");
            try {
                await invoke('toggle_decorations');
                console.log(">>> ¡Comando ejecutado con éxito en Rust!");
            } catch (error) {
                console.error(">>> Error devuelto por Rust:", error);
            }
        });
    } else {
        console.error(">>> ¡ERROR CRÍTICO! No se encontró ningún elemento con id 'toggle-btn' en el HTML.");
    }
});