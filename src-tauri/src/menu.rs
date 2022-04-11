use tauri::Menu;

#[cfg(target_os = "macos")]
use tauri::{MenuItem, Submenu};

pub fn generate_menu() -> Menu {
  #[cfg(target_os = "macos")]
  {
    let app_menu = Submenu::new(
      "App",
      Menu::new()
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit),
    );

    let edit_menu = Submenu::new(
      "Edit",
      Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::SelectAll),
    );

    Menu::new().add_submenu(app_menu).add_submenu(edit_menu)
  }

  #[cfg(not(target_os = "macos"))]
  {
    Menu::new()
  }
}
