import type {
  SystemState,
  Node,
  GraphNodeInfo,
  Edge,
  SystemErrors,
  GraphState,
  Graph,
  Process,

} from "generated/system_types_pb.js";

import systemStateStore from "stores/systemStateStore";
import * as graphlib from "graphlib";

// Define the getter and setter

export async function getSystemState(): Promise<SystemState> {
  return new Promise((resolve, _rej) => {
    systemStateStore.subscribe((systemStateStore) => {
      resolve(systemStateStore);
    });
  });
}

export function systemGraphToGraphLib(graph: Graph) {
  const g = new graphlib.Graph();

  graph.nodes.forEach((node: GraphNodeInfo) => {
    g.setNode(node.id, node.name);
  });

  graph.edges.forEach((edge: Edge) => {
    g.setEdge({ v: edge.source.id, w: edge.target.id });
  });

  return g;
}

export async function handleError(error: SystemErrors) {
  switch (error.name) {
  case "GraphDoesntExist": {
    console.log(error);
    break;
  }
  case "OtherError": {
    console.log(error);
    break;
  }
  case "GraphStateDoesntExist": {
    console.log(error);
    break;
  }
  case "NodeDoesntExist": {
    console.log(error);
    break;
  }
  default: {
    console.log("Uncovered Error");
  }
  }
}

export async function getInputVariablesByNodeId(nodeId: string): Promise<string[] | null> {
  // Get the action by ID
  const node = await getNode(nodeId);

  if (node && node.Node.type_name === "Prompt") {
    return node.Node.input_variables;
  }
  return null;
}

export async function validateGraph(systemState: SystemState): Promise<GraphNodeInfo[] | boolean> {
  const graph = systemState.graph_state?.graph;

  if (systemState.selected_node && graph) {
    const selected_node: Node = systemState.selected_node;
    if (RuntimeProcess.is(selected_node)) {
      const process: Process = selected_node.Node.node_content as Process;
      const initial_variables = process.Process.initial_variables;

      const test_orders: GraphNodeInfo[][] = await getAllTopologicalOrders(graph);

      for (let i = 0; i++; i < test_orders.length) {
        const current_order = test_orders[i];

        // to test the order we need to keep track of which variables have already been defined by collecting the output variables in an array as we go, then we only need to determine if the input variables are in the array

        const agregate_variables = initial_variables;

        for (let j = 0; j++; j < current_order.length) {
          const current_node = current_order[j];
          const node = await getNode(current_node);
          if (node) {
            const input_variables = node.Node.input_variables;
            const output_variables = node.Node.output_variables;

            // check if all of the input variables are in the agregate_variables array
            const input_variables_in_agregate = input_variables.every((variable) => {
              return agregate_variables.includes(variable);
            });

            // if the input variables are in the agregate_variables array, then add the output variables to the agregate_variables array
            if (input_variables_in_agregate) {
              agregate_variables.push(...output_variables);
              // if we are on the last node, then we have a valid order
              if (j == current_order.length - 1) {
                return current_order;
              }
            }
            else {
              return false;
            }
          }
          else {
            return false;
          }

        }

      }
      return false;

    }
    else {
      return false;
    }
  }
  return false;
}

export async function getAllTopologicalOrders(graph: Graph): Promise<GraphNodeInfo[][]> {
  // check that there is a single component (that the graph is connected) AND
  // that there are no cycles in the graph

  const graphlib_graph = systemGraphToGraphLib(graph);

  if (!graphlib.alg.isAcyclic(graphlib_graph) || graphlib.alg.components(graphlib_graph).length !== 1) {
    return [];
  }

  return await allTopologicalSorts(graph);

}

export async function returnSuccessorMap(graph: Graph): Promise<Map<GraphNodeInfo, GraphNodeInfo[]>> {
  const node_neightbors: Map<GraphNodeInfo, GraphNodeInfo[]> = new Map();

  const graphlib_graph = systemGraphToGraphLib(graph);
  const my_nodes = graphlib_graph.nodes();

  for (let i = 0; i < my_nodes.length; i++) {
    const node = my_nodes[i];
    const neighbors = graphlib_graph.successors(node);
    if (neighbors) {
      const node_info = await getNodeInfo(node);
      if (node_info) {
        const neighbors_node_info: GraphNodeInfo[] = [];
        neighbors.forEach(async (neighbor) => {
          const neighbor_node_info = await getNodeInfo(neighbor);
          if (neighbor_node_info) {
            neighbors_node_info.push(neighbor_node_info);
          }
        });
        node_neightbors.set(node_info, neighbors_node_info);
      }
    }
  }
  return node_neightbors;
}

async function allTopologicalSorts(graph: Graph): Promise<GraphNodeInfo[][]> {
  const all_orderings: GraphNodeInfo[][] = [];
  const successor_map = await returnSuccessorMap(graph);
  const start_nodes = await returnStartNodes(graph);
  const in_degree_map = await returnAllIndegree(graph);
  const visited: Map<GraphNodeInfo, boolean> = new Map();

  graph.nodes.forEach((node) => {
    visited.set(node, false);
  });

  function helper(node: GraphNodeInfo, in_degree_map: Map<GraphNodeInfo, number>, visited: Map<GraphNodeInfo, boolean>, stack: GraphNodeInfo[]): void {

    visited.set(node, true);

    stack.push(node);

    if (stack.length === graph.nodes.length) {
      all_orderings.push([...stack]);
    } else {

      const successors = successor_map.get(node);

      if (successors) {
        successors.forEach((successor) => {
          const count = in_degree_map.get(successor);
          const is_visited = visited.get(successor);

          if (count && is_visited != undefined) {
            const new_count = count - 1;
            in_degree_map.set(successor, new_count);

            if (new_count == 0 && !is_visited) {
              helper(successor, in_degree_map, visited, stack);
            }

            in_degree_map.set(successor, count);

          }

        });
      }

    }

    visited.set(node, false);
    stack.pop();
  }

  start_nodes.forEach(node => {
    helper(node, in_degree_map, visited, []);
  });

  return all_orderings;
}

async function returnStartNodes(graph: Graph): Promise<GraphNodeInfo[]> {

  const start_nodes: GraphNodeInfo[] = [];

  const graphlib_graph = systemGraphToGraphLib(graph);

  const sources = graphlib_graph.sources();

  sources.forEach(async (source_id: string) => {
    const val = await getNodeInfo(source_id);
    if (val) {
      start_nodes.push(val);
    }
  });
  return start_nodes;
}

async function returnAllIndegree(graph: Graph): Promise<Map<GraphNodeInfo, number>> {

  const in_degree_map: Map<GraphNodeInfo, number> = new Map();

  const graphlib_graph = systemGraphToGraphLib(graph);

  graphlib_graph.nodes().forEach(async (source_id: string) => {
    const val = await getNodeInfo(source_id);

    let count = 0;
    const maybe_count = graphlib_graph.predecessors(source_id);

    if (maybe_count) {
      count = maybe_count.length;
    }

    if (val) {
      in_degree_map.set(val, count);
    }
  });
  return in_degree_map;
}

export async function getOutputVariablesByNodeId(nodeId: string): Promise<string[] | null> {
  // Get the node by Id
  const node = await getNode(nodeId);
  if (node) {
    return node.Node.output_variables;
  }
  return null;
}

export async function getAncestorNodes(node: string, graph: Graph): Promise<Node[]> {

  const graphlib_graph = systemGraphToGraphLib(graph);

  const ancestors: Node[] = [];
  const visitedNodes = new Set<string>();
  const stack = [node];

  while (stack.length) {
    const currentNode = stack.pop()!;
    visitedNodes.add(currentNode);

    const parentNodes = graphlib_graph.predecessors(currentNode);
    if (parentNodes) {
      parentNodes.forEach(async parentNode => {
        if (!visitedNodes.has(parentNode)) {
          const parent_node = await getNode(parentNode);
          if (parent_node) {
            ancestors.push(parent_node);
            stack.push(parentNode);
          }

        }
      });
    }
  }

  return ancestors;
}

export async function getNode(id: string | GraphNodeInfo): Promise<Node | undefined> {

  if (RuntimeGraphNodeInfo.is(id)) {
    id = id.id;
  }

  const systemState = await getSystemState();
  const prompt = systemState.nodes.find((node: Node) => {
    if (node.Node._id) {
      return getId(node) == id;
    }
  });
  return prompt;
}

export async function getNodeInputVariables(node_id: string): Promise<string[] | null> {
  const node = await getNode(node_id);
  if (node) {
    return node.Node.input_variables;
  }
  else return null;
}

// get the name of the action by using the id
export async function getNodeName(id: string): Promise<string | undefined> {
  const system_state = await getSystemState();

  const node = system_state.nodes.find((node: Node) => {
    // get the node with the id:
    if (node.Node._id) {
      return getId(node) == id;
    }
  });
  if (node) {
    return node.Node.name;
  }
}

export async function printEdge(edge: Edge) {
  const sourceName = edge.source.name;
  const targetName = edge.target.name;
  console.log("edge: " + sourceName + " -> " + targetName);
}

export function getId(node: Node): string | undefined {
  if (node) {
    return node.Node._id?.$oid;
  }
  return undefined;

}

export async function setSystemState(systemState: SystemState) {
  systemStateStore.set(systemState);
}

export async function graphHasNode(node: GraphNodeInfo | Node, graph_state: GraphState): Promise<boolean | void> {
  let node_name = "";

  if (RuntimeNode.is(node)) {
    node_name = node.Node.name;
  }
  else if (RuntimeGraphNodeInfo.is(node)) {
    node_name = node.name;
  }

  if (!graph_state.graph) {
    await handleError({ name: "GraphDoesntExist" });
  }
  else {
    const graph = systemGraphToGraphLib(graph_state.graph);
    return graph.hasNode(node_name);
  }

}

export async function graphHasEdge(edge: Edge, graph_state: GraphState): Promise<boolean | void> {
  if (await graphExists(graph_state)) {
    const graph = systemGraphToGraphLib(graph_state.graph);
    return graph.hasEdge(edge.source.id, edge.target.id);
  }
}

export async function graphExists(graph_state: GraphState): Promise<boolean | void> {
  if (!(graph_state.graph)) {
    await handleError({ name: "GraphDoesntExist" });
  }
  else {
    return true;
  }
}

export async function addEdge(edge: Edge, graph_state: GraphState): Promise<void> {

  const system_state = await getSystemState();

  console.log("Adding Edge:");
  await printEdge(edge);

  if (!graph_state.graph.edges.includes(edge)) {
    addEdge(edge, graph_state);
  }
  graph_state.last_action = "add";
  graph_state.acted_on = edge;

  system_state.graph_state = graph_state;

  setSystemState(system_state);
}

export async function getNodeInfo(node: GraphNodeInfo | Node | string): Promise<GraphNodeInfo | void> {
  if (RuntimeGraphNodeInfo.is(node)) {
    return node;
  }
  else if (typeof node === "string") {
    const node_id = await getNode(node as string);
    if (node_id) {
      return await getNodeInfo(node_id);
    }
    else {
      await handleError({ name: "NodeDoesntExist" });
    }
  }
  else {
    return {
      id: node.Node._id.$oid,
      name: node.Node.name
    };
  }
}

export async function addNode(node: GraphNodeInfo | Node, graph_state: GraphState): Promise<void> {
  const systemState = await getSystemState();
  // add the input and output variables to the graph state

  node = await getNodeInfo(node) as GraphNodeInfo;

  //check if the node already exists in the graph
  if (await !graphHasNode(node, graph_state)) {
    // Based on the definition of graphHasNode, we can assume that the graph is defined.
    console.log("Adding node to graph");
    const graph = systemGraphToGraphLib(graph_state.graph);
    graph.setNode(node.id, node.name);
  }
  else {
    console.log("Node ", node, " is already in the graph, not adding it.");
    return;
  }
  graph_state.last_action = "add";
  graph_state.acted_on = node;

  setSystemState(systemState);
}

// function for converting a process to a graph
export async function processToGraphVisualization(process: Process, graph_state: GraphState): Promise<void> {
  await resetGraph();

  const graph = graph_state.graph;
  const nodes = graph.nodes;

  //loop through the nodes
  for (let i = 0; i < nodes.length; i++) {
    const node = await getNode(nodes[i]);
    if (node) {
      await addNode(node, graph_state);
    }
  }

  const topOrder: GraphNodeInfo[][] = await getAllTopologicalOrders(graph);

  // This function doesn't exist yet.
  findValidTopOrder(topOrder);
}

export function findValidTopOrder(topOrder: GraphNodeInfo[][]): GraphNodeInfo[] {
  console.log("REPLACE ME WITH REAL VALID TOPOLOGICAL ORDER");
  return [];
}

// export async function getParentOutputVariables(this_node_id: string): Promise<string[] | null> {
//   const systemState = await getSystemState();

//   // get topological order

//   const topological_order = systemState.execution_context.topological_order;

//   // get parent node id
//   const parent_node_id = topological_order[topological_order.indexOf(this_node_id) - 1];

//   // get the output variables of the parent node
//   const parent_output_variables = getOutputVariablesByNodeId(parent_node_id);

//   return parent_output_variables;
// }

export function addVariablesToPrompt(prompt: string, variables: Map<string, string>): string {
  let new_prompt = prompt;
  for (const [key, value] of variables) {
    new_prompt = new_prompt.replace(key, value);
  }
  return new_prompt;
}

export async function removeNode(id: string): Promise<void> {
  const systemState = await getSystemState();
  const node_info = await getNodeInfo(id);
  if (node_info) {
    const remove_index = systemState.graph_state.graph.nodes.indexOf(node_info);
    systemState.graph_state.graph.nodes.splice(remove_index);
    systemState.graph_state.last_action = "remove";
    systemState.graph_state.acted_on = node_info;
    setSystemState(systemState);
  }
}

export async function removeEdge(
  _sourceId: string,
  _targetId: string
): Promise<void> {
  const systemState = await getSystemState();
  // find the id of the edge to remove

  // console.log("removing edge:", sourceId, targetId, " from graph");

  const edge = systemState.graph_state.acted_on;
  // graphState.graph.removeEdge(edge);

  systemState.graph_state.last_action = "remove";
  systemState.graph_state.acted_on = edge;

  setSystemState(systemState);
}

export async function returnProcesses(): Promise<Node[]> {
  const systemState = await getSystemState();
  let nodes = systemState.nodes;

  // filter out the prompts
  nodes = nodes.filter((node: Node) => {
    return node.Node.type_name == "Process";
  }
  );

  // let processes = nodes.map((node: Node) => {
  //   return node.node_content as Process;
  // }
  // );

  return nodes;
}

export async function selectNode(id: string): Promise<void> {
  const system_state = await getSystemState();
  const nodes = system_state.nodes;

  const res = nodes.find((node: Node) => getId(node) == id);

  if (res) {
    const systemState = await getSystemState();
    systemState.selected_node = res;
    systemState.graph_state.last_action = "select";
    systemState.graph_state.last_acted_on = systemState.graph_state.acted_on;
    systemState.graph_state.acted_on = { id, name: res.Node.name };
    setSystemState(systemState);
  }
}

export async function selectEdge(edge: Edge
): Promise<void> {
  const systemState = await getSystemState();

  systemState.graph_state.last_action = "select";
  systemState.graph_state.acted_on = edge;
  setSystemState(systemState);
}

export async function resetLastAction(): Promise<void> {
  const systemState = await getSystemState();
  systemState.graph_state.last_action = "none";
  systemState.graph_state.acted_on = undefined;
  setSystemState(systemState);
}

export function nodes(graph_state: GraphState): GraphNodeInfo[] {
  return graph_state.graph.nodes;
}

export function edges(graph_state: GraphState): Edge[] {
  return graph_state.graph.edges;
}

// reset the graphState to a new empty graph
export async function resetGraph(): Promise<void> {
  const systemState = await getSystemState();
  systemState.graph_state.graph = { edges: [], nodes: [] };
  systemState.graph_state.last_action = "reset";
  systemState.graph_state.acted_on = undefined;
  setSystemState(systemState);
}
