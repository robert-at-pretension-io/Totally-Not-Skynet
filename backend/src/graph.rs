use crate::generated_types::{self, value, AtomicExecutionLog, UserSettings};
use crate::generated_types::{
    node_content::NodeContent as NodeContentEnum, Command, Edge, Execution, Graph, GraphNodeInfo,
    Loop, Node, NodeContent, NodeTypes, Process,
};

use async_openai::config::OpenAIConfig;
use futures_util::StreamExt;

use bollard::container::LogOutput;
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::Docker;

use petgraph::algo::tarjan_scc;

use petgraph::adj::NodeIndex;

use async_recursion::async_recursion;

use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, ChatCompletionResponseFormat,
    ChatCompletionResponseFormatType, CreateChatCompletionRequest, Role,
};

use handlebars::Handlebars;

use async_openai::Client;
use colored::*;

use petgraph::{graph::DiGraph, Direction};

use petgraph::algo::toposort;

use std::collections::HashMap;
use std::sync::Arc;

use petgraph::visit::EdgeRef;

// use anyhow::Error;

pub fn validate_nodes_in_process(
    nodes: Vec<Node>,
    graph_node_info: GraphNodeInfo,
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

    let _conditional_output_variables: Vec<String> = Vec::new();

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
        petgraph::graph::DefaultIx,
    >(&graph, &top_sort);

    // The output is the pair of the transitive reduction and the transitive closure.
    let (transative_reduct, _) = petgraph::algo::tred::dag_transitive_reduction_closure::<
        _,
        petgraph::graph::DefaultIx,
    >(&adjacency_list.0);

    // The graph should have the same nodes but different edges.

    // for all of the edges in transivite_reduct, add that edge to the mut_pruned_graph

    transative_reduct.edge_indices().for_each(|edge| {
        // how to get the source and target nodes from the edge?
        let (source_index, target_index) = transative_reduct.edge_endpoints(edge).unwrap();
        mut_pruned_graph.add_edge(source_index.into(), target_index.into(), ());
    });

    let mut new_edges: Vec<Edge> = Vec::new();

    // Count and print the number of edges
    // let edge_count = rebuilt_graph.raw_edges().len();
    // println!("Total number of raw edges: {}", edge_count);

    mut_pruned_graph.raw_edges().iter().for_each(|edge| {
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
        let node = new_graph
            .nodes_info
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

pub fn validate_nodes_in_loop(
    nodes: Vec<Node>,
    graph_node_info: GraphNodeInfo,
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

    // check that exactly one of the nodes is a conditional node:

    let mut number_of_conditionals = 0;

    for node in nodes.clone() {
        if node.node_type == (NodeTypes::Conditional as i32) {
            number_of_conditionals += 1;
        }
    }

    if number_of_conditionals != 1 {
        return Err("There must be exactly one conditional node in a loop".to_string());
    }

    let mut conditional_output_variables: Vec<String> = Vec::new();

    let mut input_vars: Vec<String> = Vec::new();
    let mut output_vars: Vec<String> = Vec::new();
    // loop through the vector of nodes and add input variables to the input_vars vector (if they are not already in the vector)
    for node in &nodes.clone() {
        for input_var in &node.clone().input_variables {
            if !input_vars.contains(input_var) {
                input_vars.push(input_var.clone());
            }
        }
        // loop through the output_variables and remove those from the input_vars vector
        for output_var in &node.output_variables {
            if node.node_type == (NodeTypes::Conditional as i32) {
                println!("{}", "Setting the conditional output variables".green());
                conditional_output_variables = node.output_variables.clone();
            }

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

    // if output_minus_input is empty then return an error:

    if output_minus_input.len() == 0 {
        return Err(
            "There must be at least one output variable that is not an input variable in a loop"
                .to_string(),
        );
    }

    //input of the process
    let mut input_minus_output = input_vars
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

    // Find strongly connected components
    let scc = tarjan_scc(&graph);

    // Count the number of strongly connected components
    let scc_count = scc.len();

    // check that there is exactly ONE strongly connected component in the graph

    if scc_count != 1 {
        return Err("There must be exactly one strongly connected component in a loop".to_string());
    }

    println!("All edges added");

    println!(
        "{}",
        "Remove the edges that go from the conditional back to the loop".red()
    );

    // Find the conditional node
    let mut this_conditional_node_index: Option<NodeIndex> = None;
    for node in &nodes {
        if node.node_type == (NodeTypes::Conditional as i32) {
            let _conditional_output_variables = node.output_variables.clone();
            if let Some(node_info) = &node.node_info {
                if let Some(&index) = node_indices.get(&node_info.id) {
                    this_conditional_node_index = Some(index.index().try_into().unwrap());
                    break;
                }
            }
        }
    }

    // for each of the strings contained in conditional_output_variables, check that these are not contained in output_minus_input. For each that is not contained, add it to input_mius_output. These will be the input variables to the process.

    println!(
        "{}: {:?}",
        "The conditional output variables are: ".green(),
        conditional_output_variables
    );

    for conditional_output_variable in conditional_output_variables {
        if !output_minus_input.contains(&conditional_output_variable) {
            input_minus_output.push(conditional_output_variable.clone());
        }
    }

    //
    //output of the process
    // let mut output_minus_input = output_vars
    //     .clone()
    //     .into_iter()
    //     .filter(|output_var| !input_vars.contains(output_var))
    //     .collect::<Vec<String>>();

    // //input of the process
    // let mut input_minus_output = input_vars
    //     .clone()
    //     .into_iter()
    //     .filter(|input_var| !output_vars.contains(input_var))
    //     .collect::<Vec<String>>();

    // Remove the back edges from the conditional node to any node in the loop
    if let Some(conditional_index) = this_conditional_node_index {
        let mut edges_to_remove = Vec::new();
        for edge in graph.edge_references() {
            if u32::try_from(edge.source().index()).unwrap() == conditional_index {
                // Add target index to the list of edges to remove if it creates a cycle
                edges_to_remove.push(edge.id());
            }
        }

        // Now remove the edges from the graph
        for edge_id in edges_to_remove {
            graph.remove_edge(edge_id);
        }
    }

    let top_sort = petgraph::algo::toposort(&graph, None).unwrap();

    let adjacency_list = petgraph::algo::tred::dag_to_toposorted_adjacency_list::<
        _,
        petgraph::graph::DefaultIx,
    >(&graph, &top_sort);

    // The output is the pair of the transitive reduction and the transitive closure.
    let (transative_reduct, _) = petgraph::algo::tred::dag_transitive_reduction_closure::<
        _,
        petgraph::graph::DefaultIx,
    >(&adjacency_list.0);

    // The graph should have the same nodes but different edges.

    // for all of the edges in transivite_reduct, add that edge to the mut_pruned_graph

    transative_reduct.edge_indices().for_each(|edge| {
        // how to get the source and target nodes from the edge?
        let (source_index, target_index) = transative_reduct.edge_endpoints(edge).unwrap();
        mut_pruned_graph.add_edge(source_index.into(), target_index.into(), ());
    });

    let mut new_edges: Vec<Edge> = Vec::new();

    // Count and print the number of edges
    // let edge_count = rebuilt_graph.raw_edges().len();
    // println!("Total number of raw edges: {}", edge_count);

    mut_pruned_graph.raw_edges().iter().for_each(|edge| {
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

    let index_vec = toposort(&mut_pruned_graph, None).unwrap();

    for index in index_vec {
        let node = new_graph
            .nodes_info
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

#[async_recursion]
pub async fn run_execution(
    execution: Execution,
    accumulator: Option<String>,
    docker_id: Option<String>,
    docker_instance: &Docker,
    user_settings: Arc<UserSettings>,
) -> Result<(Execution, Option<String>), Execution> {
    // Keep track of the variable definitions (accumulate their values as we loop through the topological order list)

    let mut variable_definitions: HashMap<String, generated_types::Value> =
        execution.clone().current_variable_definitions;

    let local_nodes: Vec<Node> = execution.process.clone().unwrap().nodes.clone();

    // Make a map out of the vec where the key is the id of the node:
    let mut local_nodes_map: HashMap<String, Node> = HashMap::new();
    local_nodes.iter().for_each(|node: &Node| {
        local_nodes_map.insert(node.node_info.clone().unwrap().id, node.clone());
    });

    let topological_order: Vec<GraphNodeInfo> =
        execution.process.clone().unwrap().topological_order.clone();

    let mut local_accumulator = accumulator.clone();

    let mut prompt_histories: Vec<AtomicExecutionLog> = execution.clone().atomic_history.clone();

    for node_info in topological_order {
        let current_node = local_nodes_map.get(&node_info.id).unwrap().clone();

        match NodeTypes::try_from(current_node.node_type) {
            Ok(NodeTypes::Process) => {
                // Once we implement this functionality just for Prompts (and other node types), we can extract this function and call it recursively to handle this case (with a max depth?)

                let process: Process;

                match current_node.node_content.unwrap().node_content.unwrap() {
                    NodeContentEnum::Process(p) => {
                        process = p;
                    }
                    _ => {
                        println!("Process not handled");
                        continue;
                    }
                }

                let local_execution = process_to_execution(
                    variable_definitions.clone(),
                    process.clone(),
                    prompt_histories.clone(),
                );

                match run_execution(
                    local_execution,
                    local_accumulator.clone(),
                    docker_id.clone(),
                    docker_instance,
                    user_settings.clone(),
                )
                .await
                {
                    Ok((progressed_execution, returned_accumulator)) => {
                        println!("{}", "Process executed successfully".green());
                        // update the variable definitions and prompt histories
                        variable_definitions
                            .extend(progressed_execution.current_variable_definitions.clone());
                        prompt_histories = progressed_execution.atomic_history.clone();

                        local_accumulator = returned_accumulator.clone();
                    }
                    // Err(execution) => {
                    //     return Err(execution);
                    // }
                    _ => {
                        println!("Error at process execution");
                    }
                }
            }
            Ok(NodeTypes::Prompt) => {
                // we need to replace the prompt text input_variables with their definitions
                match handle_prompt(
                    current_node.clone(),
                    variable_definitions.clone(),
                    local_accumulator.clone(),
                    "gpt-4-1106-preview".to_string(),
                    user_settings.clone(),
                )
                .await
                {
                    Ok((prompt_history, local_variable_definitions)) => {
                        prompt_histories.push(prompt_history);
                        // update the variable definitions
                        variable_definitions.extend(local_variable_definitions.clone());
                    }
                    Err(_) => {
                        return Err(execution);
                    }
                }
            }
            Ok(NodeTypes::Loop) => {
                let contained_loop: Loop;

                match current_node.node_content.unwrap().node_content.unwrap() {
                    NodeContentEnum::Loop(looop) => {
                        contained_loop = looop;
                    }
                    _ => {
                        println!("Somehow the stored contents is not actually a loop");
                        continue;
                    }
                }

                // Run the process (marked as a loop so that the aggregator is injected into the promp)

                let max_iterations = contained_loop.max_iterations;

                // run the following loop up to and including max iterations. This
                for _i in 1..max_iterations {
                    // an execution may be returned that contains an external branch (with an empty accumulator) OR the accumulator containing text to feed into the next iteration of the loop

                    let local_execution = process_to_execution(
                        variable_definitions.clone(),
                        contained_loop.clone().process.unwrap().clone(),
                        prompt_histories.clone(),
                    );

                    match run_execution(
                        local_execution,
                        local_accumulator.clone(),
                        docker_id.clone(),
                        docker_instance,
                        user_settings.clone(),
                    )
                    .await
                    {
                        Ok((progressed_execution, returned_accumulator)) => {
                            println!("{}", "Process executed successfully".green());
                            // update the variable definitions and prompt histories
                            variable_definitions
                                .extend(progressed_execution.current_variable_definitions.clone());
                            prompt_histories = progressed_execution.atomic_history.clone();

                            local_accumulator = returned_accumulator.clone();

                            match local_accumulator {
                                None => {
                                    break;
                                }
                                Some(_) => {
                                    continue;
                                }
                            }
                        }
                        Err(execution) => {
                            return Err(execution);
                        }
                    }
                }
            }
            Ok(NodeTypes::Conditional) => {
                // In this case, the main thing we need to do is determine if the loop should continue (by returning the accumulator and the process) OR if it should exit to one of the external branches

                // Check if any of the output_variables of the process containing this conditional are currently defined

                match handle_conditional(
                    current_node.clone(),
                    variable_definitions.clone(),
                    "gpt-4-1106-preview".to_string(),
                )
                .await
                {
                    Ok((prompt_history, local_variable_definitions, accumulator)) => {
                        prompt_histories.push(prompt_history);
                        // update the variable definitions
                        variable_definitions.extend(local_variable_definitions.clone());
                        local_accumulator = accumulator.clone();
                    }
                    Err(_) => {
                        return Err(execution);
                    }
                }

                // For inspiration, a conditional should be handled VERY similarly to a prompt
            }
            Ok(NodeTypes::Command) => {
                let command: Command;

                match current_node
                    .clone()
                    .node_content
                    .unwrap()
                    .node_content
                    .unwrap()
                {
                    NodeContentEnum::Command(c) => {
                        command = c;
                    }
                    _ => {
                        println!("Command not handled");
                        continue;
                    }
                }

                match handle_command(
                    current_node.clone(),
                    variable_definitions.clone(),
                    local_accumulator.clone(),
                    "gpt-4-1106-preview".to_string(),
                    command.clone(),
                    docker_instance,
                    docker_id.clone().unwrap(),
                )
                .await
                {
                    Ok(atomic_log) => {
                        prompt_histories.push(atomic_log);
                    }
                    Err(_err) => {
                        // return Err(execution);
                        println!("Error with running command");
                    }
                }

                // There will be a loop here that looks at the goal, the current output and determines if either: 1) a new command must be run OR 2) the goal has been reached.
            }
            _ => {
                println!("Other types not implemented yet");
                continue;
            }
        }
    }
    let mut response = execution.clone();

    // Change this to use a prompt history
    // response.node_execution_response = node_execution_response;
    response.current_variable_definitions = variable_definitions.clone();
    response.atomic_history = prompt_histories.clone();

    return Ok((response, local_accumulator.clone()));
}

pub fn process_to_execution(
    current_variables: HashMap<String, generated_types::Value>,
    process: Process,
    prompt_histories: Vec<AtomicExecutionLog>,
) -> Execution {
    let execution: Execution = Execution {
        current_variable_definitions: current_variables,
        process: Some(process.clone()),
        current_node: Some(process.clone().topological_order.first().unwrap().clone()),
        atomic_history: prompt_histories,
        execution_id: uuid::Uuid::new_v4().to_string(),
    };

    return execution;
}

pub async fn handle_prompt(
    current_node: Node,
    mut variable_definitions: HashMap<String, generated_types::Value>,
    accumulator: Option<String>,
    language_model_version: String,
    user_settings: Arc<UserSettings>,
) -> Result<(AtomicExecutionLog, HashMap<String, generated_types::Value>), ()> {
    let mut prompt_text: String = "".to_string();
    let mut hydrated_prompt_text: String = "".to_string();

    let api_key = user_settings.openai_api_key.clone();

    let additional_instruction =
        "When coming up with a response, please make the fields of the json response be the following: ".to_string();

    let more_additional_instruction = "
    
    You can also use the error field to report any problems when trying to come up with a response.
    "
    .to_string();

    // Concatenate the strings in the vector to make a comma separated string.

    let variable_string: String = current_node.output_variables.join(", ");

    match current_node.node_content.unwrap().node_content.unwrap() {
        NodeContentEnum::Prompt(prompt) => {
            // hydrate the prompt text with the variable definitions

            let mut handlebars = Handlebars::new();

            let string_map = convert_to_string_map(variable_definitions.clone());

            let json_variable_definitions: serde_json::Value = serde_json::json!(string_map);

            handlebars
                .register_template_string("prompt", prompt.clone().prompt)
                .unwrap();

            hydrated_prompt_text = handlebars
                .render("prompt", &json_variable_definitions)
                .unwrap();

            match accumulator {
                Some(accumulator_text) => {
                    prompt_text = format!(
                        "{}\n{}\n {} {} {}",
                        hydrated_prompt_text.clone(),
                        accumulator_text.clone(),
                        additional_instruction,
                        variable_string,
                        more_additional_instruction
                    );
                }
                None => {
                    prompt_text = format!(
                        "{} {} {} {}",
                        hydrated_prompt_text.clone(),
                        additional_instruction,
                        variable_string,
                        more_additional_instruction
                    );
                }
            }

            print!("Prompt text: {}", prompt_text.green());
        }
        _ => {
            println!("prompt not handled");
        }
    }

    let config = OpenAIConfig::new().with_api_key(api_key);

    let client = Client::with_config(config);

    let user_message = ChatCompletionRequestUserMessage {
        content: Some(ChatCompletionRequestUserMessageContent::Text(prompt_text)),
        role: Role::User,
    };

    let message: ChatCompletionRequestMessage = ChatCompletionRequestMessage::User(user_message);

    // Create request using builder pattern
    // Every request struct has companion builder struct with same name + Args suffix
    let mut request = CreateChatCompletionRequest::default();

    request.messages = vec![message];
    request.response_format = Some(ChatCompletionResponseFormat {
        r#type: ChatCompletionResponseFormatType::JsonObject,
    });

    // Any model can be used so long as it supports response_format
    // request.model = "gpt-4-1106-preview".to_string();
    request.model = language_model_version.clone();

    // Call API
    let response = match client.chat().create(request).await {
        Ok(response) => response,
        Err(_) => {
            println!("{}", "Error with OpenAI API".red());
            return Err(());
        }
    };

    let json_string = response
        .choices
        .first()
        .unwrap()
        .message
        .content
        .clone()
        .unwrap()
        .as_str()
        .to_string();

    let node_info = current_node.node_info.clone().unwrap();

    let hydrated_and_cleaned_prompt_text = clean_response(&hydrated_prompt_text);

    let mut execution_response_hashmap = HashMap::new();

    let value: serde_json::Value = serde_json::from_str(json_string.as_str()).unwrap();

    println!("{}", "VARIABLES RETURNED FROM PROMPT:".green());

    if let Some(obj) = value.as_object() {
        for (key, value) in obj {
            let mut return_val: crate::generated_types::Value = crate::generated_types::Value {
                value_type: Some(value::ValueType::StringValue(
                    "err: uninitialized value".to_string(),
                )),
            };
            match value {
                serde_json::Value::String(s) => {
                    return_val = crate::generated_types::Value {
                        value_type: Some(value::ValueType::StringValue(s.clone())),
                    };
                }
                serde_json::Value::Number(n) => {
                    let return_num = n.as_f64().unwrap();

                    return_val = crate::generated_types::Value {
                        value_type: Some(value::ValueType::NumberValue(return_num)),
                    };
                }
                serde_json::Value::Array(arr) => {
                    let string_list = crate::generated_types::StringList {
                        values: arr.clone().into_iter().map(|v| v.to_string()).collect(),
                    };

                    return_val = crate::generated_types::Value {
                        value_type: Some(value::ValueType::StringList(string_list)),
                    };
                }
                // Handle other types as needed
                _ => {
                    value.to_string();
                }
            };

            execution_response_hashmap.insert(key.clone(), return_val.clone());

            variable_definitions.insert(key.clone(), return_val.clone());
        }
    } else {
        println!("{}", "The JSON is not an object.".red());
        return Err(());
    }

    // see if all of the output_variables of the node are in the variable definition hashmap:

    let check_output_vars: Vec<String> = current_node.output_variables;

    //loop through check_output_vars and see if the key exists in the variable_definitions hashmap:

    for output_var in check_output_vars.iter() {
        if !variable_definitions.contains_key(output_var) {
            println!("Missing variable: {}", output_var.red());
            // Handle the missing variable case here (e.g., error handling or logging)

            // Send back the Execution to the frontend and let the user decide what to do.

            return Err(());
        }
    }

    let prompt_history = AtomicExecutionLog {
        prompt: hydrated_and_cleaned_prompt_text.clone(),
        response: execution_response_hashmap.clone(),
        node_info: Some(node_info.clone()),
    };

    return Ok((prompt_history, variable_definitions));
}

fn convert_to_string_map(
    variable_definitions: HashMap<String, generated_types::Value>,
) -> HashMap<String, String> {
    let mut string_map: HashMap<String, String> = HashMap::new();

    for (key, value) in variable_definitions {
        match value.value_type.unwrap() {
            value::ValueType::StringValue(s) => {
                string_map.insert(key, s);
            }
            value::ValueType::NumberValue(n) => {
                string_map.insert(key, n.to_string());
            }
            value::ValueType::StringList(string_list) => {
                let string_list_vec = string_list.values;
                let string_list_string = string_list_vec.join(", ");
                string_map.insert(key, string_list_string);
            }
        }
    }

    return string_map;
}

pub async fn handle_command(
    current_node: Node,
    mut variable_definitions: HashMap<String, generated_types::Value>,
    accumulator: Option<String>,
    language_model_version: String,
    command: Command,
    docker_instance: &Docker,
    docker_id: String,
) -> Result<AtomicExecutionLog, ()> {
    let mut prompt_text: String = "".to_string();

    let mut goal = command.goal.clone();

    let mut handlebars = Handlebars::new();

    let string_map = convert_to_string_map(variable_definitions.clone());

    let json_variable_definitions: serde_json::Value = serde_json::json!(string_map);

    handlebars
        .register_template_string("goal", goal.clone())
        .unwrap();

    goal = handlebars
        .render("goal", &json_variable_definitions)
        .unwrap();

    // let the command_line_history string be empty OR the contents of the accumulator:
    let command_line_history: String = match accumulator {
        Some(accumulator_text) => accumulator_text.clone(),
        None => "".to_string(),
    };

    prompt_text = format!(
        "Please write a command line command (assuming a recent debian based operating system) that fulfills the following goal given the current command line log.\n Goal: {}\nCommand Line Log: {}\n (If the command line log is empty then just give a command that tries to achieve the goal.\nPlease also come up with a corresponding command that can be run to verify that the first command succeeded in fulfilling the goal, it should print something out to the command line to indicate success. When coming up with a response, please make the fields of the json response be the following:\n command, verification_command",
        goal,
        command_line_history
    );

    // I believe this already pulls the key from the environmental variable.
    let client = Client::new();

    let user_message = ChatCompletionRequestUserMessage {
        content: Some(ChatCompletionRequestUserMessageContent::Text(
            prompt_text.clone(),
        )),
        role: Role::User,
    };

    let message: ChatCompletionRequestMessage = ChatCompletionRequestMessage::User(user_message);

    // Create request using builder pattern
    // Every request struct has companion builder struct with same name + Args suffix
    let mut request = CreateChatCompletionRequest::default();

    request.messages = vec![message];
    request.response_format = Some(ChatCompletionResponseFormat {
        r#type: ChatCompletionResponseFormatType::JsonObject,
    });

    // Any model can be used so long as it supports response_format
    // request.model = "gpt-4-1106-preview".to_string();
    request.model = language_model_version.clone();

    // Call API
    let response = client.chat().create(request).await.unwrap();

    println!(
        "{}",
        response
            .choices
            .first()
            .unwrap()
            .message
            .content
            .clone()
            .unwrap()
            .as_str()
            .to_string()
    );

    let json_string = response
        .choices
        .first()
        .unwrap()
        .message
        .content
        .clone()
        .unwrap()
        .as_str()
        .to_string();

    let node_info = current_node.node_info.clone().unwrap();

    // let hydrated_and_cleaned_prompt_text = clean_response(&hydrated_prompt_text);

    let mut execution_response_hashmap = HashMap::new();

    let value: serde_json::Value = serde_json::from_str(json_string.as_str()).unwrap();

    if let Some(obj) = value.as_object() {
        for (key, value) in obj {
            let mut return_val: crate::generated_types::Value = crate::generated_types::Value {
                value_type: Some(value::ValueType::StringValue(
                    "err: uninitialized value".to_string(),
                )),
            };
            match value {
                serde_json::Value::String(s) => {
                    return_val = crate::generated_types::Value {
                        value_type: Some(value::ValueType::StringValue(s.clone())),
                    };
                }
                serde_json::Value::Number(n) => {
                    let return_num = n.as_f64().unwrap();

                    return_val = crate::generated_types::Value {
                        value_type: Some(value::ValueType::NumberValue(return_num)),
                    };
                }
                serde_json::Value::Array(arr) => {
                    let string_list = crate::generated_types::StringList {
                        values: arr.clone().into_iter().map(|v| v.to_string()).collect(),
                    };

                    return_val = crate::generated_types::Value {
                        value_type: Some(value::ValueType::StringList(string_list)),
                    };
                }
                // Handle other types as needed
                _ => {
                    value.to_string();
                }
            };

            execution_response_hashmap.insert(key.clone(), return_val.clone());

            variable_definitions.insert(key.clone(), return_val.clone());
        }
    } else {
        println!("{}", "The JSON is not an object.".red());
        return Err(());
    }

    // if let Some(obj) = value.as_object() {
    //     for (key, value) in obj {
    //         let value_str = match value {
    //             serde_json::Value::String(s) => {
    //                 let return_val = crate::generated_types::Value {
    //                     value_type: Some(value::ValueType::StringValue(s.clone())),
    //                 };

    //                 execution_response_hashmap.insert(key.clone(), return_val);
    //                 s.clone()
    //             }
    //             serde_json::Value::Number(n) => {
    //                 let return_num = n.as_f64().unwrap();

    //                 let return_val = crate::generated_types::Value {
    //                     value_type: Some(value::ValueType::NumberValue(return_num)),
    //                 };

    //                 execution_response_hashmap.insert(key.clone(), return_val);
    //                 n.to_string()
    //             }
    //             serde_json::Value::Array(arr) => {
    //                 let string_list = crate::generated_types::StringList {
    //                     values: arr.clone().into_iter().map(|v| v.to_string()).collect(),
    //                 };

    //                 let return_val = crate::generated_types::Value {
    //                     value_type: Some(value::ValueType::StringList(string_list)),
    //                 };

    //                 // Convert the array to a string representation
    //                 // You can customize this part as per your requirements
    //                 execution_response_hashmap.insert(key.clone(), return_val);

    //                 serde_json::to_string(&arr).unwrap_or_else(|_| "[]".to_string())
    //             }
    //             // Handle other types as needed
    //             _ => value.to_string(),
    //         };

    //         println!("{}: {}", key.green(), value_str);
    //         variable_definitions.insert(key.clone(), value_str.clone());
    //     }
    // } else {
    //     println!("{}", "The JSON is not an object.".red());
    //     return Err(());
    // }

    // extract the command from the variable_definitions hashmap:
    let mut run_this_command: String = "".to_string();
    let mut verification_command: String = "".to_string();

    let string_map = convert_to_string_map(variable_definitions.clone());

    match string_map.get("command") {
        Some(command) => {
            run_this_command = command.clone();
        }
        None => {
            println!(
                "{}",
                "The command field was not found in the response.".red()
            );
            return Err(());
        }
    }

    match string_map.get("verification_command") {
        Some(command) => {
            verification_command = command.clone();
        }
        None => {
            println!(
                "{}",
                "The command field was not found in the response.".red()
            );
            return Err(());
        }
    }

    println!(
        "Running the following command: '{}'",
        run_this_command.green()
    );

    match run_command(run_this_command.clone(), docker_id.clone(), docker_instance).await {
        Ok(res) => {
            println!(
                "{}\nResult: {:?}",
                "The command was run successfully".green(),
                res
            );

            let return_value = crate::generated_types::Value {
                value_type: Some(value::ValueType::StringValue(res.clone())),
            };

            // Need to add the command to the atomic history of the execution_details

            execution_response_hashmap.insert("command_response".to_string(), return_value.clone());
        }
        Err(_err) => {
            println!("{}", "The command was not run successfully".red());
        }
    }

    // run the verfication command here:
    match run_command(verification_command.clone(), docker_id, docker_instance).await {
        Ok(verification_res) => {
            println!(
                "{}\nResult: {:?}",
                "The verification command was run successfully".green(),
                verification_res
            );

            let return_value = crate::generated_types::Value {
                value_type: Some(value::ValueType::StringValue(verification_res.clone())),
            };

            execution_response_hashmap.insert(
                "verification_command_response".to_string(),
                return_value.clone(),
            );
        }
        Err(err) => {
            println!("{}: ", "Error verifying command".red());
        }
    }

    // if successful, we should re

    let prompt_history = AtomicExecutionLog {
        prompt: prompt_text.clone(),
        response: execution_response_hashmap.clone(),
        node_info: Some(node_info.clone()),
    };

    return Ok(prompt_history);
}

pub async fn handle_conditional(
    current_node: Node,
    mut variable_definitions: HashMap<String, generated_types::Value>,
    _language_model_version: String,
) -> Result<
    (
        AtomicExecutionLog,
        HashMap<String, generated_types::Value>,
        Option<String>,
    ),
    (),
> {
    let mut prompt_text: String = "".to_string();
    let mut hydrated_prompt_text: String = "".to_string();

    let additional_instruction = "

        When coming up with a response, please make the fields be any number of the following: "
        .to_string();

    let more_additional_instruction =
        "
    
    If there is not enough information, you should put an explanation of what information is needed in the \"accumulator\" field. If you respond with the accumulator field, do NOT respond with any other field.
    
    If there is absolutely no way You can also use the \"error\" field to report any problems when trying to come up with a response.
    ".to_string();

    // Concatenate the strings in the vector to make a comma separated string.

    let variable_string: String = current_node.output_variables.join(", ");

    match current_node.node_content.unwrap().node_content.unwrap() {
        NodeContentEnum::Prompt(prompt) => {
            // hydrate the prompt text with the variable definitions

            let mut handlebars = Handlebars::new();

            let string_map = convert_to_string_map(variable_definitions.clone());

            let json_variable_definitions: serde_json::Value = serde_json::json!(string_map);

            handlebars
                .register_template_string("prompt", prompt.clone().prompt)
                .unwrap();

            hydrated_prompt_text = handlebars
                .render("prompt", &json_variable_definitions)
                .unwrap();

            prompt_text = format!(
                "{} {} {} {}",
                hydrated_prompt_text.clone(),
                additional_instruction,
                variable_string,
                more_additional_instruction
            );

            print!("Prompt text: {}", prompt_text.green());
        }
        _ => {
            println!("prompt not handled");
        }
    }

    // I believe this already pulls the key from the environmental variable.
    let client = Client::new();

    let user_message = ChatCompletionRequestUserMessage {
        content: Some(ChatCompletionRequestUserMessageContent::Text(prompt_text)),
        role: Role::User,
    };

    let message: ChatCompletionRequestMessage = ChatCompletionRequestMessage::User(user_message);

    // Create request using builder pattern
    // Every request struct has companion builder struct with same name + Args suffix
    let mut request = CreateChatCompletionRequest::default();

    request.messages = vec![message];
    request.response_format = Some(ChatCompletionResponseFormat {
        r#type: ChatCompletionResponseFormatType::JsonObject,
    });

    // Any model can be used so long as it supports response_format
    request.model = "gpt-4-1106-preview".to_string();

    // Call API
    let response = client.chat().create(request).await.unwrap();

    println!(
        "{}",
        response
            .choices
            .first()
            .unwrap()
            .message
            .content
            .clone()
            .unwrap()
            .as_str()
            .to_string()
    );

    let json_string = response
        .choices
        .first()
        .unwrap()
        .message
        .content
        .clone()
        .unwrap()
        .as_str()
        .to_string();

    let node_info = current_node.node_info.clone().unwrap();

    let hydrated_and_cleaned_prompt_text = clean_response(&hydrated_prompt_text);

    let mut execution_response_hashmap = HashMap::new();

    let value: serde_json::Value = serde_json::from_str(json_string.as_str()).unwrap();
    // if let Some(obj) = value.as_object() {
    //     for (key, value) in obj {
    //         println!("{}: {}", key, value);

    //         let return_value = crate::generated_types::Value {
    //             value_type: Some(value::ValueType::StringValue(value.clone().to_string())),
    //         };

    //         variable_definitions.insert(key.clone(), value.clone().to_string());
    //         execution_response_hashmap.insert(key.clone(), return_value);
    //     }
    // } else {
    //     println!("{}", "The JSON is not an object.".red());
    // }

    if let Some(obj) = value.as_object() {
        for (key, value) in obj {
            let mut return_val: crate::generated_types::Value = crate::generated_types::Value {
                value_type: Some(value::ValueType::StringValue(
                    "err: uninitialized value".to_string(),
                )),
            };
            match value {
                serde_json::Value::String(s) => {
                    return_val = crate::generated_types::Value {
                        value_type: Some(value::ValueType::StringValue(s.clone())),
                    };
                }
                serde_json::Value::Number(n) => {
                    let return_num = n.as_f64().unwrap();

                    return_val = crate::generated_types::Value {
                        value_type: Some(value::ValueType::NumberValue(return_num)),
                    };
                }
                serde_json::Value::Array(arr) => {
                    let string_list = crate::generated_types::StringList {
                        values: arr.clone().into_iter().map(|v| v.to_string()).collect(),
                    };

                    return_val = crate::generated_types::Value {
                        value_type: Some(value::ValueType::StringList(string_list)),
                    };
                }
                // Handle other types as needed
                _ => {
                    value.to_string();
                }
            };

            execution_response_hashmap.insert(key.clone(), return_val.clone());

            variable_definitions.insert(key.clone(), return_val.clone());
        }
    } else {
        println!("{}", "The JSON is not an object.".red());
        return Err(());
    }

    // see if all of the output_variables of the node are in the variable definition hashmap:

    let check_output_vars: Vec<String> = current_node.output_variables;

    //loop through check_output_vars and see if the key exists in the variable_definitions hashmap:

    for output_var in check_output_vars.iter() {
        if !variable_definitions.contains_key(output_var) {
            println!("Missing variable: {}", output_var.red());
            // Handle the missing variable case here (e.g., error handling or logging)

            // Send back the Execution to the frontend and let the user decide what to do.

            return Err(());
        }
    }

    // check to see if the execution_response contains "accumulator"

    let mut accumulator = String::new();

    match execution_response_hashmap
        .get("accumulator")
        .unwrap()
        .clone()
        .value_type
        .unwrap()
    {
        value::ValueType::StringValue(value) => {
            accumulator = value.clone();
        }
        _ => {
            println!(
                "{}",
                "The accumulator field was not found in the response.".red()
            );
        }
    }

    let prompt_history = AtomicExecutionLog {
        prompt: hydrated_and_cleaned_prompt_text.clone(),
        response: execution_response_hashmap.clone(),
        node_info: Some(node_info.clone()),
    };

    return Ok((
        prompt_history,
        variable_definitions,
        Some(accumulator.clone()),
    ));
}

fn clean_response(input: &str) -> String {
    input
        .replace("\\n", "") // Removes all newlines
        .replace("\\\"", "\"") // Replaces escaped double quotes with regular double quotes
        .replace("&quot;", "\"") // Replaces HTML entity &quot; with double quotes
}
async fn run_command(
    command: String,
    docker_id: String,
    docker_instance: &Docker,
) -> Result<String, String> {
    println!("Preparing to run command: {}", command);

    // strip any '"' characters from the command string
    let command = command.replace("\"", "");

    let exec_options = CreateExecOptions {
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        cmd: Some(vec!["sh", "-c", &command]),
        ..Default::default()
    };

    println!("The docker id is: {:?}", docker_id);

    let exec_created = match docker_instance.create_exec(&docker_id, exec_options).await {
        Ok(exec) => {
            println!("Exec instance created successfully.");
            exec
        }
        Err(e) => {
            println!("Failed to create exec instance: {:?}", e);
            return Err(format!("Failed to create exec instance: {}", e));
        }
    };

    println!("Starting the exec instance.");
    let exec_started = match docker_instance.start_exec(&exec_created.id, None).await {
        Ok(result) => {
            println!("Exec instance started.");
            result
        }
        Err(e) => {
            println!("Failed to start exec instance: {:?}", e);
            return Err(format!("Failed to start exec instance: {}", e));
        }
    };

    match exec_started {
        StartExecResults::Attached { mut output, .. } => {
            println!("Processing output from the exec instance.");
            let mut full_output = String::new();

            while let Some(item) = output.next().await {
                match item {
                    Ok(log) => {
                        match log {
                            LogOutput::StdOut { message } => {
                                if let Ok(str_message) = String::from_utf8(message.to_vec()) {
                                    println!("Received StdOut: {}", str_message);
                                    full_output.push_str(&str_message);
                                } else {
                                    println!("Received non-UTF8 StdOut data");
                                }
                            }
                            LogOutput::StdErr { message } => {
                                if let Ok(str_message) = String::from_utf8(message.to_vec()) {
                                    println!("Received StdErr: {}", str_message);
                                    full_output.push_str(&str_message);
                                } else {
                                    println!("Received non-UTF8 StdErr data");
                                }
                            }
                            _ => {} // Handle other types of LogOutput if necessary
                        }
                    }
                    Err(e) => {
                        println!("Error during execution: {:?}", e);
                        return Err(format!("Error during execution: {}", e));
                    }
                }
            }

            println!("Command execution completed.");
            Ok(full_output)
        }
        StartExecResults::Detached => {
            println!("The exec instance completed execution and detached.");
            Err("The exec instance completed execution and detached".to_string())
        }
    }
}
