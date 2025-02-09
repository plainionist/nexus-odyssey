mod dot_parser;

use std::path::Path;
use std::io::{Error, ErrorKind};
use tauri::menu::*;
use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

fn read_file(path: &Path) -> std::io::Result<String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => std::fs::read_to_string(path),
        Some("dot") => dot_parser::parse_dot_to_json(path),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Unsupported file type")),
    }
}

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
                    if let Some(file_path) = app
                        .dialog()
                        .file()
                        .add_filter("DOT", &["dot"])
                        .add_filter("JSON", &["json"])
                        .blocking_pick_file()
                    {
                        if let Err(err) = file_path
                            .as_path()
                            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Invalid file path"))
                            .and_then(read_file)
                            .and_then(|content| {
                                app.emit("load:json", content)
                                    .map_err(|e| Error::new(ErrorKind::Other, e))
                            })
                        {
                            eprintln!("Failed to read and emit JSON file: {}", err);
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
