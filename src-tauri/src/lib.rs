use std::fs::read_to_string;
use tauri::menu::*;
use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("open", "Open")
                .separator()
                .quit()
                .build()?;

            let menu = MenuBuilder::new(app).item(&file_menu).build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| {
                if event.id() == "open" {
                    if let Some(file_path) = app.dialog().file().add_filter("JSON", &["json"]).blocking_pick_file() {
                        match read_to_string(file_path.as_path().unwrap()) {
                            Ok(content) => {
                                app.emit("load:json", content).expect("Emit 'load:json' failed");
                            }
                            Err(err) => {
                                eprintln!("Failed to read file: {}", err);
                            }
                        }
                    }
                }
            });

            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
