use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn scan_markdown_files(dir: &Path) -> io::Result<HashMap<String, String>> {
    let mut file_word_map = HashMap::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            if file_word_map.len() >= 50 {
                break;
            }
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let nested_files = scan_markdown_files(&path)?;
                file_word_map.extend(nested_files);
            } else if path.extension().map_or(false, |ext| ext == "md") {
                if let Ok(contents) = fs::read_to_string(&path) {
                    let words: Vec<&str> = contents.split_whitespace().collect();
                    file_word_map.insert(path.to_string_lossy().to_string(), words.join(" "));
                }
            }
        }
    }

    Ok(file_word_map)
}

pub fn analyze(dir: &Path) -> std::io::Result<String> {
    scan_markdown_files(dir).map(|_| "{}".to_string())
}
