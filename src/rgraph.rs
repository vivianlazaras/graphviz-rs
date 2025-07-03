//! This module provides a RustGraph type that can be
//! converted into a Graph type, but supports all of the rust tooling
//! especially serialization, deserialization, etc
//! 

use std::collections::HashMap;
use uuid::Uuid;
use crate::style::{EdgeAttribute, NodeAttribute};
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Node {
    id: Uuid,
    label: String, 
    attributes: Vec<NodeAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Edge {
    id: Uuid,
    label: String,
    source: Uuid,
    dest: Uuid,
    attributes: Vec<EdgeAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct RustGraph {
    name: String,
    nodes: HashMap<Uuid, Node>,
    edges: HashMap<Uuid, Edge>,
}

