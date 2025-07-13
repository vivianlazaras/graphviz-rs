#![allow(non_camel_case_types)]
use structopt::StructOpt;

use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::error::Error;

// Adjust these use statements to your custom crate structure:
use wrappedviz::{Context, Graph, Layout, OutputFormat};

#[derive(Debug, Clone, StructOpt)]
pub struct Args {
    /// Input DOT file
    #[structopt(short, long)]
    input: PathBuf,

    /// Output SVG file (optional)
    #[structopt(short, long)]
    output: Option<PathBuf>,
}

fn read_file(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();

    // Read DOT file into a string
    let content = read_file(&args.input)?;

    // Create rendering context
    let context = Context::new();

    // Create graph from DOT string
    let mut graph = Graph::new(content, &context);

    // Set layout engine (dot)
    graph.set_layout(Layout::Dot);

    // Render to SVG
    let svg_slice = context.render(&graph, OutputFormat::Svg)?;

    // Convert &[u8] to String (assuming UTF-8)
    let svg = String::from_utf8_lossy(svg_slice.as_slice());

    match args.output {
        Some(output_path) => {
            let mut file = File::create(output_path)?;
            file.write_all(svg.as_bytes())?;
        }
        None => {
            println!("{}", svg);
        }
    }

    Ok(())
}
