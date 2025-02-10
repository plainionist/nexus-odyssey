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

    for file in &metadata_list {
        nodes.insert(Node {
            id: file.file_path.clone(),
            title: file.title.clone(),
            kind: "file".to_string(),
        });

        for tag in &file.tags {
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

            if let Some(topic_id) = parent {
                links.insert(Link {
                    source: file.file_path.clone(),
                    target: topic_id,
                    value: 2,
                });
            }
        }
    }

    json!({
        "nodes": nodes.into_iter().collect::<Vec<_>>(),
        "links": links.into_iter().collect::<Vec<_>>()
    })
}

// TODO: tags contain relative paths - first build entire tree of topics
fn scan_markdown_files(dir: &Path) -> io::Result<Vec<MarkdownMeta>> {
    let mut metadata_list = Vec::new();

    let matter = Matter::<YAML>::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            if metadata_list.len() >= 50 {
                break;
            }

            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let nested_files = scan_markdown_files(&path)?;
                metadata_list.extend(nested_files);
            } else if path.extension().map_or(false, |ext| ext == "md") {
                if let Ok(contents) = fs::read_to_string(&path) {
                    let result = matter.parse(&contents);
                    let front_matter: FrontMatter = result.data.and_then(|x| x.deserialize().ok()).unwrap_or_default();

                    if front_matter.ignore {
                        continue;
                    }

                    let file_name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Unnamed")
                        .to_string();

                    metadata_list.push(MarkdownMeta {
                        title: front_matter.title.unwrap_or_else(|| file_name),
                        file_path: path.to_string_lossy().to_string(),
                        tags: front_matter
                            .tags
                            .map(|tags| {
                                tags.split(' ')
                                    .map(|s| s.trim().to_string())
                                    .filter(|x| !x.is_empty())
                                    .collect()
                            })
                            .unwrap_or_else(Vec::new),
                    });
                }
            }
        }
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
