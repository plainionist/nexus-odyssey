use rust_bert::pipelines::pos_tagging::{POSConfig, POSModel, POSTag};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::LazyLock;

static NOUN_TAGS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| ["NN", "NNS", "NNP", "NNPS"].iter().cloned().collect());

fn scan_markdown_files(dir: &Path, model: &POSModel) -> io::Result<HashMap<String, Vec<String>>> {
    let mut file_word_map = HashMap::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            if file_word_map.len() >= 50 {
                break;
            }

            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let nested_files = scan_markdown_files(&path, model)?;
                file_word_map.extend(nested_files);
            } else if path.extension().map_or(false, |ext| ext == "md") {
                if let Ok(contents) = fs::read_to_string(&path) {
                    let predictions = model.predict(&[contents]);

                    let proper_nouns: Vec<String> = predictions
                        .into_iter()
                        .flatten()
                        .filter_map(|token: POSTag| {
                            if NOUN_TAGS.contains(token.label.as_str()) {
                                Some(token.word)
                            } else {
                                None
                            }
                        })
                        .collect();

                    file_word_map.insert(path.to_string_lossy().to_string(), proper_nouns);
                }
            }
        }
    }

    Ok(file_word_map)
}

pub fn analyze(dir: &Path) -> io::Result<String> {
    let config = POSConfig::default();
    let model = POSModel::new(config).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    scan_markdown_files(dir, &model).map(|result| serde_json::to_string(&result).unwrap())
}
