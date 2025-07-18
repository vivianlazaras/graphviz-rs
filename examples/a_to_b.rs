use graphviz::*;
use graphviz::style::{NodeAttr, GraphAttr, CommonAttr};
use graphviz::style::shape::NodeShape;
fn main() {
    let ctx = Context::new();
    // Start with an empty directed graph
    let mut graph = Graph::new("digraph G {}", &ctx);
    // Add nodes A and B
    graph.add_node("A");
    graph.add_node("B");
    graph.add_edge("A", "B", "A_to_B");
    // Style node A
    graph.set_attr_on_node("A", NodeAttr::Shape(NodeShape::Box)).unwrap();
    // Label the edge
    graph.set_attr_on_edge("A_to_B", CommonAttr::Label("A to B".into())).unwrap();
    // Set graph-level attributes
    graph.set_attr_on_graph(GraphAttr::RankDir(style::RankDir::LR)).unwrap();
    // Layout the graph using `dot`
    graph.set_layout(Layout::Dot);
    // Render the graph to SVG format
    let svg = ctx.render(&graph, OutputFormat::Svg);
    std::fs::write("example.svg", svg).unwrap();
}