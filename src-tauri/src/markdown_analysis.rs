use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde_json::json;
use std::collections::{HashMap, HashSet};
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

/// Resolves all relative tags to their absolute versions based on known overlaps
fn resolve_tag_paths(tags: &HashSet<String>) -> HashMap<String, String> {
    let mut resolved_tags = HashMap::new();

    // Step 1: Sort tags by length to ensure longest known paths are processed first
    let mut sorted_tags: Vec<&String> = tags.iter().collect();
    sorted_tags.sort_by_key(|t| t.len());

    for tag in &sorted_tags {
        let tag_parts: Vec<&str> = tag.split('/').filter(|s| !s.is_empty()).collect();
        let mut best_match = None;
        let mut best_match_path = String::new();

        // Step 2: Find the longest existing absolute prefix
        for i in 1..=tag_parts.len() {
            let potential_path = format!("/{}", tag_parts[..i].join("/"));

            if resolved_tags.contains_key(&potential_path) {
                best_match = Some(potential_path.clone());
            }
        }

        // Step 3: If we found a valid prefix, extend it
        if let Some(prefix) = best_match {
            best_match_path = prefix;
            if !best_match_path.ends_with('/') {
                best_match_path.push('/');
            }
        }

        // Step 4: Store the resolved absolute path
        let final_path = format!("{}{}", best_match_path, tag_parts.join("/"));
        resolved_tags.insert(tag.to_string(), final_path);
    }

    resolved_tags
}

fn build_graph(metadata_list: Vec<MarkdownMeta>) -> serde_json::Value {
    let mut nodes = HashSet::new();
    let mut links = HashSet::new();
    let mut tags = HashSet::new();

    for file in &metadata_list {
        // we want the tags also from "to be ignored files" - those are "meta files"
        for tag in &file.tags {
            tags.insert(tag.clone());
        }

        if file.ignore {
            continue;
        }

        nodes.insert(Node {
            id: file.file_path.clone(),
            title: file.title.clone(),
            kind: "file".to_string(),
        });

        for tag in &file.tags {
            links.insert(Link {
                source: file.file_path.clone(),
                target: tag.clone(),
                value: 2,
            });
        }
    }

    let resolved = resolve_tag_paths(&tags);
    println!("Resolved tags: {:?}", resolved);

    for tag in &tags {
        let parts: Vec<&str> = tag.split('/').collect();
        let mut path = String::new();
        let mut parent: Option<String> = None;

        for part in parts {
            if !path.is_empty() {
                path.push('/');
            }
            path.push_str(part);

            nodes.insert(Node {
                id: path.clone(),
                title: part.to_string().clone(),
                kind: "topic".to_string(),
            });

            if let Some(parent_id) = parent {
                links.insert(Link {
                    source: parent_id.clone(),
                    target: path.clone(),
                    value: 5,
                });
            }

            parent = Some(path.clone());
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
