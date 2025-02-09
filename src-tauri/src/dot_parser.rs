use std::path::Path;
use std::io::{Error, ErrorKind};
use serde::{Serialize, Deserialize};
use serde_json;
use dot_parser::*;

#[derive(Serialize, Deserialize)]
struct SerializableGraph {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

impl From<canonical::Graph<(String, String)>> for SerializableGraph {
    fn from(graph: canonical::Graph<(String, String)>) -> Self {
        Self {
            nodes: graph.nodes.set.iter().map(|(id,_)| id.clone()).collect(),
            edges: graph.edges.set.iter().map(|x| (x.from.clone(), x.to.clone())).collect(),
        }
    }
}

fn parse_dot_to_json_core(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
  let graph = ast::Graph::from_file(path)?;
  let graph = canonical::Graph::from(graph);
  let serializable_graph: SerializableGraph = graph.into();
  let json = serde_json::to_string_pretty(&serializable_graph)?;

  Ok(json)
}

pub fn parse_dot_to_json(path: &Path) -> std::io::Result<String> {
  parse_dot_to_json_core(path).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
}
