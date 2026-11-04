// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn toggle_decorations(window: tauri::WebviewWindow) {
    match window.is_decorated() {
        Ok(decorated) => {
            let new_state = !decorated;
            if let Err(e) = window.set_decorations(new_state) {
                eprintln!("Error al cambiar decoraciones: {}", e);
            } else {
                // Truco para forzar el redibujado del marco en Windows
                #[cfg(target_os = "windows")]
                {
                    if let Ok(size) = window.inner_size() {
                        let _ = window.set_size(tauri::LogicalSize::new(size.width as f64 + 1.0, size.height as f64));
                        let _ = window.set_size(size);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error al obtener estado de decoración: {}", e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, toggle_decorations])
        .setup(|app| {
            let win = app.get_webview_window("main").unwrap();
            let _ = win.set_always_on_bottom(false);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}