//! This module provides a RustGraph type that can be
//! converted into a Graph type, but supports all of the rust tooling
//! especially serialization, deserialization, etc
//!
#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
use std::collections::HashMap;
use uuid::Uuid;
use std::fmt::Write;
use crate::style::{EdgeAttribute, Attribute, GraphAttr, NodeAttribute, CommonAttr};
/// a module for incrementally editing graphs using a domain specific structured query language.
//pub mod command;
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    id: Uuid,
    label: String, 
    attributes: Vec<NodeAttribute>,
}

impl Node {
    pub fn new<S: AsRef<str>>(id: Uuid, label: S) -> Self {
        Self {
            id,
            label: label.as_ref().to_string(),
            attributes: Vec::new(),
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\" [id=\"{}\", label=\"{}\"",
            self.id, self.id, self.label
        )?;
        for attr in &self.attributes {
            if let NodeAttribute::Common(CommonAttr::Id(_))
                | NodeAttribute::Common(CommonAttr::Label(_)) = attr
            {
                // skip duplicate id/label
                continue;
            }
            write!(f, ", {}", attr)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug)]
pub enum ParseNodeError {
    MissingId,
    MissingLabel,
    IdMismatch { expected: Uuid, found: Uuid },
    LabelMismatch { expected: String, found: String },
    ParseError(String),
}

impl std::fmt::Display for ParseNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingId => write!(f, "missing id attribute"),
            Self::MissingLabel => write!(f, "missing label attribute"),
            Self::IdMismatch { expected, found } =>
                write!(f, "id mismatch: expected '{}', found '{}'", expected, found),
            Self::LabelMismatch { expected, found } =>
                write!(f, "label mismatch: expected '{}', found '{}'", expected, found),
            Self::ParseError(e) => write!(f, "parse error: {}", e),
        }
    }
}
impl std::error::Error for ParseNodeError {}

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // expected: id [id="...", label="...", ...]
        let bracket_start = s.find('[').ok_or_else(|| ParseNodeError::ParseError("missing '['".into()))?;
        let bracket_end = s.find(']').ok_or_else(|| ParseNodeError::ParseError("missing ']'".into()))?;

        let id_str = s[..bracket_start].trim();
        let node_id = Uuid::parse_str(id_str)
            .map_err(|e| ParseNodeError::ParseError(format!("invalid uuid '{}': {}", id_str, e)))?;

        let inside = &s[bracket_start+1 .. bracket_end];

        let mut id_in_attr: Option<Uuid> = None;
        let mut label: Option<String> = None;
        let mut attributes = Vec::new();

        for part in inside.split(',') {
            let part = part.trim();
            if part.is_empty() { continue; }

            let eq_idx = part.find('=').ok_or_else(|| ParseNodeError::ParseError(format!("invalid attribute '{}'", part)))?;
            let key = part[..eq_idx].trim();
            let value = part[eq_idx+1..].trim().trim_matches('"');

            match key {
                "id" => {
                    let parsed_id = Uuid::parse_str(value)
                        .map_err(|e| ParseNodeError::ParseError(format!("invalid id uuid '{}': {}", value, e)))?;
                    id_in_attr = Some(parsed_id);
                }
                "label" => {
                    label = Some(value.to_string());
                }
                _ => {
                    let attr = NodeAttribute::from_str(part)
                        .map_err(|_| ParseNodeError::ParseError(format!("invalid attribute '{}'", part)))?;
                    attributes.push(attr);
                }
            }
        }

        let label = label.ok_or(ParseNodeError::MissingLabel)?;

        // Check id consistency
        if let Some(dup_id) = id_in_attr {
            if dup_id != node_id {
                return Err(ParseNodeError::IdMismatch { expected: node_id, found: dup_id });
            }
        }

        // check duplicate label in attributes
        for attr in &attributes {
            if let NodeAttribute::Common(CommonAttr::Label(dup_label)) = attr {
                if dup_label != &label {
                    return Err(ParseNodeError::LabelMismatch { expected: label.clone(), found: dup_label.clone() });
                }
            }
        }

        Ok(Node { id: node_id, label, attributes })
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Node::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    id: Uuid,
    label: String,
    source: Uuid,
    dest: Uuid,
    attributes: Vec<EdgeAttribute>,
}

impl Edge {
    pub fn new(id: Uuid, label: &str, source: Uuid, dest: Uuid) -> Self {
        Self {
            id,
            source,
            dest,
            label: label.to_string(),
            attributes: Vec::new(),
        }
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\" -> \"{}\" [id=\"{}\", label=\"{}\"",
            self.source, self.dest, self.id, self.label
        )?;
        for attr in &self.attributes {
            if let EdgeAttribute::Common(CommonAttr::Id(_))
                | EdgeAttribute::Common(CommonAttr::Label(_)) = attr
            {
                // skip id and label if somehow present
                continue;
            }
            write!(f, ", {}", attr)?;
        }
        write!(f, "]")
    }
}

use std::str::FromStr;

#[derive(Debug)]
pub enum ParseEdgeError {
    MissingId,
    MissingLabel,
    LabelMismatch { expected: String, found: String },
    IdMismatch { expected: Uuid, found: Uuid },
    ParseError(String),
}

impl std::fmt::Display for ParseEdgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingId => write!(f, "missing id attribute"),
            Self::MissingLabel => write!(f, "missing label attribute"),
            Self::LabelMismatch { expected, found } =>
                write!(f, "label mismatch: expected '{}', found '{}'", expected, found),
            Self::IdMismatch { expected, found } =>
                write!(f, "id mismatch: expected '{}', found '{}'", expected, found),
            Self::ParseError(e) => write!(f, "parse error: {}", e),
        }
    }
}

impl std::error::Error for ParseEdgeError {}

impl FromStr for Edge {
    type Err = ParseEdgeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Split source -> dest
        let arrow_split: Vec<&str> = s.splitn(2, "->").collect();
        if arrow_split.len() != 2 {
            return Err(ParseEdgeError::ParseError("missing '->'".into()));
        }

        let source_str = arrow_split[0].trim();
        let rest = arrow_split[1].trim();

        let bracket_start = rest.find('[').ok_or_else(|| ParseEdgeError::ParseError("missing '['".into()))?;
        let dest_str = rest[..bracket_start].trim();

        let bracket_end = rest.find(']').ok_or_else(|| ParseEdgeError::ParseError("missing ']'".into()))?;
        let inside = &rest[bracket_start+1 .. bracket_end];

        let source = Uuid::parse_str(source_str).map_err(|e| ParseEdgeError::ParseError(format!("invalid source uuid: {}", e)))?;
        let dest = Uuid::parse_str(dest_str).map_err(|e| ParseEdgeError::ParseError(format!("invalid dest uuid: {}", e)))?;

        let mut id: Option<Uuid> = None;
        let mut label: Option<String> = None;
        let mut attributes = Vec::new();

        for part in inside.split(',') {
            let part = part.trim();
            if part.is_empty() { continue; }

            let eq_idx = part.find('=').ok_or_else(|| ParseEdgeError::ParseError(format!("invalid attribute '{}'", part)))?;
            let key = part[..eq_idx].trim();
            let value = part[eq_idx+1..].trim().trim_matches('"');

            match key {
                "id" => {
                    let parsed_id = Uuid::parse_str(value)
                        .map_err(|e| ParseEdgeError::ParseError(format!("invalid id uuid: {}", e)))?;
                    if id.is_some() {
                        return Err(ParseEdgeError::ParseError("duplicate id".into()));
                    }
                    id = Some(parsed_id);
                }
                "label" => {
                    if label.is_some() {
                        return Err(ParseEdgeError::ParseError("duplicate label".into()));
                    }
                    label = Some(value.to_string());
                }
                _ => {
                    // parse as EdgeAttribute, fallback to CommonAttr
                    let attr = EdgeAttribute::from_str(part)
                        .map_err(|_| ParseEdgeError::ParseError(format!("invalid attribute '{}'", part)))?;
                    attributes.push(attr);
                }
            }
        }

        let id = id.ok_or(ParseEdgeError::MissingId)?;
        let label = label.ok_or(ParseEdgeError::MissingLabel)?;

        // check for duplicate id or label in attributes
        for attr in &attributes {
            if let EdgeAttribute::Common(CommonAttr::Id(dup_id_str)) = attr {
                let dup_id = Uuid::parse_str(dup_id_str)
                    .map_err(|e| ParseEdgeError::ParseError(format!("invalid id uuid in attribute: {}", e)))?;
                if dup_id != id {
                    return Err(ParseEdgeError::IdMismatch { expected: id, found: dup_id });
                }
            }
            if let EdgeAttribute::Common(CommonAttr::Label(dup_label)) = attr {
                if dup_label != &label {
                    return Err(ParseEdgeError::LabelMismatch { expected: label.clone(), found: dup_label.clone() });
                }
            }
        }

        Ok(Edge { source, dest, id, label, attributes })
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Edge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Edge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Edge::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[test]
fn test_edge_display_and_parse() {
    let id = Uuid::new_v4();
    let source = Uuid::new_v4();
    let dest = Uuid::new_v4();

    let edge = Edge {
        id,
        label: "my edge".into(),
        source,
        dest,
        attributes: vec![EdgeAttribute::Common(CommonAttr::FontSize(12.0))],
    };

    let s = edge.to_string();
    let parsed = s.parse::<Edge>().unwrap();
    assert_eq!(parsed.id, edge.id);
    assert_eq!(parsed.label, edge.label);
    assert_eq!(parsed.source, edge.source);
    assert_eq!(parsed.dest, edge.dest);
    assert_eq!(parsed.attributes, edge.attributes);
}

#[test]
fn test_node_display_and_parse() {
    let id = Uuid::new_v4();
    let node = Node {
        id,
        label: "My Node".into(),
        attributes: vec![
            NodeAttribute::Common(CommonAttr::FontSize(10.0)),
        ],
    };

    let s = node.to_string();
    let parsed = s.parse::<Node>().unwrap();
    assert_eq!(parsed.id, node.id);
    assert_eq!(parsed.label, node.label);
    assert_eq!(parsed.attributes, node.attributes);
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustGraph {
    name: String,
    nodes: HashMap<Uuid, Node>,
    edges: HashMap<Uuid, Edge>,
    clusters: HashMap<String, RustGraph>,
    attributes: Vec<GraphAttr>,
}

impl RustGraph {
    /// Create a new RustGraph with the given name
    pub fn new(name: String) -> Self {
        Self {
            name,
            nodes: HashMap::new(),
            edges: HashMap::new(),
            attributes: vec![],
            clusters: HashMap::new(),
        }
    }

    /// Add a new edge to the graph with the given id, label, source, and destination node IDs
    pub fn add_edge(&mut self, id: Uuid, label: &str, source: Uuid, dest: Uuid) {
        let edge = Edge::new(id, label, source, dest);
        self.edges.insert(id, edge);
    }

    /// Add a new node to the graph with the given id and label
    pub fn add_node(&mut self, id: Uuid, label: &str) {
        let node = Node::new(id, label);
        self.nodes.insert(id, node);
    }

    /// Add an attribute to an existing node
    pub fn add_node_attr<A: Attribute + Into<NodeAttribute>>(&mut self, node_id: Uuid, attr: A) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.attributes.push(attr.into());
        } else {
            eprintln!("Node with id {:?} not found", node_id);
        }
    }

    /// Add an attribute to an existing edge
    pub fn add_edge_attr<A: Attribute + Into<EdgeAttribute>>(&mut self, edge_id: Uuid, attr: A) {
        if let Some(edge) = self.edges.get_mut(&edge_id) {
            edge.attributes.push(attr.into());
        } else {
            eprintln!("Edge with id {:?} not found", edge_id);
        }
    }

    /// Add a graph-level attribute
    pub fn add_graph_attr<A: Attribute + Into<GraphAttr>>(&mut self, attr: A) {
        self.attributes.push(attr.into());
    }

    /// Creates a new cluster subgraph within this graph.
    ///
    /// # Parameters
    /// - `graph`: The graph structure that defines the cluster.
    ///
    /// # Returns
    /// Returns a String representing the new cluster's name cluster_{graph.name}.
    ///
    /// # Errors
    /// Returns `Err` if the cluster could not be created (e.g., due to invalid name or allocation failure).
    pub fn add_cluster(&mut self, graph: RustGraph) -> Result<String, String> {
        // Prepend "cluster_" to comply with Graphviz convention
        let cluster_name = format!("cluster_{}", graph.name);
        self.clusters.insert(cluster_name.clone(), graph);
        // Wrap in new Graph struct
        Ok(cluster_name)
    }

    /// converts the RustGraph to graphviz dot structure with appropriate attributes.
    /// please note this impl currently doesn't add cluster's that will be imnplemented later.
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph G {\n");

        for (_id, node) in &self.nodes {
            writeln!(&mut dot, "    {}", node).unwrap();
        }

        for (_id, edge) in &self.edges {
            writeln!(&mut dot, "    {}", edge).unwrap();
        }

        dot.push_str("}\n");
        dot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let graph = RustGraph { nodes: vec![], edges: vec![] };
        let dot = graph.to_dot();

        assert!(dot.starts_with("digraph G {"));
        assert!(dot.ends_with("}\n"));
        assert!(!dot.contains("["));
    }

    #[test]
    fn test_single_node() {
        let node = Node { id: "A".into() };
        let graph = RustGraph { nodes: vec![node], edges: vec![] };
        let dot = graph.to_dot();

        assert!(dot.contains("A [label=\"A\"]"));
        assert!(dot.starts_with("digraph G {"));
        assert!(dot.ends_with("}\n"));
    }

    #[test]
    fn test_single_edge() {
        let node_a = Node { id: "A".into() };
        let node_b = Node { id: "B".into() };
        let edge = Edge { source: "A".into(), target: "B".into() };
        let graph = RustGraph { nodes: vec![node_a, node_b], edges: vec![edge] };

        let dot = graph.to_dot();

        assert!(dot.contains("A [label=\"A\"]"));
        assert!(dot.contains("B [label=\"B\"]"));
        assert!(dot.contains("A -> B [weight=1]"));
    }

}