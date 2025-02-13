use gray_matter::engine::YAML;
use gray_matter::Matter;
use ignore::{gitignore::GitignoreBuilder, WalkBuilder};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Config {
    ignore: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
struct MarkdownMeta {
    title: String,
    file_path: String,
    tags: Vec<String>,
    ignore: bool,
}

#[derive(serde::Deserialize, Debug, Default)]
struct FrontMatter {
    title: Option<String>,
    tags: Option<serde_yaml::Value>,
    #[serde(default)]
    ignore: bool,
}

#[derive(Debug, serde::Serialize, Eq, PartialEq, Hash)]
struct Node {
    id: String,
    title: String,
    kind: String,
}

#[derive(Debug, serde::Serialize, Eq, PartialEq, Hash)]
struct Link {
    source: String,
    target: String,
}

fn load_config(config_path: &Path) -> io::Result<Config> {
    let config_data = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_data)?;
    Ok(config)
}

fn build_graph(metadata_list: Vec<MarkdownMeta>) -> serde_json::Value {
    let mut nodes = HashSet::new();
    let mut links = HashSet::new();

    for file in &metadata_list {
        if !file.ignore {
            nodes.insert(Node {
                id: file.file_path.clone(),
                title: file.title.clone(),
                kind: "file".to_string(),
            });
        }

        for tag in &file.tags {
            let parts: Vec<&str> = tag.split('/').collect();
            let mut parent: Option<&str> = None;

            for &part in &parts {
                nodes.insert(Node {
                    id: part.to_string(),
                    title: part.to_string(),
                    kind: "topic".to_string(),
                });

                if let Some(parent_id) = parent {
                    links.insert(Link {
                        source: parent_id.to_string(),
                        target: part.to_string(),
                    });
                }

                parent = Some(part);
            }

            if !file.ignore {
                // parent is now the last part of the tag
                if let Some(last_part) = parent {
                    links.insert(Link {
                        source: file.file_path.clone(),
                        target: last_part.to_string(),
                    });
                }
            }
        }
    }

    json!({
        "meta": {
            "semantics": "markdown",
        },
        "nodes": nodes.into_iter().collect::<Vec<_>>(),
        "links": links.into_iter().collect::<Vec<_>>()
    })
}

fn extract_tags(tags_value: &Option<serde_yaml::Value>) -> Vec<String> {
    match tags_value {
        Some(serde_yaml::Value::String(tags_str)) => tags_str
            .split_whitespace()
            .map(|s| s.trim().to_lowercase().to_string())
            .filter(|x| !x.is_empty())
            .collect(),
        Some(serde_yaml::Value::Sequence(seq)) => seq
            .iter()
            .filter_map(|val| val.as_str().map(|s| s.trim().to_string()))
            .collect(),
        _ => vec![],
    }
}

fn create_title(path: &Path, front_matter_title: Option<String>) -> String {
    front_matter_title.unwrap_or_else(|| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unnamed>")
            .to_string()
    })
}

fn scan_markdown_files(dir: &Path, ignore_patterns: &[String]) -> io::Result<Vec<MarkdownMeta>> {
    let mut files = Vec::new();

    let mut gitignore_builder = GitignoreBuilder::new(dir);
    for pattern in ignore_patterns {
        gitignore_builder.add_line(Some(dir.to_path_buf()), pattern).unwrap();
    }

    let gitignore = gitignore_builder.build().unwrap();

    for result in WalkBuilder::new(dir).git_ignore(false).build() {
        match result {
            Ok(entry) => {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    if !gitignore.matched(path, false).is_ignore() {
                        files.push(path.to_path_buf());
                    } else {
                        println!("Ignoring: {}", path.display());
                    }
                }
            }
            Err(err) => eprintln!("Error reading directory: {}", err),
        }
    }

    let mut metadata_list = Vec::new();

    let matter = Matter::<YAML>::new();

    for path in &files {
        println!("Analyzing: {}", path.display());

        match fs::read_to_string(&path) {
            Ok(contents) => {
                let result = matter.parse(&contents);
                let front_matter: FrontMatter = result.data.and_then(|x| x.deserialize().ok()).unwrap_or_default();

                metadata_list.push(MarkdownMeta {
                    title: create_title(&path, front_matter.title),
                    file_path: path.to_string_lossy().to_string(),
                    tags: extract_tags(&front_matter.tags),
                    ignore: front_matter.ignore,
                });
            }
            Err(e) => {
                eprintln!("Skipping file {}: {}", path.display(), e);
            }
        }
    }

    Ok(metadata_list)
}

pub fn analyze(dir: &Path) -> io::Result<String> {
    let config_path = dir.join("nexus-odyssey.json");
    let config = load_config(&config_path).unwrap_or_else(|_| Config { ignore: vec![] });

    scan_markdown_files(dir, &config.ignore).map(|result| {
        let graph_json = build_graph(result);
        let json = serde_json::to_string_pretty(&graph_json).unwrap();
        println!("Analysis complete!");
        json
    })
}
