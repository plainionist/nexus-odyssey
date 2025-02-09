use gray_matter::engine::YAML;
use gray_matter::Matter;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, serde::Serialize)]
struct MarkdownMeta {
    title: String,
    file_path: String,
    tags: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
struct FrontMatter {
    title: String,
    tags: String,
}

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
                    println!("Parsed front matter: {:?}", result.data);
                    let front_matter: Option<FrontMatter> = result.data.and_then(|x| x.deserialize().ok());

                    let file_name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Unnamed")
                        .to_string();

                    metadata_list.push(MarkdownMeta {
                        title: front_matter.as_ref().map_or_else(|| file_name, |x| x.title.clone()),
                        file_path: path.to_string_lossy().to_string(),
                        tags: front_matter.map_or_else(Vec::new, |x| {
                            x.tags
                                .split(' ')
                                .map(|s| s.trim().to_string())
                                .filter(|x| !x.is_empty())
                                .collect()
                        }),
                    });
                }
            }
        }
    }

    Ok(metadata_list)
}

pub fn analyze(dir: &Path) -> io::Result<String> {
    scan_markdown_files(dir).map(|result| {
        let json = serde_json::to_string_pretty(&result).unwrap();
        println!("Analysis complete:\n{}", json);
        json
    })
}
