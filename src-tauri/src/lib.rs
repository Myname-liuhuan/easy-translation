mod commands;
mod language_detect;
mod translator;

use commands::translate::translate_text;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![translate_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
