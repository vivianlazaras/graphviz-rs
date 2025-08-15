use crate::sys::*;
use std::ffi::{CStr, CString, c_char};
use crate::{Layout, OutputFormat};
use crate::style::{Attribute, NodeAttr, EdgeAttr};

use std::collections::HashMap;
pub struct EdgeBuilder<'e, 'c, S: AsRef<str>> {
    edge_name: S,
    graph: &'e mut Graph<'c>,
}

impl<'e, 'c, S: AsRef<str>> EdgeBuilder<'e, 'c, S> {
    pub fn set(&mut self, attr: EdgeAttr) -> Result<&mut Self, String> {
        self.graph
            .set_attr_on_edge(&self.edge_name.as_ref(), attr)?;
        Ok(self)
    }
}

pub struct NodeBuilder<'n, 'c, S: AsRef<str>> {
    node_name: S,
    graph: &'n mut Graph<'c>,
}

impl<'n, 'c, S: AsRef<str>> NodeBuilder<'n, 'c, S> {
    pub fn set(&mut self, attr: NodeAttr) -> Result<&mut Self, String> {
        self.graph.set_attr_on_node(self.node_name.as_ref(), attr)?;
        Ok(self)
    }
}

/// an idiomatic wrapper around graphviz's `Agraph_t` type for interacting with graph structures
pub struct Graph<'c> {
    graph: *mut Agraph_t,
    layout: Option<Layout>,
    ctx: &'c Context,
    nodes: HashMap<String, Node>,
    edges: HashMap<String, Edge>,
}

impl<'c> Graph<'c> {
    /// construct an existing graph from dot structure
    /// useful for when a graph was initially generated from a crate like petgraph, or graphviz-rust
    /// # Arguments
    /// - `dot`: do struct diagram as str reference
    /// - `ctx`: Context used to create the graph see `Context`
    /// # Note
    /// graph cannot outlive `Context`
    pub fn new<S: AsRef<str>>(dot: S, ctx: &'c Context) -> Self {
        let str_slice = dot.as_ref();
        let c_dot = CString::new(str_slice).expect("DOT string contains null bytes");

        let graph = unsafe { agmemread(c_dot.as_ptr()) };

        let mut nodes = HashMap::new();
        let mut edges = HashMap::new();

        println!("about to get nodes");
        unsafe {
            let mut n = agfstnode(graph);
            while !n.is_null() {
                // Get node name
                let name_ptr = agnameof(n as *mut _) as *const c_char;
                let name = CStr::from_ptr(name_ptr).to_string_lossy().into_owned();
                println!("about to try insert: {:?}", name);
                nodes.insert(name.clone(), Node { raw: n });

                // Iterate outgoing edges
                let mut e = agfstout(graph, n);
                while !e.is_null() {
                    
                    let tail = rust_agtail(e);  // source node pointer
                    let head = rust_aghead(e);  // destination node pointer

                    let tail_name_ptr = agnameof(tail as *mut _) as *const c_char;
                    let head_name_ptr = agnameof(head as *mut _) as *const c_char;

                    let tail_name = CStr::from_ptr(tail_name_ptr).to_string_lossy();
                    let head_name = CStr::from_ptr(head_name_ptr).to_string_lossy();

                    let edge_key = format!("{}->{}", tail_name, head_name);
                    edges.insert(edge_key, Edge { raw: e });
                    e = agnxtout(graph, e);
                }

                n = agnxtnode(graph, n);
            }
        }
        println!("about to return from Graph::new()");
        Self {
            graph,
            layout: None,
            ctx,
            nodes,
            edges,
        }
    }

    /// create's an empty graph with the given context
    pub fn empty(ctx: &'c Context) -> Self {
        let graph: *mut Agraph_t = unsafe {
            agopen(
                CString::new("G").unwrap().as_ptr() as *mut i8,
                Agdirected,
                std::ptr::null_mut(),
            )
        };
        Self {
            graph,
            layout: None,
            ctx,
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Adds a node by name (creates if not exist).
    /// # Arguments
    /// `name` - The name of the new node, used to identify it in the future.
    pub fn add_node<'n, S: AsRef<str> + 'n>(
        &'n mut self,
        name: S,
    ) -> Option<NodeBuilder<'n, 'c, S>> {
        let name_str = name.as_ref().to_string();
        let cname = CString::new(name_str.clone()).ok()?;
        let raw_node = unsafe { agnode(self.graph, cname.as_ptr() as *mut i8, 1) };
        if raw_node.is_null() {
            None
        } else {
            let node = Node { raw: raw_node };
            self.nodes.insert(name_str, node);
            Some(NodeBuilder {
                graph: self,
                node_name: name,
            })
        }
    }
    
    /// Add an edge between two existing nodes in the graph.
    ///
    /// # Parameters
    /// - `tail`: Name of the source node (edge starts here)
    /// - `head`: Name of the destination node (edge points here)
    /// - `name`: Unique name to identify this edge
    ///
    /// # Returns
    /// - `Some(String)`: The name of the added edge, if both nodes exist and adding succeeds
    /// - `None`: If either the `tail` or `head` node does not exist, or the internal add fails
    ///
    /// # Note
    /// This function requires that both nodes (`tail` and `head`) already exist in the graph's `nodes` map.
    /// Internally, it inserts the new edge into the `edges` map with the provided `name`.
    ///
    /// Whenever possible, prefer using this method over directly manipulating the internal maps,
    /// as it ensures consistency and handles internal setup.
    pub fn add_edge<'e, S: AsRef<str>>(
        &'e mut self,
        tail: S,
        head: S,
        name: S,
    ) -> Option<String> {
        let head: &Node = if let Some(head_node) = self.nodes.get(head.as_ref()) {
            head_node
        } else {
            return None;
        };

        let tail: &Node = if let Some(tail_node) = self.nodes.get(tail.as_ref()) {
            tail_node
        } else {
            return None;
        };

        if let Some(edge) = self.add_edge_internal(&tail, &head, &name) {
            self.edges.insert(name.as_ref().to_string(), edge);
            Some(name.as_ref().to_string())
        }else{
            None
        }
    }

    /// Adds a node from `tail` to `head` with optional `name`
    fn add_edge_internal<'e, S: AsRef<str>>(
        &'e self,
        tail: &Node,
        head: &Node,
        name: S,
    ) -> Option<Edge> {
        let name_str = name.as_ref().to_string();
        let cname = CString::new(name_str.clone()).ok();

        let name_ptr = cname.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());

        let raw_edge = unsafe { agedge(self.graph, tail.raw, head.raw, name_ptr as *mut i8, 1) };
        if raw_edge.is_null() {
            None
        } else {
            let edge = Edge { raw: raw_edge };
            Some(edge)
        }
    }

    /// sets an attribute on node identified by `node_name`
    /// # Note
    /// this is extensible because of the use of the Attribute trait
    /// but if an invalid type is passed in the failure will happen in C code, not rust code
    /// please use existing Attribute implementations whenever possible, or submit PR to add new attributes.
    pub fn set_attr_on_node<A>(&mut self, node_name: &str, attr: A) -> Result<(), String>
    where
        A: Attribute,
    {
        let node = self
            .nodes
            .get(node_name)
            .ok_or_else(|| format!("Node '{}' not found", node_name))?;

        let (name_cstr, value_cstr) = attr.to_cstrings();
        println!("trying to set attribute {:?} with value {:?}", name_cstr, value_cstr);
        ensure_node_attr(self.graph, name_cstr.clone(), attr.default());
        set_attr_on_ptr(node.raw as *mut libc::c_void, &name_cstr, &value_cstr)
    }

    /// Set attribute on an edge by name
    /// # Note
    /// Whenever possible use existing Attribute implementations defined in module style
    /// or submit a PR request to add attributes.
    pub fn set_attr_on_edge<A>(&mut self, edge_name: &str, attr: A) -> Result<(), String>
    where
        A: Attribute,
    {
        let edge = self
            .edges
            .get(edge_name)
            .ok_or_else(|| format!("Edge '{}' not found", edge_name))?;

        let (name_cstr, value_cstr) = attr.to_cstrings();
        set_attr_on_ptr(edge.raw as *mut libc::c_void, &name_cstr, &value_cstr)
    }

    /// Set attribute on the graph by name
    /// 
    /// # Note
    /// Whenever possible use existing Attribute implementations defined in module style
    /// or submit a PR request to add attributes.
    pub fn set_attr_on_graph<A>(&mut self, attr: A) -> Result<(), String>
    where
        A: Attribute,
    {
        let (name_cstr, value_cstr) = attr.to_cstrings();
        set_attr_on_ptr(self.graph as *mut libc::c_void, &name_cstr, &value_cstr)
    }

    /// Set attribute on the graph itself
    pub fn set_graph_attr<A>(&mut self, attr: A) -> Result<(), String>
    where
        A: Attribute,
    {
        let (name_cstr, value_cstr) = attr.to_cstrings();
        println!("created cstrings in set_graph_attr, {:?}, {:?}", name_cstr, value_cstr);

        if self.graph.is_null() {
            return Err("graph is null".into());
        }
        let def = CString::new("").unwrap();
        println!("ptr values: {:?}, {:?}, {:?}", self.graph, name_cstr.as_ptr(), value_cstr.as_ptr());
        let ret = unsafe {
            agsafeset(
                self.graph as *mut libc::c_void,
                name_cstr.as_ptr() as *mut libc::c_char,
                value_cstr.as_ptr(),
                def.as_ptr(),
            )
        };
        println!("called agsafeset successfully");
        if ret != 0 {
            Err(format!(
                "Failed to set graph attribute '{}'='{}'",
                name_cstr.to_string_lossy(),
                value_cstr.to_string_lossy()
            ))
        } else {
            Ok(())
        }
    }

    fn set_attribute_cstrings(&mut self, name: &CString, value: &CString) -> Result<(), String> {
        let def = std::ptr::null();
        let ret = unsafe {
            agsafeset(
                self.graph as *mut libc::c_void,
                name.as_ptr() as *mut i8,
                value.as_ptr() as *mut i8,
                def,
            )
        };
        if ret != 0 {
            Err(format!(
                "Failed to set graph attribute '{}'='{}'",
                name.to_string_lossy(),
                value.to_string_lossy()
            ))
        } else {
            Ok(())
        }
    }

    /// Sets the layout engine for the graph (e.g., "dot", "neato").
    ///
    /// This method must be called before rendering or exporting the graph,
    /// as the layout engine determines how the graph is positioned and styled.
    ///
    /// # Panics
    /// * If the layout string contains interior null bytes.
    /// * If the underlying `gvLayout` call fails (non-zero error code).
    ///
    /// # Arguments
    /// * `layout` - The chosen layout engine as a [`Layout`] enum.
    ///
    /// # Example
    /// ```none
    /// graph.set_layout(Layout::Dot);
    /// ```
    pub fn set_layout(&mut self, layout: Layout) -> Result<(), String> {
        self.layout = Some(layout);
        let layout_str = layout.to_string();

        let c_layout =
            CString::new(layout_str).expect("Layout string contains interior null bytes");

        // Call the layout function with the layout string pointer
        let result = unsafe { gvLayout(self.ctx.ctx, self.graph, c_layout.as_ptr()) };

        if result != 0 {
            return Err(format!("gvLayout failed with error code {}", result));
        }
        Ok(())
    }

    /// Generates the DOT representation of the current graph.
    ///
    /// # Panics
    /// Panics if no layout engine has been set before calling this method.
    /// Make sure to call [`set_layout`] first.
    ///
    /// # Returns
    /// * `Ok(String)` containing the DOT format of the graph.
    /// * `Err(std::string::FromUtf8Error)` if the generated data is not valid UTF-8.
    ///
    /// # Example
    /// ```none
    /// graph.set_layout(Layout::Dot);
    /// let dot_output = graph.to_dot().unwrap();
    /// println!("{}", dot_output);
    /// ```
    pub fn to_dot(&self) -> Result<String, String> {
        if self.layout.is_none() {
            panic!("called to_dot without setting layout");
        }
        let data = self.ctx.render(self, OutputFormat::Dot)?;
        Ok(String::from_utf8(data).unwrap())
    }

    /// Creates a new cluster subgraph within this graph.
    ///
    /// # Parameters
    /// - `name`: The name of the cluster (without the `"cluster_"` prefix).
    ///
    /// # Returns
    /// Returns a new `Graph` instance wrapping the created cluster.
    ///
    /// # Errors
    /// Returns `Err` if the cluster could not be created (e.g., due to invalid name or allocation failure).
    pub fn add_cluster<S: AsRef<str>>(&mut self, name: S) -> Result<Graph<'c>, String> {
        // Prepend "cluster_" to comply with Graphviz convention
        let cluster_name = format!("cluster_{}", name.as_ref());

        // Convert to CString; handle potential interior null byte error
        let c_cluster_name = CString::new(cluster_name)
            .map_err(|_| "Cluster name contains interior null byte".to_string())?;

        // Call agsubg to create the cluster
        let cluster_ptr = unsafe { agsubg(self.graph, c_cluster_name.as_ptr() as *mut c_char, 1) };

        if cluster_ptr.is_null() {
            return Err("Failed to create cluster".into());
        }

        // Wrap in new Graph struct
        Ok(Graph {
            graph: cluster_ptr,
            layout: None,                    // cluster starts without layout
            ctx: self.ctx,                   // reuse the same context
            nodes: HashMap::new(),           // cluster initially empty
            edges: HashMap::new(),
        })
    }
}

fn ensure_node_attr(graph: *mut Agraph_t, name: CString, default: &str) {
    
    let name_cstr = CString::new(name).unwrap();
    let default_cstr = CString::new(default).unwrap();
    unsafe {
        agattr(
            graph,
            AGNODE.try_into().unwrap(),
            name_cstr.as_ptr() as *mut i8,
            default_cstr.as_ptr() as *mut i8
        );
    }
}

fn set_attr_on_ptr(obj: *mut libc::c_void, name: &CString, value: &CString) -> Result<(), String> {
    let def = CString::new("").unwrap();
    let ret = unsafe {
        agsafeset_text(
            obj,
            name.as_ptr() as *mut i8,
            value.as_ptr(),
            def.as_ptr(),
        )
    };
    if ret != 0 {
        Err(format!(
            "Failed to set attribute '{}'='{}'",
            name.to_string_lossy(),
            value.to_string_lossy()
        ))
    } else {
        Ok(())
    }
}

impl<'c> std::ops::Drop for Graph<'c> {
    fn drop(&mut self) {
        unsafe {
            if let Some(_) = self.layout {
                //gvFreeLayout(self.ctx.ctx, self.graph);
            }
            //println!("freed layout");
            agclose(self.graph);
        }
    }
}

/// Created from `sys::gvContext()` this struct is used to initialize graphs
pub struct Context {
    ctx: *mut GVC_t,
}

impl Context {
    pub fn new() -> Context {
        Self {
            ctx: unsafe { gvContext() },
        }
    }

    /// Renders the provided `graph` into the specified `format` using Graphviz.
    ///
    /// # Arguments
    /// - `graph`: The graph to render.
    /// - `format`: The desired output format (e.g., `OutputFormat::Svg`, `OutputFormat::Png`).
    ///
    /// # Returns
    /// A `Vec<u8>` containing the rendered graph output (e.g., SVG or image bytes).
    ///
    /// # Panics
    /// Panics if rendering fails or if the format string cannot be converted to a C string.
    pub fn render(&self, graph: &Graph, format: OutputFormat) -> Result<Vec<u8>, String> {
        let format_cstr = CString::new(format.to_string())
            .expect("Failed to convert output format to CString");
        let result_str = CString::new("").unwrap();
        let mut result_ptr: *mut std::os::raw::c_char = result_str.as_ptr() as *mut std::os::raw::c_char;
        let mut length: usize = 0;
    
        unsafe {
            println!("about to render data with ptrs: {:?}, {:?}, {:?}, {:?}, {:?}", self.ctx, graph.graph, format_cstr.as_ptr(), &mut result_ptr, &mut length as *mut usize);
            if gvRenderData(self.ctx, graph.graph, format_cstr.as_ptr(), &mut result_ptr, &mut length as *mut usize) != 0 {
                return Err("Graphviz render failed".into());
            }
            println!("after gvRender");
            let output = std::slice::from_raw_parts(result_ptr as *const u8, length).to_vec();
            println!("tried to construct output");
            gvFreeRenderData(result_ptr);
            Ok(output)
        }
    }
}

/// wrapper struct for `sys::Agnode_t` for handling graph nodes
#[derive(Clone)]
pub struct Node {
    raw: *mut Agnode_t,
}



/// wrapper struct for `sys::Agedge_t` for adding edges between nodes
pub struct Edge {
    raw: *mut Agedge_t,
}

impl std::ops::Drop for Context {
    fn drop(&mut self) {
        unsafe {
            gvFreeContext(self.ctx);
        }
    }
}

pub trait DotGraph {
    type Err: std::error::Error;
    fn generate_dot(&self) -> Result<String, Self::Err>;
}

pub trait GraphEngine {
    type Err: std::error::Error;
    fn render_graph<G: DotGraph>(&self, graph: &G, format: OutputFormat) -> Result<String, Self::Err>;
}

/*
impl GraphEngine for Context {
    
    fn render_graph<G: Graph>(&self, graph: &G, format: OutputFormat) -> Result<String, Self::Err> {
        self.render(graph, format)
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    /// Dummy Context for testing.
    /// Replace with your actual Context initialization if needed.
    fn dummy_context() -> Context {
        // adjust this to create an actual context, e.g., Context::new()
        Context::new()
    }

    #[test]
    fn test_graph_new_with_valid_dot() {
        let ctx = dummy_context();
        let dot = r#"
            digraph G {
                A -> B;
                B -> C;
                C -> A;
                D;
            }
        "#;

        let mut graph = Graph::new(dot, &ctx);

        // Make sure nodes were created
        assert!(graph.nodes.contains_key("A"));
        assert!(graph.nodes.contains_key("B"));
        assert!(graph.nodes.contains_key("C"));
        assert!(graph.nodes.contains_key("D"));

        // Make sure edges were created
        assert!(!graph.edges.is_empty());
        // Edge names might vary; check there’s at least the number we expect
        assert!(graph.edges.len() >= 3);

        let edge_list = graph.edges.keys().map(|k| k.to_string()).collect::<Vec<String>>();

        for edge_name in edge_list.iter() {
            // Example attributes: color and label
            let color_attr = EdgeAttr::Color("#ff0000".parse().unwrap());
            let label_attr = EdgeAttr::LabelDistance(1.5); // replace with actual EdgeAttr variant if needed
    
            graph
                .set_attr_on_edge(edge_name, color_attr.clone())
                .expect(&format!("Failed to set color attribute on edge {}", edge_name));
    
            graph
                .set_attr_on_edge(edge_name, label_attr.clone())
                .expect(&format!("Failed to set label attribute on edge {}", edge_name));
        }
    }
    
    #[test]
    fn test_graph_new_empty_dot() {
        let ctx = dummy_context();
        let dot = "digraph G {}";

        let graph = Graph::new(dot, &ctx);

        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
    }

    /*#[test]
    fn test_graph_new_invalid_dot() {
        let ctx = dummy_context();
        let dot = "invalid_dot {";

        // Graphviz will likely still create a graph but with no nodes/edges
        let graph = Graph::new(dot, &ctx);
        assert!(graph.nodes.is_empty() || graph.nodes.len() == 1);
        // There shouldn’t be edges if dot is invalid
        assert!(graph.edges.is_empty());
    }*/
}