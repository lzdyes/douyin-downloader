#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod command;
mod menu;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      command::show,
      command::exists,
      command::download
    ])
    .menu(menu::generate_menu())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
