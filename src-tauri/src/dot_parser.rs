use std::path::Path;
use std::io::{Error, ErrorKind};
use serde::{Serialize, Deserialize};
use serde_json;
use dot_parser::*;

#[derive(Serialize, Deserialize)]
struct SerializableGraph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

#[derive(Serialize, Deserialize)]
struct Node {
    id: String,
    group: i32, // Defaulting to `i32`, adjust if needed
}

#[derive(Serialize, Deserialize)]
struct Link {
    source: String,
    target: String,
    value: i32, // Represents edge weight
}

fn parse(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
  let graph = ast::Graph::from_file(path)?;
  let graph = canonical::Graph::from(graph);

  let nodes: Vec<Node> = graph
      .nodes
      .set
      .iter()
      .map(|(id, _)| Node {
          id: id.clone(),
          group: 1,
      })
      .collect();

  let links: Vec<Link> = graph
      .edges
      .set
      .iter()
      .map(|edge| Link {
          source: edge.from.clone(),
          target: edge.to.clone(),
          value: 1,
      })
      .collect();

  let serializable_graph = SerializableGraph { nodes, links };

  let json = serde_json::to_string_pretty(&serializable_graph)?;

  Ok(json)
}

pub fn parse_dot_to_json(path: &Path) -> std::io::Result<String> {
  parse(path).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
}
