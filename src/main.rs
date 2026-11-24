use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use tree_meeting_point::Tree;

const USAGE: &str = "Usage: tree-meeting-point <tree-file> [--compare <vertex>]";

fn main() -> ExitCode {
    match run(env::args_os().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Vec<OsString>) -> Result<(), String> {
    let (path, comparison) = match args.as_slice() {
        [help] if help == "--help" || help == "-h" => {
            println!("{USAGE}");
            return Ok(());
        }
        [path] => (PathBuf::from(path), None),
        [path, flag, vertex] if flag == "--compare" => {
            let vertex = vertex
                .to_str()
                .and_then(|value| value.parse::<usize>().ok())
                .ok_or("the comparison vertex must be a positive integer")?;
            (PathBuf::from(path), Some(vertex))
        }
        _ => return Err(USAGE.into()),
    };

    let input =
        fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let tree = parse_tree(&input).map_err(|error| format!("{}: {error}", path.display()))?;
    let center = tree.center();
    let comparison = comparison
        .map(|vertex| tree.eccentricity(vertex).map(|distance| (vertex, distance)))
        .transpose()
        .map_err(|error| error.to_string())?;

    let vertices = center
        .vertices
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    let label = if center.vertices.len() == 1 {
        "Optimal meeting point"
    } else {
        "Optimal meeting points"
    };
    println!("{label}: {vertices}");
    println!("Optimal worst-case walk: {}", edge_count(center.radius));
    if let Some((vertex, distance)) = comparison {
        println!(
            "Vertex {vertex} worst-case walk: {} (+{})",
            edge_count(distance),
            edge_count(distance - center.radius)
        );
    }
    Ok(())
}

fn edge_count(count: usize) -> String {
    let unit = if count == 1 { "edge" } else { "edges" };
    format!("{count} {unit}")
}

fn parse_tree(input: &str) -> Result<Tree, String> {
    let mut lines = input.lines().enumerate();
    let (_, header) = lines.next().ok_or("expected a vertex count on line 1")?;
    let vertex_count = header
        .trim()
        .parse()
        .map_err(|_| "line 1: expected an integer vertex count")?;

    let mut edges = Vec::new();
    for (index, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        let error = || format!("line {}: expected two integer vertex IDs", index + 1);
        let mut fields = line.split_whitespace();
        let a = fields
            .next()
            .and_then(|field| field.parse().ok())
            .ok_or_else(&error)?;
        let b = fields
            .next()
            .and_then(|field| field.parse().ok())
            .ok_or_else(&error)?;
        if fields.next().is_some() {
            return Err(error());
        }
        edges.push((a, b));
    }
    Tree::from_edges(vertex_count, &edges).map_err(|error| error.to_string())
}
