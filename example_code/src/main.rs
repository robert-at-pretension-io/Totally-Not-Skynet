use petgraph::graph::{ DiGraph, NodeIndex };
use petgraph::dot::{ Dot, Config };
use petgraph::visit::EdgeRef;
use petgraph::algo::tarjan_scc;

use std::fs::File;
use std::io::Write;
use std::collections::HashSet;

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
    let i = graph.add_node("I");
    let j = graph.add_node("J");

    // Add edges
    graph.add_edge(a, b, "to B");
    graph.add_edge(b, c, "to C");
    graph.add_edge(b, f, "to F");
    graph.add_edge(g, b, "to B");
    graph.add_edge(c, a, "to A"); // This creates a cycle A -> B -> C -> A
    graph.add_edge(a, d, "to D");
    graph.add_edge(d, e, "to E");
    graph.add_edge(h, a, "to A");
    graph.add_edge(a, i, "to I");
    graph.add_edge(i, j, "to J");
    graph.add_edge(j, a, "to A");

    // Print and save the original graph's DOT
    save_dot_file(&graph, "original_graph.dot");

    // Generate image for the original graph
    generate_image("original_graph.dot", "original_graph.png");

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    // Tarjan's algorithm to find SCCs
    let sccs = tarjan_scc(&graph);

    for scc in &sccs {
        let scc_names: Vec<&str> = scc
            .iter()
            .map(|node_index| graph[*node_index])
            .collect();
        println!("SCC: {:?}", scc_names);
    }
    // For simplicity, take the first SCC with more than one node and abstract it
    let first_scc = sccs
        .into_iter()
        .find(|scc| scc.len() > 1)
        .unwrap_or_default();
    let mut node_set = HashSet::new();
    node_set.extend(first_scc.iter().cloned());

    // println!(
    //     "First SCC to abstract: {:?}",
    //     first_scc
    //         .iter()
    //         .map(|&node_index| graph[node_index])
    //         .collect::<Vec<&str>>()
    // );

    // Convert first SCC to a set of node data instead of indices

    // Create a new node to represent the SCC
    let scc_node = graph.add_node("SCC");

    let mut edges_to_add = Vec::new();

    for &node in &node_set {
        for edge in graph.edges_directed(node, petgraph::Direction::Incoming) {
            if !node_set.contains(&edge.source()) {
                edges_to_add.push((edge.source(), scc_node, "to SCC"));
            }
        }

        for edge in graph.edges_directed(node, petgraph::Direction::Outgoing) {
            if !node_set.contains(&edge.target()) {
                edges_to_add.push((scc_node, edge.target(), "from SCC"));
            }
        }
    }

    // Now, add the collected edges to the graph
    for (source, target, label) in edges_to_add {
        graph.add_edge(source, target, label);
    }

    println!("Graph before removing SCC nodes:");
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let node_set: HashSet<&str> = first_scc
        .iter()
        .map(|&node_index| graph[node_index])
        .collect();

    // Remove the nodes that are part of the SCC
    for node_data in &node_set {
        if let Some(node_index) = graph.node_indices().find(|&n| graph[n] == *node_data) {
            println!("Removing node: {:?}", graph[node_index]); // Debug print for node removal
            graph.remove_node(node_index);
        }
    }

    // Print and save the modified graph's DOT
    save_dot_file(&graph, "modified_graph.dot");

    // Generate image for the modified graph
    generate_image("modified_graph.dot", "modified_graph.png");
}

fn save_dot_file(graph: &DiGraph<&str, &str>, filename: &str) {
    let dot = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let mut file = File::create(filename).unwrap();
    file.write_all(dot.as_bytes()).unwrap();
}

fn generate_image(input_file: &str, output_file: &str) {
    let _output = Command::new("dot")
        .args(&["-Tpng", input_file, "-o", output_file])
        .output()
        .unwrap();
}
