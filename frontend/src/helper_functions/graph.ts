import type {
  GraphState,
  AiSystemState,
  SystemState,
  Action,
} from "../system_types";
import { graphStore } from "../stores/graphStore";
import { Process } from "../system_types";
import { aiSystemStore } from "../stores/aiSystemStore";
import systemStateStore from "stores/systemStateStore";
import { Graph } from "graphlib";
import { Edge } from "@dagrejs/graphlib";

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

export function topologicalSort(graph) {
  const visited = new Set();
  const stack = [];

  function visit(node) {
    // Mark the node as visited
    visited.add(node);

    // Visit all neighbors
    const neighbors = graph.neighbors(node) || [];
    for (const neighbor of neighbors) {
      if (!visited.has(neighbor)) {
        visit(neighbor);
      }
    }

    // Push the node to the stack after visiting all descendants
    stack.push(node);
  }

  // Visit all nodes
  graph.nodes().forEach(node => {
    if (!visited.has(node)) {
      visit(node);
    }
  });

  // The stack now contains a topological ordering of the nodes
  return stack.reverse();
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
  // const input_variables = await getAllInputVariables();
  // const output_variables = await getAllOutputVariables();
  // graphState.input_variables = input_variables;
  // graphState.output_variables = output_variables;
  // console.log("The graphstate is:\n ", graphState);
  graphStore.set(graphState);
}

export async function addGlobalVariable(variable_name: string) {
  const current_state = await getGraphState();
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
  console.log("resetting graph");
  const graphState = await getGraphState();
  graphState.graph = new Graph();
  graphState.lastAction = "resetGraph";
  graphState.actedOn = null;
  graphState.name = null;
  setGraphState(graphState);
}
