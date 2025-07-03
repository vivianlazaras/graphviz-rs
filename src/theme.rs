
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct GraphTheme {
    pub name: String,
    pub layout: crate::Layout,
    /// these attributes are added to all nodes
    pub node_attrs: Vec<crate::NodeAttr>,
    /// these attributes are added to all edges
    pub edge_attrs: Vec<crate::EdgeAttr>,
}

impl Default for GraphTheme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            layout: crate::Layout::Dot,
            node_attrs: Vec::new(),
            edge_attrs: Vec::new(),
        }
    }
}