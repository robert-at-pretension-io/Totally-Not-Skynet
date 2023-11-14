use crate::generated_types::{
    Node,
    NodeContent,
    Process,
    Graph,
    Edge,
    GraphNodeInfo,
    node_content::NodeContent as NodeContentEnum,
};
use colored::*;

use petgraph::{ Direction, graph::DiGraph };

use petgraph::algo::toposort;

use std::collections::HashMap;

use anyhow::Error;

pub fn validate_nodes_in_process(
    nodes: Vec<Node>,
    graph_node_info: GraphNodeInfo
) -> Result<Node, String> {
    //generate maximal graph from nodes (based on input_variables and output_variables)
    println!("Validating nodes");
    // println!("Validating nodes for user: {:?}", msg.0);
    println!("need to return a Graph object");

    // create a petgraph digraph:

    println!("Number of nodes: {:?}", nodes.len());

    // this will go in the node_info of the response node
    let containing_node: GraphNodeInfo = GraphNodeInfo {
        id: uuid::Uuid::new_v4().to_string(),
        name: graph_node_info.clone().name,
        description: graph_node_info.clone().description,
    };

    let mut input_vars: Vec<String> = Vec::new();
    let mut output_vars: Vec<String> = Vec::new();
    // loop through the vector of nodes and add input variables to the input_vars vector (if they are not already in the vector)
    for node in &nodes {
        for input_var in &node.input_variables {
            if !input_vars.contains(input_var) {
                input_vars.push(input_var.clone());
            }
        }
        // loop through the output_variables and remove those from the input_vars vector
        for output_var in &node.output_variables {
            if !output_vars.contains(output_var) {
                output_vars.push(output_var.clone());
            }
        }
    }

    //output of the process
    let output_minus_input = output_vars
        .clone()
        .into_iter()
        .filter(|output_var| !input_vars.contains(output_var))
        .collect::<Vec<String>>();

    //input of the process
    let input_minus_output = input_vars
        .clone()
        .into_iter()
        .filter(|input_var| !output_vars.contains(input_var))
        .collect::<Vec<String>>();

    // From the nodes and their dependencies, we need to populate a petgraph graph so that we can run the transitive reduction algorithm on it
    // This will give a "minimal" graph that has the same topological order as the larger graph
    // This will allow the user to have a visualization of the graph that is not cluttered with unnecessary nodes and is easy to understand the topological order

    let mut graph = DiGraph::new();
    println!("Initialized empty graph");

    let mut new_nodes: Vec<GraphNodeInfo> = Vec::new();

    // let mut node_indices;
    let mut node_indices: HashMap<String, petgraph::graph::NodeIndex> = HashMap::new();

    for node in &nodes {
        let node_info = node.node_info.clone().unwrap();

        println!("Processing node: {:?}", node_info.id);
        new_nodes.push(node_info);
        let node_index = graph.add_node(node.clone());
        node_indices.insert(node.node_info.clone().unwrap().id, node_index);
    }

    let mut mut_pruned_graph = graph.clone();

    println!("All nodes added to graph");

    // Add edges based on input_variables and output_variables
    for node in &nodes {
        let node_index = node_indices[&node.node_info.clone().unwrap().id];
        println!(
            "Adding edges for node: {:?}, Input Vars: {:?}",
            node.node_info.clone().unwrap().id,
            node.input_variables
        );
        for input_var in &node.input_variables {
            // Find nodes that output this input_var
            // For demonstration, using the same list of nodes
            println!("Checking for input_var: {:?}", input_var);
            for other_node in &nodes {
                println!(
                    "Other node: {:?}, Output Vars: {:?}",
                    other_node.node_info.clone().unwrap().id,
                    other_node.output_variables
                );
                if other_node.output_variables.contains(input_var) {
                    let other_node_index = node_indices[&other_node.node_info.clone().unwrap().id];
                    println!(
                        "Found matching output_var in node: {:?}",
                        other_node.node_info.clone().unwrap().id
                    );
                    graph.add_edge(other_node_index, node_index, ());
                }
            }
        }
    }
    println!("All edges added");

    println!("{}", "Remove the excess edges here".red());

    let top_sort = petgraph::algo::toposort(&graph, None).unwrap();

    let adjacency_list = petgraph::algo::tred::dag_to_toposorted_adjacency_list::<
        _,
        petgraph::graph::DefaultIx
    >(&graph, &top_sort);

    // The output is the pair of the transitive reduction and the transitive closure.
    let (transative_reduct, _) = petgraph::algo::tred::dag_transitive_reduction_closure::<
        _,
        petgraph::graph::DefaultIx
    >(&adjacency_list.0);

    // The graph should have the same nodes but different edges.

    // for all of the edges in transivite_reduct, add that edge to the mut_pruned_graph

    transative_reduct.edge_indices().for_each(|edge| {
        // how to get the source and target nodes from the edge?
        let (sourceIndex, targetIndex) = transative_reduct.edge_endpoints(edge).unwrap();
        mut_pruned_graph.add_edge(sourceIndex.into(), targetIndex.into(), ());
    });

    let mut new_edges: Vec<Edge> = Vec::new();

    // Count and print the number of edges
    // let edge_count = rebuilt_graph.raw_edges().len();
    // println!("Total number of raw edges: {}", edge_count);

    mut_pruned_graph
        .raw_edges()
        .iter()
        .for_each(|edge| {
            let source_node = graph.node_weight(edge.source()).unwrap();
            let target_node = graph.node_weight(edge.target()).unwrap();

            let new_edge: Edge = Edge {
                source: source_node.node_info.clone(),
                target: target_node.node_info.clone(),
            };

            new_edges.push(new_edge);
        });

    let new_graph = Graph {
        nodes_info: new_nodes,
        edges: new_edges,
    };

    let mut starting_nodes = Vec::new();

    for node in graph.node_indices() {
        if graph.neighbors_directed(node, Direction::Incoming).count() == 0 {
            println!("Node with no incoming edges: {:?}", graph[node]);
            starting_nodes.push(node.clone());
        }
    }

    let mut topological_order = Vec::new();

    // println!("Starting nodes: {:?}", starting_nodes);

    // if starting_nodes.len() == 0 {
    //     println!("{}", "No starting nodes found".red());
    //     continue;
    // } else {
    //     let start_node = starting_nodes[0].clone();
    //     let mut bfs = Bfs::new(&mut_pruned_graph, start_node);
    //     while let Some(nx) = bfs.next(&graph) {
    //         // we can access `graph` mutably here still

    //         //loop through the nodes vector and find the node with the same id as mut_pruned_graph[nx]

    //         let node = new_graph.nodes
    //             .iter()
    //             .find(
    //                 |node|
    //                     node.id ==
    //                     mut_pruned_graph[nx].node_info.as_mut().unwrap().id
    //             )
    //             .unwrap();

    //         topological_order.push(node.clone());
    //     }
    // }

    let index_vec = toposort(&mut_pruned_graph, None).unwrap();

    for index in index_vec {
        let node = new_graph.nodes_info
            .iter()
            .find(|node| node.id == mut_pruned_graph[index].node_info.as_mut().unwrap().id)
            .unwrap();
        topological_order.push(node.clone());
    }

    let process: Process = Process {
        nodes: nodes,
        graph: Some(new_graph),
        topological_order: topological_order,
    };

    let node_content: NodeContent = NodeContent {
        node_content: Some(NodeContentEnum::Process(process)),
    };

    let node: crate::generated_types::Node = crate::generated_types::Node {
        node_info: Some(containing_node),
        node_type: crate::generated_types::NodeTypes::Process as i32,
        input_variables: input_minus_output,
        output_variables: output_minus_input,
        node_content: Some(node_content),
    };

    return Ok(node);
}
