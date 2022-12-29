#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod index;
mod about;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            count,
            index::commands::emit_sysinfo,
            about::commands::open_about,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn count(time: isize) -> String {
    format!("Count is {}", time)
}
