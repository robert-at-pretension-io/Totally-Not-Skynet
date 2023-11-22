use petgraph::graph::{ DiGraph, NodeIndex };
use petgraph::dot::{ Dot, Config };
use petgraph::visit::EdgeRef;
use petgraph::algo::tarjan_scc;

use std::fs::File;
use std::io::Write;
use std::collections::{ HashSet, HashMap };

use std::process::Command;

use petgraph::visit::{ Dfs, IntoNodeReferences, Visitable };
fn dfs_from_node<'a>(graph: &'a DiGraph<&'a str, &'a str>, start_node: NodeIndex) -> Vec<&'a str> {
    let mut dfs = Dfs::new(graph, start_node);
    let mut visit_order = Vec::new();

    while let Some(nx) = dfs.next(graph) {
        visit_order.push(graph[nx]);
    }

    visit_order
}

fn main() {
    // Create a new directed graph
    let mut graph = DiGraph::<&str, &str>::new();
    let mut data_to_index = HashMap::new();

    // Add nodes
    let a = graph.add_node("A");
    data_to_index.insert("A", a);
    let b = graph.add_node("B");
    data_to_index.insert("B", b);
    let c = graph.add_node("C");
    data_to_index.insert("C", c);
    let d = graph.add_node("D");
    data_to_index.insert("D", d);
    let e = graph.add_node("E");
    data_to_index.insert("E", e);
    let f = graph.add_node("F");
    data_to_index.insert("F", f);
    let g = graph.add_node("G");
    data_to_index.insert("G", g);
    let h = graph.add_node("H");
    data_to_index.insert("H", h);
    let i = graph.add_node("I");
    data_to_index.insert("I", i);
    let j = graph.add_node("J");
    data_to_index.insert("J", j);
    let k = graph.add_node("K");
    data_to_index.insert("K", k);
    let l = graph.add_node("L");
    data_to_index.insert("L", l);

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
    graph.add_edge(e, k, "to K");
    graph.add_edge(k, l, "to L");
    graph.add_edge(l, e, "to E");
    // Print and save the original graph's DOT
    save_dot_file(&graph, "original_graph.dot");

    // Generate image for the original graph
    generate_image("original_graph.dot", "original_graph.png");

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    // Tarjan's algorithm to find SCCs
    let sccs = tarjan_scc(&graph);

    let mut all_node_set = HashSet::new();

    for (scc_index, scc) in sccs.iter().enumerate() {
        // Skip SCCs with only one node
        if scc.len() <= 1 {
            continue;
        }

        let mut node_set = HashSet::new();
        node_set.extend(scc.iter().cloned());
        all_node_set.extend(scc.iter().cloned());

        let mut subgraph = DiGraph::<&str, &str>::new();
        let mut subgraph_nodes = HashMap::new();

        // Add nodes of the SCC to the subgraph
        for &node_index in &node_set {
            let node_data = graph[node_index];
            let subgraph_node = subgraph.add_node(node_data);
            subgraph_nodes.insert(node_index, subgraph_node);
        }

        // Add edges of the SCC to the subgraph
        for &node_index in &node_set {
            if let Some(&subgraph_source) = subgraph_nodes.get(&node_index) {
                for edge in graph.edges(node_index) {
                    if node_set.contains(&edge.target()) {
                        if let Some(&subgraph_target) = subgraph_nodes.get(&edge.target()) {
                            subgraph.add_edge(subgraph_source, subgraph_target, *edge.weight());
                        }
                    }
                }
            }
        }
        // Choose a starting node for DFS in the subgraph
        let start_node = subgraph_nodes[&scc[0]];

        // Perform DFS on the subgraph and get the visit order
        let visit_order = dfs_from_node(&subgraph, start_node);

        println!("Visit order in subgraph {}: {:?}", scc_index, visit_order);

        // Print and save the subgraph's DOT
        let subgraph_dot_filename = format!("subgraph_{}.dot", scc_index);
        save_dot_file(&subgraph, &subgraph_dot_filename);

        // Generate image for the subgraph
        let subgraph_image_filename = format!("subgraph_{}.png", scc_index);
        generate_image(&subgraph_dot_filename, &subgraph_image_filename);
    }

    // let mut labels = Vec::new();

    // let mut indices = Vec::new();
    // let mut index = -1;

    // for scc in sccs {
    //     index += 1;

    //     if scc.len() <= 1 {
    //         continue;
    //     }

    //     let scc_node_label = format!("SCC_{}", scc[0].index());

    //     labels.push(scc_node_label.clone()); // Push the label to the vector

    //     indices.push(index); // Push the index to the vector
    // }

    for scc in sccs {
        if scc.len() <= 1 {
            continue;
        }
        // Retrieve the reference to the label
        // let label_ref = labels.last().unwrap();

        // Add the node to the graph
        let scc_node = graph.add_node("SCC");

        let mut edges_to_add = Vec::new();

        let node_set: HashSet<&str> = scc
            .iter()
            .map(|&node_index| graph[node_index])
            .collect();

        for node_data in &node_set {
            let node_index = data_to_index[node_data];

            for edge in graph.edges_directed(node_index, petgraph::Direction::Incoming) {
                if !node_set.contains(&graph[edge.source()]) {
                    edges_to_add.push((edge.source(), scc_node, "to SCC"));
                }
            }

            for edge in graph.edges_directed(node_index, petgraph::Direction::Outgoing) {
                if !node_set.contains(&graph[edge.target()]) {
                    edges_to_add.push((scc_node, edge.target(), "from SCC"));
                }
            }
        }

        // Now, add the collected edges to the graph
        for (source, target, label) in edges_to_add {
            graph.add_edge(source, target, label);
        }

        // Remove the nodes that are part of the SCC
        for node_data in &node_set {
            if let Some(node_index) = graph.node_indices().find(|&n| graph[n] == *node_data) {
                println!("Removing node: {:?}", graph[node_index]); // Debug print for node removal
                graph.remove_node(node_index);
            }
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
