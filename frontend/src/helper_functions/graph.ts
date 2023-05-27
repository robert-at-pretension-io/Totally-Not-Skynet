import type {
  GraphState,
  AiSystemState,
  SystemState,
  Action,
  Prompt,
} from "../system_types";
import { graphStore } from "../stores/graphStore";
import { Process } from "../system_types";
import { aiSystemStore } from "../stores/aiSystemStore";
import systemStateStore from "stores/systemStateStore";
import { Graph } from "graphlib";
import { Edge } from "@dagrejs/graphlib";
import { alg } from "graphlib";
import websocketStore from "stores/websocketStore";

// Define the getter and setter

export async function getGraphState(): Promise<GraphState> {
  return new Promise((resolve, _) => {
    graphStore.subscribe((graphState: GraphState) => {
      resolve(graphState);
    });
  });
}

export function getGlobalVariableNames() {
  let globalVariableNames: string[] = [];
  graphStore.subscribe(store => {
    globalVariableNames = Array.from(store.global_variables.keys());
  })();
  return globalVariableNames;
}

export async function getAncestorNodes(node: string, graph: Graph): Promise<Action[]> {
  const ancestors: Action[] = [];
  const visitedNodes = new Set<string>();
  const stack = [node];

  while (stack.length) {
    const currentNode = stack.pop()!;
    visitedNodes.add(currentNode);

    const parentNodes = graph.predecessors(currentNode);
    if (parentNodes) {
      parentNodes.forEach(async parentNode => {
        if (!visitedNodes.has(parentNode)) {
          const parentAction = await getActionById(parentNode);
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

export async function getActionById(id: string): Promise<Action | null> {
  const aiSystemState = await getAiSystemState();
  const action = aiSystemState.actions.find(action => getId(action) == id);
  return action || null;
}

export async function getProcessById(id: string): Promise<Process | null> {
  const aiSystemState = await getAiSystemState();
  const process = aiSystemState.processes.find(process => getId(process) == id);
  return process || null;
}

export async function getAiSystemState(): Promise<AiSystemState> {
  return new Promise((resolve, _) => {
    aiSystemStore.subscribe((aiSystemState: AiSystemState) => {
      resolve(aiSystemState);
    });
  });
}

export function topologicalSort(graph: Graph) {
  const sorted = alg.topsort(graph);

  // print out the nodes in the stack
  sorted.forEach(async node => {
    const name = await getNodeName(node);
    console.log("node: " + name);
  });

  // The stack now contains a topological ordering of the nodes
  return sorted;
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

export async function printEdge(edge: Edge) {
  const sourceName = await getNodeName(edge.v);
  const targetName = await getNodeName(edge.w);
  console.log("edge: " + sourceName + " -> " + targetName);
}

export function getId(actionOrProcess: Process | Action): string {
  return actionOrProcess._id.$oid;
}

export async function setGraphState(graphState: GraphState) {
  // const input_variables = await getAllInputVariables();
  // const output_variables = await getAllOutputVariables();
  // graphState.input_variables = input_variables;
  // graphState.output_variables = output_variables;
  // console.log("The graphstate is:\n ", graphState);
  graphStore.set(graphState);
}

export async function addGlobalVariable(variable_name: string, variable_value: string) {
  const current_state = await getGraphState();
  current_state.global_variables.set(variable_name, variable_value);
  await setGraphState(current_state);
}

export async function addNode(node_id: string): Promise<void> {
  const graphState = await getGraphState();
  // add the input and output variables to the graph state

  //check if the node already exists in the graph
  if (!graphState.graph.hasNode(node_id)) {
    graphState.graph.setNode(node_id);
  }
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

  const topOrder = topologicalSort(graph);

  console.log("edges: ", my_edges);

  for (const node of topOrder) {
    // filter edges where the source node is the current node
    const nodeEdges = my_edges.filter(edge => edge.v === node);

    // iterate over the node's edges and add them
    for (const edge of nodeEdges) {
      // if edge does not exist, add it
      await addEdge(edge); // assuming 'addEdge' is your helper function

    }
  }
}

export async function sendPrompt(prompt: Prompt) {
  // send via websocketstore
  websocketStore.subscribe((ws: WebSocket) => {
    ws.send(JSON.stringify(prompt));
  });
}

export async function checkEdgeVariables(
  sourceNode: string,
  targetNode: string,
  globalVariables: string[],
  g: Graph
): Promise<boolean> {
  // let sourceName = await getNodeName(sourceNode);
  // let targetName = await getNodeName(targetNode);
  // console.log(
  //   "Checking edge variables between nodes ",
  //   sourceName,
  //   " and ",
  //   targetName
  // );

  // Get the input variables of target action
  const targetAction = await getActionById(targetNode);
  if (targetAction == null) {
    console.log("targetAction is null");
    return false;
  }
  const targetInputVariables = targetAction.input_variables;
  // console.log("Target Action input variables: ", targetInputVariables);

  // Get the output variables of source node
  const sourceAction = await getActionById(sourceNode);
  if (sourceAction == null) {
    console.log("sourceAction is null");
    return false;
  }
  const sourceOutputVariables = sourceAction.output_variables;
  // console.log("Source Action output variables: ", sourceOutputVariables);

  // Get all ancestor nodes of the target node
  const ancestorNodes = await getAncestorNodes(targetNode, g);
  // console.log("Ancestor Nodes of the target node: ", ancestorNodes);

  // Collect the output variables of all ancestor nodes
  const ancestorOutputVariables = ancestorNodes.flatMap(
    (node) => node.output_variables
  );
  console.log(
    "Output variables of the ancestor nodes: ",
    ancestorOutputVariables
  );

  // Combine the output variables of the source node, the ancestor nodes, and the global variables
  const allValidInputs = [
    ...sourceOutputVariables,
    ...ancestorOutputVariables,
    ...globalVariables,
  ];
  console.log("All valid inputs: ", allValidInputs);

  // Ensure every input variable of the target node exists in the combined array of valid input variables
  const isValid = targetInputVariables.every((variable) =>
    allValidInputs.includes(variable)
  );
  console.log("Are all target input variables valid? ", isValid);

  return isValid;
}

export async function addEdge(edge: Edge): Promise<void> {

  await printEdge(edge);

  const graphState = await getGraphState();
  // check if the edge already exists
  const edgeExists = graphState.graph.hasEdge(edge);
  if (!edgeExists) {
    graphState.graph.setEdge(edge);
  }
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
  graphState.actedOn = { v: source, w: target };
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
  // console.log("resetting graph");
  const graphState = await getGraphState();
  graphState.graph = new Graph();
  graphState.lastAction = "resetGraph";
  graphState.actedOn = null;
  graphState.name = null;
  setGraphState(graphState);
}
