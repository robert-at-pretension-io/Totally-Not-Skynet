use crate::generated_types::{ variable_definition, PromptHistory };
use crate::generated_types::{
    node_content::NodeContent as NodeContentEnum,
    Edge,
    Graph,
    GraphNodeInfo,
    Node,
    NodeContent,
    Process,
    Execution,
    NodeTypes,
};

use async_recursion::async_recursion;

use async_openai::types::{
    Prompt,
    ChatCompletionRequestUserMessage,
    CreateChatCompletionRequest,
    ChatCompletionRequestUserMessageContent,
    Role,
    ChatCompletionRequestMessage,
    ChatCompletionResponseFormat,
    ChatCompletionResponseFormatType,
};

use handlebars::Handlebars;

use async_openai::Client;
use colored::*;

use petgraph::{ graph::DiGraph, Direction };

use petgraph::algo::toposort;

use std::collections::HashMap;

use serde_json::Value;

// use anyhow::Error;

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
        let (source_index, target_index) = transative_reduct.edge_endpoints(edge).unwrap();
        mut_pruned_graph.add_edge(source_index.into(), target_index.into(), ());
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

#[async_recursion]
pub async fn run_execution(mut execution: Execution) -> Result<Execution, Execution> {
    // Keep track of the variable definitions (accumulate their values as we loop through the topological order list)

    let mut variable_definitions: HashMap<
        String,
        String
    > = execution.clone().current_variable_definitions;

    let local_nodes: Vec<Node> = execution.process.clone().unwrap().nodes.clone();

    // Make a map out of the vec where the key is the id of the node:
    let mut local_nodes_map: HashMap<String, Node> = HashMap::new();
    local_nodes.iter().for_each(|node: &Node| {
        local_nodes_map.insert(node.node_info.clone().unwrap().id, node.clone());
    });

    let topological_order: Vec<GraphNodeInfo> = execution.process
        .clone()
        .unwrap()
        .topological_order.clone();

    let mut prompt_histories: Vec<PromptHistory> = execution.clone().prompt_history;

    for node_info in topological_order {
        let current_node = local_nodes_map.get(&node_info.id).unwrap().clone();

        // Process the node, which ever type it is:

        // let process = NodeTypes::Process as i32;
        // let prompt = NodeTypes::Prompt as i32;

        match NodeTypes::try_from(current_node.node_type) {
            Ok(NodeTypes::Process) => {
                // Once we implement this functionality just for Prompts (and other node types), we can extract this function and call it recursively to handle this case (with a max depth?)

                let mut process: Process;

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
                    prompt_histories.clone()
                );

                match run_execution(local_execution).await {
                    Ok(progressed_execution) => {
                        println!("{}", "Process executed successfully".green());
                        // update the variable definitions and prompt histories
                        variable_definitions.extend(
                            progressed_execution.current_variable_definitions.clone()
                        );
                        prompt_histories = progressed_execution.prompt_history.clone();
                    }
                    Err(execution) => {
                        return Err(execution);
                    }
                }
            }
            Ok(NodeTypes::Prompt) => {
                // we need to replace the prompt text input_variables with their definitions
                match handle_prompt(current_node.clone(), variable_definitions.clone()).await {
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
    response.prompt_history = prompt_histories.clone();

    return Ok(response);
}

pub fn process_to_execution(
    current_variables: HashMap<String, String>,
    process: Process,
    prompt_histories: Vec<PromptHistory>
) -> Execution {
    let execution: Execution = Execution {
        current_variable_definitions: current_variables,
        process: Some(process.clone()),
        current_node: Some(process.clone().topological_order.first().unwrap().clone()),
        prompt_history: prompt_histories,
        execution_id: uuid::Uuid::new_v4().to_string(),
    };

    return execution;
}

pub async fn handle_prompt(
    current_node: Node,
    mut variable_definitions: HashMap<String, String>
) -> Result<(PromptHistory, HashMap<String, String>), ()> {
    let mut prompt_text: String = "".to_string();
    let mut hydrated_prompt_text: String = "".to_string();

    let additional_instruction =
        "When coming up with a response, please make the fields of the json response be the following: ".to_string();

    let more_additional_instruction =
        "
    
    You can also use the error field to report any problems when trying to come up with a response.
    ".to_string();

    // Concatenate the strings in the vector to make a comma separated string.

    let variable_string: String = current_node.output_variables.join(", ");

    match current_node.node_content.unwrap().node_content.unwrap() {
        NodeContentEnum::Prompt(prompt) => {
            // hydrate the prompt text with the variable definitions

            let mut handlebars = Handlebars::new();

            let json_variable_definitions: Value = serde_json::json!(variable_definitions);

            handlebars.register_template_string("prompt", prompt.clone().prompt).unwrap();

            hydrated_prompt_text = handlebars.render("prompt", &json_variable_definitions).unwrap();

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
        response.choices.first().unwrap().message.content.clone().unwrap().as_str().to_string()
    );

    let json_string = response.choices
        .first()
        .unwrap()
        .message.content.clone()
        .unwrap()
        .as_str()
        .to_string();

    let node_info = current_node.node_info.clone().unwrap();

    let hydrated_and_cleaned_prompt_text = clean_response(&hydrated_prompt_text);

    let mut execution_response_hashmap = HashMap::new();

    let value: Value = serde_json::from_str(json_string.as_str()).unwrap();
    if let Some(obj) = value.as_object() {
        for (key, value) in obj {
            println!("{}: {}", key, value);
            variable_definitions.insert(key.clone(), value.clone().to_string());
            execution_response_hashmap.insert(key.clone(), value.clone().to_string());
        }
    } else {
        println!("{}", "The JSON is not an object.".red());
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

    let prompt_history = PromptHistory {
        prompt: hydrated_and_cleaned_prompt_text.clone(),
        response: execution_response_hashmap.clone(),
        node_info: Some(node_info.clone()),
    };

    return Ok((prompt_history, variable_definitions));
}

fn clean_response(input: &str) -> String {
    input
        .replace("\\n", "") // Removes all newlines
        .replace("\\\"", "\"") // Replaces escaped double quotes with regular double quotes
        .replace("&quot;", "\"") // Replaces HTML entity &quot; with double quotes
}
