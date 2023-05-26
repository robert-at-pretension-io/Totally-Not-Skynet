import type {
  GraphState,
  AiSystemState,
  SystemState,
  Action,
} from "../system_types";
import {graphStore} from "../stores/graphStore";
import {Process} from "../system_types";
import {aiSystemStore} from "../stores/aiSystemStore";
import systemStateStore from "stores/systemStateStore";
import {Graph} from "graphlib";
import {Edge} from "@dagrejs/graphlib";

// Define the getter and setter

export async function getGraphState(): Promise<GraphState> {
  return new Promise((resolve, _) => {
    graphStore.subscribe((graphState: GraphState) => {
      resolve(graphState);
    });
  });
}

export async function getAiSystemState(): Promise<AiSystemState> {
  return new Promise((resolve, _) => {
    aiSystemStore.subscribe((aiSystemState: AiSystemState) => {
      resolve(aiSystemState);
    });
  });
}

export async function getAllInputVariables() : Promise<string[]>{
  let graph_state = await getGraphState();
  let graph = graph_state.graph;
  let input_variables = []
  // for each node in the graph
  let nodes = graph.nodes();
  for (let node of nodes){
    let input = await returnInputVariablesOfNode(node)
    input_variables.push(...input)
  }
  return input_variables
}

export async function getAllOutputVariables() : Promise<string[]>{
  let graph_state = await getGraphState();
  let graph = graph_state.graph;
  let output_variables = []
  // for each node in the graph
  let nodes = graph.nodes();
  for (let node of nodes){
    let output = await returnOutputVariablesOfNode(node)
    output_variables.push(...output)
  }
  return output_variables
}


export async function returnInputVariablesOfNode(node_id : string) {
  let ai_system_state = await getAiSystemState();
  // loop through the actions
  for (let i = 0; i++; i< ai_system_state.actions.length) {
    let current_action = ai_system_state.actions[i];
    // if the node_id matches the id of the action then return the input_variables
    if (current_action._id.$oid == node_id) {
      return current_action.input_variables
    }
  }
  return []
}

export async function returnOutputVariablesOfNode(node_id : string) {
  let ai_system_state = await getAiSystemState();
  // loop through the actions
  for (let i = 0; i++; i< ai_system_state.actions.length) {
    let current_action = ai_system_state.actions[i];
    // if the node_id matches the id of the action then return the input_variables
    if (current_action._id.$oid == node_id) {
      return current_action.output_variables
    }
  }
  console.log("the node/action wasn't found");
  return []
}

export async function returnNodeWithMatchingInputVariable(graph: Graph, input_variable: string) {
  let nodes = graph.nodes();

  // loop through nodes
  for (let i=0; i++; i < nodes.length) {
    let test_node_id = nodes[i];
    let test_input_variables = await returnInputVariablesOfNode(test_node_id);
    if (test_input_variables.indexOf(input_variable) != -1){
      return test_node_id;
    }
  }

  return null
}

export async function returnNodeWithMatchingOutputVariable(graph: Graph, input_variable: string) {
  let nodes = graph.nodes();

  // loop through nodes
  for (let i=0; i++; i < nodes.length) {
    let test_node_id = nodes[i];
    let test_input_variables = await returnOutputVariablesOfNode(test_node_id);
    if (test_input_variables.indexOf(input_variable) != -1){
      return test_node_id;
    }
  }

  return null
}

// get the name of the action by using the id
export async function getNodeName(id: string): Promise<string | undefined> {
  const res: AiSystemState = await new Promise((resolve, _) => {
    aiSystemStore.subscribe((aiSystemState: AiSystemState) => {
      resolve(aiSystemState);
    });
  });
  const action = await res.actions.find(action => {
    return getId(action) == id;
  });
  if (action) {
    // console.log("action name: " + action.name);
    return action.name;
  }
}

export function getId(actionOrProcess: Process | Action): string {
  return actionOrProcess._id.$oid;
}

export async function setGraphState(graphState: GraphState) {
  let input_variables = await getAllInputVariables();
  let output_variables = await getAllOutputVariables();
  graphState.input_variables = input_variables;
  graphState.output_variables = output_variables;
  console.log("The graphstate is:\n ", graphState)
  graphStore.set(graphState);
}

export async function addGlobalVariable(variable_name : string) {
  let current_state = await getGraphState();
  current_state.global_variables.push(variable_name);
  await setGraphState(current_state);
}

export async function addNode(node_id: string): Promise<void> {
  const graphState = await getGraphState();
  // add the input and output variables to the graph state
  
  graphState.graph.setNode(node_id);
  graphState.lastAction = "addNode";
  const node_name = await getNodeName(node_id);
  if (node_name) {
    graphState.name = node_name;
    graphState.actedOn = [node_id, node_name];
  } else {
    graphState.actedOn = [node_id, ""];
  }
  setGraphState(graphState);
}

// function for converting a process to a graph
export async function processToGraph(process: Process): Promise<void> {
  await resetGraph();

  // verify that all of the steps have corresponding actions
  const graph = process.graph;
  const nodes = graph.nodes();

  console.log("nodes: ", nodes);

  // for each of the node ids stored in nodes, get the name of the action

  //loop through the nodes
  for (let i = 0; i < nodes.length; i++) {
    const name = await getNodeName(nodes[i]);
    if (name) {
      await addNode(nodes[i]);
    }
  }

  const my_edges = graph.edges();

  console.log("edges: ", my_edges);

  //loop through the edges
  for (let i = 0; i < my_edges.length; i++) {
    await addEdge(my_edges[i]);
  }
}

export async function addEdge(edge: Edge): Promise<void> {
  const graphState = await getGraphState();
  graphState.graph.setEdge(edge);
  graphState.lastAction = "addEdge";
  graphState.actedOn = edge;
  setGraphState(graphState);
}

export async function removeNode(id: string): Promise<void> {
  const name = await getNodeName(id);
  const graphState = await getGraphState();
  graphState.graph.removeNode(id);
  graphState.lastAction = "removeNode";
  if (name) {
    graphState.actedOn = [id, name];
  } else {
    graphState.actedOn = [id, "unknown"];
  }
  setGraphState(graphState);
}

export async function removeSelectedNode(): Promise<void> {
  const graphState = await getGraphState();
  if (Array.isArray(graphState.actedOn)) {
    const selected = graphState.actedOn[0];
    await removeNode(selected);
  }
}

export async function removeSelectedEdge(): Promise<void> {
  const graphState = await getGraphState();
  if (
    !Array.isArray(graphState.actedOn) &&
    graphState.lastAction == "selectEdge"
  ) {
    const selected = graphState.actedOn;
    if (selected != null) {
      await removeEdge(selected.v, selected.w);
    }
  } else {
    // console.log("not removing edge, doesn't meet criteria");
  }
}

export async function removeEdge(
  sourceId: string,
  targetId: string
): Promise<void> {
  const graphState = await getGraphState();
  // find the id of the edge to remove

  // console.log("removing edge:", sourceId, targetId, " from graph");

  const edge = graphState.actedOn;
  // graphState.graph.removeEdge(edge);

  graphState.lastAction = "removeEdge";
  graphState.actedOn = edge;
  graphState.name = null;
  setGraphState(graphState);
}

export async function selectNode(id: string): Promise<void> {
  const ai_system_state = await getAiSystemState();
  const actions = ai_system_state.actions;
  let specific_action: Action;

  const res = actions.find(action => {
    return getId(action) == id;
  });
  if (res) {
    specific_action = res;

    systemStateStore.update((system_state: SystemState) => {
      // Return a new SystemState object with the updated selectedAction property
      return {
        ...system_state,
        selectedAction: specific_action,
        currentlySelected: "action",
      };
    });

    const graphState = await getGraphState();

    graphState.lastAction = "selectNode";
    graphState.lastActedOn = graphState.actedOn;
    graphState.actedOn = [id, specific_action.name];
    graphState.name = specific_action.name;
    setGraphState(graphState);
  }
}

export async function selectEdge(
  source: string,
  target: string
): Promise<void> {
  const graphState = await getGraphState();

  graphState.lastAction = "selectEdge";
  graphState.actedOn = {v: source, w: target};
  graphState.name = null;
  setGraphState(graphState);
}

export async function resetLastAction(): Promise<void> {
  const graphState = await getGraphState();
  graphState.lastAction = "none";
  graphState.actedOn = null;
  setGraphState(graphState);
}

export async function nodes(): Promise<string[]> {
  const graphState = await getGraphState();
  return graphState.graph.nodes();
}

export async function edges(): Promise<Edge[]> {
  const graphState = await getGraphState();
  return graphState.graph.edges();
}

// reset the graphState to a new empty graph
export async function resetGraph(): Promise<void> {
  console.log("resetting graph");
  const graphState = await getGraphState();
  graphState.graph = new Graph();
  graphState.lastAction = "resetGraph";
  graphState.actedOn = null;
  graphState.name = null;
  setGraphState(graphState);
}
