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
}

#[derive(serde::Deserialize, Debug, Default)]
struct FrontMatter {
    title: Option<String>,
    tags: Option<String>,
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
    let mut tags = HashSet::new();

    for file in &metadata_list {
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

            tags.insert(tag);
        }
    }

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

fn parse_tags(tags: Option<String>) -> Vec<String> {
    tags.map(|tags| {
        tags.split(' ')
            .map(|s| s.trim().to_lowercase().to_string())
            .filter(|x| !x.is_empty())
            .collect()
    })
    .unwrap_or_else(Vec::new)
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

        if path.extension().map_or(false, |ext| ext != "md") {
            continue;
        }

        let contents = fs::read_to_string(&path)?;
        let result = matter.parse(&contents);
        let front_matter: FrontMatter = result.data.and_then(|x| x.deserialize().ok()).unwrap_or_default();

        if front_matter.ignore {
            continue;
        }

        metadata_list.push(MarkdownMeta {
            title: create_title(&path, front_matter.title),
            file_path: path.to_string_lossy().to_string(),
            tags: parse_tags(front_matter.tags),
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
