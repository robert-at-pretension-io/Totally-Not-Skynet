use petgraph::graph::{ DiGraph, NodeIndex };
use petgraph::dot::{ Dot, Config };

use std::fs::File;
use std::io::Write;

use std::process::Command;

fn main() {
    // Create a new directed graph
    let mut graph = DiGraph::<&str, &str>::new();

    // Add nodes
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    let f = graph.add_node("F");
    let g = graph.add_node("G");
    let h = graph.add_node("H");

    // Add edges
    graph.add_edge(a, b, "to B");
    graph.add_edge(b, c, "to C");
    graph.add_edge(b, f, "to F");
    graph.add_edge(g, b, "to B");
    graph.add_edge(c, a, "to A"); // This creates a cycle A -> B -> C -> A
    graph.add_edge(a, d, "to D");
    graph.add_edge(d, e, "to E");
    graph.add_edge(h, a, "to A");

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    // Convert Dot to a string
    let dot = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    // Write the DOT representation to a file
    let mut file = File::create("graph.dot").unwrap();
    file.write_all(dot.as_bytes()).unwrap();

    // Execute the Graphviz 'dot' command
    let output = Command::new("dot")
        .args(&["-Tpng", "graph.dot", "-o", "graph.png"])
        .output()
        .unwrap();
}
