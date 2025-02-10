mod dot_parser;
mod markdown_analysis;

use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;
use tauri::command;
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

#[command]
fn open_in_vscode(file_path: String) {
    let cmd = if cfg!(target_os = "windows") {
        "code.cmd"
    } else {
        "code"
    };
    let output = Command::new(cmd).arg(file_path.clone()).spawn();

    match output {
        Ok(_) => println!("Opened file in VS Code: {}", file_path),
        Err(e) => println!("Failed to open file in VS Code: {}", e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_in_vscode])
        .setup(|app| {
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("open", "Open ...")
                .text("analyze-markdown", "Analyze MarkDown ...")
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
                        .add_filter("Json|Dot", &["dot", "json"])
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
                } else if event.id() == "analyze-markdown" {
                    if let Some(folder_path) = app.dialog().file().blocking_pick_folder() {
                        if let Some(path) = folder_path.as_path() {
                            match markdown_analysis::analyze(path) {
                                Ok(content) => {
                                    if let Err(err) = app.emit("load:json", &content) {
                                        eprintln!("Failed to emit json: {}", err);
                                    }

                                    // store as kind of cache
                                    let cache_file = path.join("analysis.json");
                                    if let Err(err) = std::fs::write(cache_file, &content) {
                                        eprintln!("Failed to write cache file: {}", err);
                                    }
                                }
                                Err(err) => eprintln!("Failed to analyze folder: {}", err),
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
