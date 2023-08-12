import type {
  SystemState,
  Node,
  CrudBundle,
  GraphNodeInfo,
  Edge,
  SystemErrors,
  GraphState,
  Graph
} from "../system_types";
import { Process, RuntimeGraphNodeInfo, RuntimeNode } from "../system_types";
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
  let g = new graphlib.Graph()


  graph.nodes.forEach((node: GraphNodeInfo) => {
    g.setNode(node.id, node.name);
  })

  graph.edges.forEach((edge: Edge) => {
    g.setEdge({ v: edge.source.id, w: edge.target.id })
  })

  return g;
}

export async function handleError(error: SystemErrors) {
  switch (error.name) {
    case "GraphDoesntExist": {

    }
    case "OtherError": {

    }
    default: {

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

export async function validateGraph(systemState: SystemState): Promise<string[] | boolean> {
  const graph = systemState.graph_state?.graph;


  if (systemState.selected_node && graph) {
    const selected_node: Node = systemState.selected_node;
    if (selected_node.Node.type_name == "Process") {
      const process: Process = selected_node.Node.node_content as Process;
      const initial_variables = process.Process.initial_variables;

      const test_orders: string[][] = getAllTopologicalOrders(graph);

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

export function getAllTopologicalOrders(graph: Graph): string[][] {
  // check that there is a single component (that the graph is connected) AND
  // that there are no cycles in the graph

  let graphlib_graph = systemGraphToGraphLib(graph);

  if (!graphlib.alg.isAcyclic(graphlib_graph) || graphlib.alg.components(graphlib_graph).length !== 1) {
    return [];
  }

  // get the local graph
  const local_graph = graphToLocalGraph(graph);

  return allTopologicalSorts(local_graph);

}



export function graphToLocalGraph(graph: Graph): Map<GraphNodeInfo, string[]> {
  const node_neightbors: Map<GraphNodeInfo, string[]>;

  let graphlib_graph = systemGraphToGraphLib(graph);

  const my_nodes = graphlib_graph.nodes();

  for (let i = 0; i < my_nodes.length; i++) {
    const node = my_nodes[i];
    const neighbors = graphlib_graph.successors(node);
    if (neighbors) {
      node_neightbors[node] = neighbors;
    }
  }

  return local_graph;
}

function allTopologicalSorts(graph: LocalGraph): string[][] {
  const allOrderings: string[][] = [];
  const indegreeMap = calculateIndegreeForAllVertex(graph);
  const startNodes = Array.from(Object.keys(indegreeMap)).filter((node) => indegreeMap[node] === 0);
  const visited: { [node: string]: boolean } = {};

  for (const node in graph) {
    visited[node] = false;
  }

  function helper(node: string, indegreeMap: { [node: string]: number }, visited: { [node: string]: boolean }, stack: string[]): void {
    visited[node] = true;
    stack.push(node);

    if (stack.length === Object.keys(graph).length) {
      allOrderings.push([...stack]);
    } else {
      for (const neighbor of graph[node]) {
        indegreeMap[neighbor]--;
        if (indegreeMap[neighbor] === 0 && !visited[neighbor]) {
          helper(neighbor, indegreeMap, visited, stack);
        }
        indegreeMap[neighbor]++;
      }
    }

    visited[node] = false;
    stack.pop();
  }

  for (const node of startNodes) {
    helper(node, { ...indegreeMap }, { ...visited }, []);
  }

  return allOrderings;
}

function calculateIndegreeForAllVertex(graph: LocalGraph): { [node: string]: number } {
  const indegreeMap: { [node: string]: number } = {};

  for (const node in graph) {
    indegreeMap[node] = 0;
  }

  for (const node in graph) {
    for (const neighbor of graph[node]) {
      indegreeMap[neighbor]++;
    }
  }

  return indegreeMap;
}

export async function getOutputVariablesByNodeId(nodeId: string): Promise<string[] | null> {
  // Get the node by Id
  const node = await getNode(nodeId);
  if (node) {
    return node.Node.output_variables;
  }
  return null;
}

export async function getGlobalVariableNames(): Promise<Map<string, string> | null> {
  let system_state = await getSystemState();
  let global_vars = system_state.graph_state?.global_variables;

  return global_vars ? global_vars : null;

}

export async function getAncestorNodes(node: string, graph: Graph): Promise<Node[]> {
  const ancestors: Node[] = [];
  const visitedNodes = new Set<string>();
  const stack = [node];

  while (stack.length) {
    const currentNode = stack.pop()!;
    visitedNodes.add(currentNode);

    const parentNodes = graph.predecessors(currentNode);
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
  const targetName = edge.target.name
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

export async function graphHasNode(node: GraphNodeInfo | Node, graph_state: GraphState): Promise<Boolean | void> {
  let node_name = "";

  if (RuntimeNode.is(node)) {
    node_name = node.Node.name
  }
  else if (RuntimeGraphNodeInfo.is(node)) {
    node_name = node.name;
  }

  if (!graph_state.graph) {
    await handleError({ name: "GraphDoesntExist" })
  }
  else {
    graph_state.graph.hasNode(node_name)
  }

}

export async function graphHasEdge(edge: Edge, graph_state: GraphState): Promise<Boolean, void> {
  if (await graphExists(graph_state)) {
    return graph_state.graph.hasEdge(edge.source.id, edge.target.id)
  }
}

export async function graphStringToGraph(graph_state: GraphState): Promise<Graph | void> {
  if (await graphExists()) {

  }
}
}

let current_graph_string = JSON.stringify(json.write(current_graph));

export async function graphStringToGraph(graph_state: GraphState): Promise<Graph | void> {
  if (await graphExists()) {

  }
}
}

export async function graphExists(graph_state: GraphState): Promise<boolean | void> {
  if (!(graph_state.graph)) {
    await handleError({ name: "GraphDoesntExist" })
  }
  else {
    return true;
  }
}


export async function addEdge(edge: Edge, graph_state: GraphState): Promise<void> {

  let system_state = await getSystemState();

  console.log("Adding Edge:")
  await printEdge(edge);

  if (!graphHasEdge(edge, graph_state.graph)) {
    addEdge(edge, graph_state)
  }
  graph_state.last_action = "addEdge";
  graph_state.acted_on = edge;

  system_state.graph_state = graph_state;

  setSystemState(system_state);
}

export async function getNodeInfo(node: GraphNodeInfo | Node | string): Promise<GraphNodeInfo | void> {
  if (RuntimeGraphNodeInfo.is(node)) {
    return node;
  }
  else if (typeof node === "string") {
    let node_id = await getNode(node as string);
    if (node_id) {
      return await getNodeInfo(node_id);
    }
    else {
      await handleError({ name: "NodeDoesntExist" })
    }
  }
  else {
    return {
      id: node.Node._id.$oid,
      name: node.Node.name
    }
  }
}

export async function addNode(node: GraphNodeInfo | Node, graph_state: GraphState): Promise<void> {
  const systemState = await getSystemState();
  // add the input and output variables to the graph state

  node = getNodeInfo(node) as GraphNodeInfo;

  //check if the node already exists in the graph
  if (await !graphHasNode(node, graph_state)) {
    // Based on the definition of graphHasNode, we can assume that the graph is defined.
    console.log("Adding node to graph");
    graph_state.graph?.setNode(node.id, node.name);
  }
  else {
    console.log("Node ", node, " is already in the graph, not adding it.");
    return;
  }
  graph_state.last_action = "addNode";
  graph_state.acted_on = node;

  setSystemState(systemState);
}

// function for converting a process to a graph
export async function processToGraphVisualization(process: Process, graph_state: GraphState): Promise<void> {
  await resetGraph();

  // verify that all of the steps have corresponding actions
  let graph: string | Graph = process.Process.graph;

  let nodes: string[] = [];

  // check if the graph is a string and if so, parse it into a graphlib Graph object
  if (typeof graph === "string") {
    const parsed_graph = JSON.parse(graph);
    graph = new Graph(parsed_graph);
  }

  nodes = graph.nodes()

  //loop through the nodes
  for (let i = 0; i < nodes.length; i++) {
    let node = await getNode(nodes[i]);
    if (node) {
      await addNode(node, graph_state);
    }
  }

  let edges: Edge[] = [];

  if (graph instanceof Graph) {
    edges = graph.edges();
  }

  // note that this contains a list of lists of ALL possible orders. We must still go through the list and ensure that the input/output variables within each ordering is "valid"
  let topOrder: string[][] = [];

  if (graph instanceof Graph) {
    topOrder = getAllTopologicalOrders(graph);
  }

  // This function doesn't exist yet.
  findValidTopOrder(topOrder);
}

export function findValidTopOrder(topOrder: string[][]): string[] {
  console.log("REPLACE ME WITH REAL VALID TOPOLOGICAL ORDER");
  return [];
}

export async function getParentOutputVariables(this_node_id: string): Promise<string[] | null> {
  const systemState = await getSystemState();

  // get topological order

  const topological_order = systemState.execution_context.topological_order;

  // get parent node id
  const parent_node_id = topological_order[topological_order.indexOf(this_node_id) - 1];

  // get the output variables of the parent node
  const parent_output_variables = getOutputVariablesByNodeId(parent_node_id);

  return parent_output_variables;
}

export function addVariablesToPrompt(prompt: string, variables: Map<string, string>): string {
  let new_prompt = prompt;
  for (const [key, value] of variables) {
    new_prompt = new_prompt.replace(key, value);
  }
  return new_prompt;
}

export async function sendWebsocketMessage(message: CrudBundle) {

  console.log("sending websocket message: ", message);
  const systemState = await getSystemState();
  const message_string = JSON.stringify(message);
  systemState.websocket.send(message_string);
}


export async function removeNode(id: string): Promise<void> {
  const name = await getNodeName(id);
  const systemState = await getSystemState();
  systemState.graph_state.graph.removeNode(id);
  systemState.graph_state.last_action = "removeNode";

  const node_name = await getNodeName(id);

  const nodeInfo: GraphNodeInfo = {
    id: id,
    name: node_name ? node_name : ""
  };

  systemState.graph_state.acted_on = nodeInfo;

  setSystemState(systemState);
}

export async function removeSelectedNode(): Promise<void> {
  const systemState = await getSystemState();
  if (Array.isArray(systemState.graph_state.acted_on)) {
    const selected = systemState.graph_state.acted_on[0];
    await removeNode(selected);
  }
}

export async function removeSelectedEdge(): Promise<void> {
  const systemState = await getSystemState();
  if (
    !Array.isArray(systemState.graph_state.acted_on) &&
    systemState.graph_state.last_action == "selectEdge"
  ) {
    const selected = systemState.graph_state.acted_on;
    if (selected != null) {
      const edge = selected as Edge;
      await removeEdge(edge.v, edge.w);
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

  const edge = systemState.graph_state.acted_on;
  // graphState.graph.removeEdge(edge);

  systemState.graph_state.last_action = "removeEdge";
  systemState.graph_state.acted_on = edge;
  systemState.graph_state.name = null;
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

  // const res = actions.find((action: Prompt) => {
  //   return getId(action) == id;
  // });

  const res = nodes.find((node: Node) => getId(node) == id);

  if (res) {
    const systemState = await getSystemState();
    systemState.selected_node = res;
    systemState.graph_state.last_action = "selectNode";
    systemState.graph_state.last_acted_on = systemState.graph_state.acted_on;
    systemState.graph_state.acted_on = { id, name: res.Node.name };
    systemState.graph_state.name = res.Node.name;
    setSystemState(systemState);
  }
}

export async function selectEdge(
  source: string,
  target: string
): Promise<void> {
  const systemState = await getSystemState();

  systemState.graph_state.last_action = "selectEdge";
  systemState.graph_state.acted_on = { v: source, w: target };
  systemState.graph_state.name = null;
  setSystemState(systemState);
}

export async function resetLastAction(): Promise<void> {
  const systemState = await getSystemState();
  systemState.graph_state.last_action = "none";
  systemState.graph_state.acted_on = undefined;
  setSystemState(systemState);
}

export async function nodes(): Promise<string[] | null> {
  const systemState = await getSystemState();
  if (systemState.graph_state?.graph != null) {
    return systemState.graph_state.graph.nodes();
  }
  else {
    return null;
  }
}

export async function edges(): Promise<Edge[]> {
  const systemState = await getSystemState();
  return systemState.graph_state.graph.edges();
}

// reset the graphState to a new empty graph
export async function resetGraph(): Promise<void> {
  // console.log("resetting graph");
  const systemState = await getSystemState();
  systemState.graph_state.graph = new Graph();
  systemState.graph_state.last_action = "resetGraph";
  systemState.graph_state.acted_on = null;
  systemState.graph_state.name = null;
  setSystemState(systemState);
}
