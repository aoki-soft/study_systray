use tauri::{SystemTray, SystemTrayEvent};
use tauri::Manager;

fn toggle_window_visible(window: &tauri::Window) {
  match window.is_visible() {
    Ok(visible) => {
      if visible {
        window.hide().unwrap();
      } else {
        window.show().unwrap();
        window.set_focus().unwrap();
      }
    }
    Err(err) => {
      panic!("failed toggle visible for main window {}", err);
    }
  }
}

fn main() {
  let system_tray = SystemTray::new();

  tauri::Builder::default()
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick {
        ..
      } => {
        let window = app.get_window("main").unwrap();
        toggle_window_visible(&window);
      }
      _ => {}
    })
    .system_tray(system_tray)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}