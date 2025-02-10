use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde_json::json;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

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
    value: i32,
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
                        value: 5,
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
                        value: 10,
                    });
                }
            }
        }
    }

    json!({
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

fn scan_markdown_files(dir: &Path) -> io::Result<Vec<MarkdownMeta>> {
    let mut metadata_list = Vec::new();

    if !dir.is_dir() {
        return Ok(metadata_list);
    }

    let matter = Matter::<YAML>::new();

    for entry in fs::read_dir(dir)? {
        if metadata_list.len() >= 50 {
            break;
        }

        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            metadata_list.extend(scan_markdown_files(&path)?);
            continue;
        }

        if path.extension().map_or(true, |ext| ext != "md") {
            continue;
        }

        let contents = fs::read_to_string(&path)?;
        let result = matter.parse(&contents);
        let front_matter: FrontMatter = result.data.and_then(|x| x.deserialize().ok()).unwrap_or_default();

        metadata_list.push(MarkdownMeta {
            title: create_title(&path, front_matter.title),
            file_path: path.to_string_lossy().to_string(),
            tags: extract_tags(&front_matter.tags),
            ignore: front_matter.ignore,
        });
    }

    Ok(metadata_list)
}

pub fn analyze(dir: &Path) -> io::Result<String> {
    scan_markdown_files(dir).map(|result| {
        let graph_json = build_graph(result);
        let json = serde_json::to_string_pretty(&graph_json).unwrap();
        println!("Analysis complete!");
        json
    })
}
