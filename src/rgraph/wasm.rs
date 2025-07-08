use wasm_bindgen::prelude::*;
use std::convert::TryFrom;
use std::collections::HashMap;
use uuid::Uuid;
use crate::rgraph::{RustGraph};
use crate::style::{EdgeAttribute, Attribute, GraphAttr, NodeAttribute, CommonAttr};
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;

#[wasm_bindgen]
pub struct WasmRustGraph {
    inner: RustGraph,
}


#[wasm_bindgen]
pub struct WasmAttribute {
    key: String,
    value: String,
}

#[wasm_bindgen]
impl WasmAttribute {
    #[wasm_bindgen(constructor)]
    pub fn new(key: &str, value: &str) -> WasmAttribute {
        WasmAttribute {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

// Conversion impls to your enums using existing TryFrom<(&str, &str)>

impl TryFrom<WasmAttribute> for EdgeAttribute {
    type Error = &'static str;

    fn try_from(attr: WasmAttribute) -> Result<Self, Self::Error> {
        EdgeAttribute::try_from((attr.key.as_str(), attr.value.as_str()))
    }
}

impl TryFrom<WasmAttribute> for NodeAttribute {
    type Error = &'static str;

    fn try_from(attr: WasmAttribute) -> Result<Self, Self::Error> {
        NodeAttribute::try_from((attr.key.as_str(), attr.value.as_str()))
    }
}

impl TryFrom<WasmAttribute> for GraphAttr {
    type Error = &'static str;

    fn try_from(attr: WasmAttribute) -> Result<Self, Self::Error> {
        GraphAttr::try_from((attr.key.as_str(), attr.value.as_str()))
    }
}


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WasmRustGraph {
    /// Create a new graph with the given name.
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> WasmRustGraph {
        WasmRustGraph {
            inner: RustGraph::new(name.to_string()),
        }
    }

    /// Add a node by id (UUID string) and label.
    #[wasm_bindgen]
    pub fn add_node(&mut self, id: &str, label: &str) {
        let uuid = Uuid::parse_str(id).expect("Invalid UUID");
        self.inner.add_node(uuid, label);
    }

    /// Add an edge by id, label, source UUID and destination UUID.
    #[wasm_bindgen]
    pub fn add_edge(&mut self, id: &str, label: &str, source: &str, dest: &str) {
        let uuid = Uuid::parse_str(id).expect("Invalid UUID");
        let source_uuid = Uuid::parse_str(source).expect("Invalid source UUID");
        let dest_uuid = Uuid::parse_str(dest).expect("Invalid dest UUID");
        self.inner.add_edge(uuid, label, source_uuid, dest_uuid);
    }

    /// Add an attribute to a node by UUID string.
    #[wasm_bindgen]
    pub fn add_node_attr(&mut self, node_id: &str, attr_json: &JsValue) {
        let uuid = Uuid::parse_str(node_id).expect("Invalid UUID");
        let attr: NodeAttribute = serde_wasm_bindgen::from_value(attr_json.clone())
            .expect("Failed to parse NodeAttribute from JS");
        self.inner.add_node_attr(uuid, attr);
    }

    /// Add an attribute to an edge by UUID string.
    #[wasm_bindgen]
    pub fn add_edge_attr(&mut self, edge_id: &str, attr_json: &JsValue) {
        let uuid = Uuid::parse_str(edge_id).expect("Invalid UUID");
        let attr: EdgeAttribute = serde_wasm_bindgen::from_value(attr_json.clone())
            .expect("Failed to parse EdgeAttribute from JS");
        self.inner.add_edge_attr(uuid, attr);
    }

    /// Add a graph-level attribute.
    #[wasm_bindgen]
    pub fn add_graph_attr(&mut self, attr_json: &JsValue) {
        let attr: GraphAttr = serde_wasm_bindgen::from_value(attr_json.clone())
            .expect("Failed to parse GraphAttr from JS");
        self.inner.add_graph_attr(attr);
    }
}

