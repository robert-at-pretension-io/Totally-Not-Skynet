import type {
  SystemState,
  AiSystemState,
  Prompt,
  NodeType
} from "../system_types";
import { Process } from "../system_types";
import systemStateStore from "stores/systemStateStore";
import { Graph } from "graphlib";
import { Edge } from "@dagrejs/graphlib";
import { alg } from "graphlib";

// Define the getter and setter

export async function getSystemState(): Promise<SystemState> {
  return new Promise((resolve, _rej) => {
    systemStateStore.subscribe((systemStateStore) => {
      resolve(systemStateStore);
    });
  });
}

export async function getInputVariablesByNodeId(nodeId: string): Promise<string[] | null> {
  // Get the action by ID
  const prompt = await getPromptById(nodeId);

  // If action exists, return its input variables; else, return null
  return prompt ? prompt.prompt.input_variables : null;
}

export function getGlobalVariableNames() {
  let globalVariableNames: string[] = [];
  systemStateStore.subscribe(store => {
    globalVariableNames = Array.from(store.graphState.global_variables.keys());
  })();
  return globalVariableNames;
}

export async function getAncestorNodes(node: string, graph: Graph): Promise<Prompt[]> {
  const ancestors: Prompt[] = [];
  const visitedNodes = new Set<string>();
  const stack = [node];

  while (stack.length) {
    const currentNode = stack.pop()!;
    visitedNodes.add(currentNode);

    const parentNodes = graph.predecessors(currentNode);
    if (parentNodes) {
      parentNodes.forEach(async parentNode => {
        if (!visitedNodes.has(parentNode)) {
          const parentAction = await getPromptById(parentNode);
          if (parentAction) {
            ancestors.push(parentAction);
            stack.push(parentNode);
          }

        }
      });
    }
  }

  return ancestors;
}

export async function getPromptById(id: string): Promise<Prompt | null> {
  const systemState = await getSystemState();
  const action = systemState.aiSystemState.actions.find((action: Prompt) => getId(action) == id);
  return action || null;
}

export async function getProcessById(id: string): Promise<Process | null> {
  const systemState = await getSystemState();
  const process = systemState.aiSystemState.processes.find((process: Process) => getId(process) == id);
  return process || null;
}

export function topologicalSort(graph: Graph) {
  const sorted = alg.topsort(graph);

  // The stack now contains a topological ordering of the nodes
  return sorted;
}

// get the name of the action by using the id
export async function getNodeName(id: string): Promise<string | undefined> {
  const res: AiSystemState = await new Promise((resolve, _rej) => {
    systemStateStore.subscribe((systemStateStore) => {
      resolve(systemStateStore.aiSystemState);
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

export async function printEdge(edge: Edge) {
  const sourceName = await getNodeName(edge.v);
  const targetName = await getNodeName(edge.w);
  console.log("edge: " + sourceName + " -> " + targetName);
}

export function getId(actionOrProcess: Process | Prompt): string {
  return actionOrProcess._id.$oid;
}

export async function setSystemState(systemState: SystemState) {
  systemStateStore.set(systemState);
}

export async function addGlobalVariable(variable_name: string, variable_value: string) {
  const current_state = await getSystemState();
  current_state.graphState.global_variables.set(variable_name, variable_value);
  await setSystemState(current_state);
}

export async function addNode(node_id: string, node_type: NodeType): Promise<void> {
  const systemState = await getSystemState();
  // add the input and output variables to the graph state

  //check if the node already exists in the graph
  if (!systemState.graphState.graph.hasNode(node_id)) {
    systemState.graphState.graph.setNode(node_id);
  }
  systemState.graphState.lastAction = "addNode";
  const node_name = await getNodeName(node_id);
  if (node_name) {
    systemState.graphState.name = node_name;
    systemState.graphState.actedOn = [node_id, node_name];
  } else {
    systemState.graphState.actedOn = [node_id, ""];
  }
  setSystemState(systemState);
}

// function for converting a process to a graph
export async function processToGraph(process: Process): Promise<void> {
  await resetGraph();

  // verify that all of the steps have corresponding actions
  const graph = process.graph;

  let nodes: string[] = [];

  // check if graph has the type Graph
  if (graph instanceof Graph) {
    nodes = graph.nodes();
  }
  else {
    console.error("The graph is not of type Graph");
  }

  // console.log("nodes: ", nodes);

  // for each of the node ids stored in nodes, get the name of the action

  //loop through the nodes
  for (let i = 0; i < nodes.length; i++) {
    const name = await getNodeName(nodes[i]);
    if (name) {
      await addNode(nodes[i], NodeType.Action);
    }
  }

  let edges: Edge[] = [];

  if (graph instanceof Graph) {
    edges = graph.edges();
  }

  let topOrder: string[] = [];

  if (graph instanceof Graph) {
    topOrder = topologicalSort(graph);
  }

  for (const node of topOrder) {
    // filter edges where the source node is the current node
    const nodeEdges = edges.filter(this_edge => this_edge.v === node);

    // iterate over the node's edges and add them
    for (const edge of nodeEdges) {
      // if edge does not exist, add it
      await addEdge(edge); // assuming 'addEdge' is your helper function

    }
  }
}

export async function sendPrompt(prompt: Prompt) {
  const systemState = await getSystemState();
  systemState.executionContext.prompts.set(prompt.action_id, prompt.prompt_text);
  systemState.websocket.send(JSON.stringify(prompt));
  await setSystemState(systemState);
}

export async function getParentOutputVariables(this_node_id: string): Promise<string[]> {
  const systemState = await getSystemState();

  // get topological order

  const topological_order = systemState.executionContext.topological_order;

  // get parent node id
  const parent_node_id = topological_order[topological_order.indexOf(this_node_id) - 1];

  // get the output variables of the parent node
  const parent_output_variables = systemState.aiSystemState.actions.find(action => getId(action) == parent_node_id)?.output_variables || [];

  return parent_output_variables;
}

export async function setLocalExecutionVariable(variable_name: string, variable_value: string): Promise<Map<string, string>> {
  const systemState = await getSystemState();
  systemState.executionContext.local_variables.set(variable_name, variable_value);
  await setSystemState(systemState);
  return systemState.executionContext.local_variables;
}

export async function setGlobalExecutionVariable(variable_name: string, variable_value: string) {
  const systemState = await getSystemState();
  systemState.executionContext.global_variables.set(variable_name, variable_value);
  await setSystemState(systemState);
}

export function addVariablesToPrompt(prompt: string, variables: Map<string, string>): string {
  let new_prompt = prompt;
  for (const [key, value] of variables) {
    new_prompt = new_prompt.replace(key, value);
  }
  return new_prompt;
}

export async function incrementCurrentNode(): Promise<string> {
  const systemState = await getSystemState();

  // look at the topological order and the current_node and set the next node to be the current node
  const topological_order = systemState.executionContext.topological_order;

  // get the index of the current node
  if (systemState.executionContext.current_node != null) {
    const current_node_index = topological_order.indexOf(systemState.executionContext.current_node);
    if (current_node_index + 1 < topological_order.length) {
      systemState.executionContext.current_node = topological_order[current_node_index + 1];

      // get the prompt of the current_node
      const prompt = await getPromptofAction(systemState.executionContext.current_node);

      systemState.executionContext.prompts.set(systemState.executionContext.current_node, prompt);
    }
    else {
      console.error("current node index is out of bounds");
    }
  }
  else {
    console.error("current node is null");
    systemState.executionContext.current_node = topological_order[0];
  }

  await setSystemState(systemState);
  return systemState.executionContext.current_node;
}

export async function getPromptofAction(action_id: string): Promise<string> {
  const systemState = await getSystemState();
  const prompt = systemState.aiSystemState.actions.filter(action => action._id.$oid === action_id)[0].prompt;

  return prompt;
}

export async function sendWebsocketMessage(message: string) {
  const systemState = await getSystemState();
  systemState.websocket.send(message);
}

export async function addEdge(edge: Edge): Promise<void> {

  await printEdge(edge);

  const systemState = await getSystemState();
  // check if the edge already exists
  const edgeExists = systemState.graphState.graph.hasEdge(edge);
  if (!edgeExists) {
    systemState.graphState.graph.setEdge(edge);
  }
  systemState.graphState.lastAction = "addEdge";
  systemState.graphState.actedOn = edge;
  setSystemState(systemState);
}

export async function removeNode(id: string): Promise<void> {
  const name = await getNodeName(id);
  const systemState = await getSystemState();
  systemState.graphState.graph.removeNode(id);
  systemState.graphState.lastAction = "removeNode";
  if (name) {
    systemState.graphState.actedOn = [id, name];
  } else {
    systemState.graphState.actedOn = [id, "unknown"];
  }
  setSystemState(systemState);
}

export async function removeSelectedNode(): Promise<void> {
  const systemState = await getSystemState();
  if (Array.isArray(systemState.graphState.actedOn)) {
    const selected = systemState.graphState.actedOn[0];
    await removeNode(selected);
  }
}

export async function removeSelectedEdge(): Promise<void> {
  const systemState = await getSystemState();
  if (
    !Array.isArray(systemState.graphState.actedOn) &&
    systemState.graphState.lastAction == "selectEdge"
  ) {
    const selected = systemState.graphState.actedOn;
    if (selected != null) {
      await removeEdge(selected.v, selected.w);
    }
  } else {
    // console.log("not removing edge, doesn't meet criteria");
  }
}

export async function removeEdge(
  _sourceId: string,
  _targetId: string
): Promise<void> {
  const systemState = await getSystemState();
  // find the id of the edge to remove

  // console.log("removing edge:", sourceId, targetId, " from graph");

  const edge = systemState.graphState.actedOn;
  // graphState.graph.removeEdge(edge);

  systemState.graphState.lastAction = "removeEdge";
  systemState.graphState.actedOn = edge;
  systemState.graphState.name = null;
  setSystemState(systemState);
}

export async function selectNode(id: string): Promise<void> {
  const ai_system_state = (await getSystemState()).aiSystemState;
  const actions = ai_system_state.actions;

  const res = actions.find((action: Prompt) => {
    return getId(action) == id;
  });
  if (res) {
    const systemState = await getSystemState();
    systemState.selectedAction = res;
    systemState.currentlySelected = "action";
    systemState.graphState.lastAction = "selectNode";
    systemState.graphState.lastActedOn = systemState.graphState.actedOn;
    systemState.graphState.actedOn = [id, res.name];
    systemState.graphState.name = res.name;
    setSystemState(systemState);
  }
}

export async function selectEdge(
  source: string,
  target: string
): Promise<void> {
  const systemState = await getSystemState();

  systemState.graphState.lastAction = "selectEdge";
  systemState.graphState.actedOn = { v: source, w: target };
  systemState.graphState.name = null;
  setSystemState(systemState);
}

export async function resetLastAction(): Promise<void> {
  const systemState = await getSystemState();
  systemState.graphState.lastAction = "none";
  systemState.graphState.actedOn = null;
  setSystemState(systemState);
}

export async function nodes(): Promise<string[]> {
  const systemState = await getSystemState();
  return systemState.graphState.graph.nodes();
}

export async function edges(): Promise<Edge[]> {
  const systemState = await getSystemState();
  return systemState.graphState.graph.edges();
}

// reset the graphState to a new empty graph
export async function resetGraph(): Promise<void> {
  // console.log("resetting graph");
  const systemState = await getSystemState();
  systemState.graphState.graph = new Graph();
  systemState.graphState.lastAction = "resetGraph";
  systemState.graphState.actedOn = null;
  systemState.graphState.name = null;
  setSystemState(systemState);
}
