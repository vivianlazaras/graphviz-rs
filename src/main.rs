#![allow(non_camel_case_types)]
use graphviz::*;

fn main() {
    let context = Context::new();
    let dot = "digraph G { a -> b; b -> c; }";
    let mut graph = Graph::new(dot, &context);
    graph.set_layout(Layout::Dot);

    let svg_slice = context.render(graph, OutputFormat::Svg);
    let svg = String::from_utf8_lossy(&svg_slice);
    println!("{}", svg);

}
