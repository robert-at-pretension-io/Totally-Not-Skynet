import {
  SystemState,
  Node,
  GraphNodeInfo,
  Edge,
  GraphState,
  Graph,
  Process,
  GraphAction,
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

export function systemGraphToGraphLib(graph_state: GraphState): graphlib.Graph {
  const graph = graph_state.getGraph() as Graph;

  const g = new graphlib.Graph();

  graph.getNodesList().forEach((node: GraphNodeInfo) => {
    g.setNode(node.getId(), node.getName());
  });

  graph.getEdgesList().forEach((edge: Edge) => {
    const source = edge.getSource()?.getId();
    const target = edge.getTarget()?.getId();
    if (source != undefined && target != undefined) {
      g.setEdge({ v: source, w: target });
    }
  });

  return g;
}

export async function handleError(_error: any) {
  // switch (error.name) {
  // case "GraphDoesntExist": {
  //   console.log(error);
  //   break;
  // }
  // case "OtherError": {
  //   console.log(error);
  //   break;
  // }
  // case "GraphStateDoesntExist": {
  //   console.log(error);
  //   break;
  // }
  // case "NodeDoesntExist": {
  //   console.log(error);
  //   break;
  // }
  // default: {
  //   console.log("Uncovered Error");
  // }
  // }

  alert("REIMPLEMENT THIS USING PROTO BUF");
}

export async function validateGraph(
  system_state: SystemState
): Promise<GraphNodeInfo[] | boolean> {
  const graph_state = system_state.getGraphState() as GraphState;
  if (!graph_state) {
    await handleError({ name: "GraphDoesntExist" });
  } else {
    const test_orders: GraphNodeInfo[][] = await getAllTopologicalOrders(
      graph_state
    );
    console.log("test_orders: ", test_orders);
  }
  alert("Actually need to validate the graph");
  return true;
}

export async function getAllTopologicalOrders(
  graph_state: GraphState
): Promise<GraphNodeInfo[][]> {
  // check that there is a single component (that the graph is connected) AND
  // that there are no cycles in the graph

  const graphlib_graph = systemGraphToGraphLib(graph_state);

  if (
    !graphlib.alg.isAcyclic(graphlib_graph) ||
    graphlib.alg.components(graphlib_graph).length !== 1
  ) {
    return [];
  }

  return await allTopologicalSorts(graph_state);
}

export async function returnSuccessorMap(
  graph_state: GraphState
): Promise<Map<GraphNodeInfo, GraphNodeInfo[]>> {
  const node_neightbors: Map<GraphNodeInfo, GraphNodeInfo[]> = new Map();

  const graphlib_graph = systemGraphToGraphLib(graph_state);
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

async function allTopologicalSorts(
  graph_state: GraphState
): Promise<GraphNodeInfo[][]> {
  const all_orderings: GraphNodeInfo[][] = [];
  const graph = graph_state.getGraph() as Graph;
  const successor_map = await returnSuccessorMap(graph_state);
  const start_nodes = await returnStartNodes(graph_state);
  const in_degree_map = await returnAllIndegree(graph_state);
  const visited: Map<GraphNodeInfo, boolean> = new Map();

  const node_list = graph.getNodesList();

  node_list.forEach(async (node_info) => {
    visited.set(node_info, false);
  });

  function helper(
    node: GraphNodeInfo,
    in_degree_map: Map<GraphNodeInfo, number>,
    visited: Map<GraphNodeInfo, boolean>,
    stack: GraphNodeInfo[]
  ): void {
    visited.set(node, true);

    stack.push(node);

    if (stack.length === node_list.length) {
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

  start_nodes.forEach((node) => {
    helper(node, in_degree_map, visited, []);
  });

  return all_orderings;
}

async function returnStartNodes(
  graph_state: GraphState
): Promise<GraphNodeInfo[]> {
  const start_nodes: GraphNodeInfo[] = [];

  const graphlib_graph = systemGraphToGraphLib(graph_state);

  const sources = graphlib_graph.sources();

  sources.forEach(async (source_id: string) => {
    const val = await getNodeInfo(source_id);
    if (val) {
      start_nodes.push(val);
    }
  });
  return start_nodes;
}

export async function getNode(id: string): Promise<Node> {
  const system_state = await getSystemState();

  const nodes = system_state.getNodesList();

  const node = nodes.find((node: Node) => {
    const test_id = node.getNodeInfo();
    if (test_id) {
      return test_id.getId() == id;
    }
  });

  return node as Node;
}

export async function getNodeInfo(id: string): Promise<GraphNodeInfo> {
  const node_info = await getNode(id);
  return node_info.getNodeInfo() as GraphNodeInfo;
}

async function returnAllIndegree(
  graph_state: GraphState
): Promise<Map<GraphNodeInfo, number>> {
  const in_degree_map: Map<GraphNodeInfo, number> = new Map();

  const graphlib_graph = systemGraphToGraphLib(graph_state);

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

// export async function getAncestorNodes(
//   node: string,
//   graph_state: GraphState
// ): Promise<Node[]> {
//   const graphlib_graph = systemGraphToGraphLib(graph_state);

//   const ancestors: Node[] = [];
//   const visitedNodes = new Set<string>();
//   const stack = [node];

//   while (stack.length) {
//     const currentNode = stack.pop()!;
//     visitedNodes.add(currentNode);

//     const parentNodes = graphlib_graph.predecessors(currentNode);
//     if (parentNodes) {
//       parentNodes.forEach(async (parentNode) => {
//         if (!visitedNodes.has(parentNode)) {
//           const parent_node = await getNode(parentNode);
//           if (parent_node) {
//             ancestors.push(parent_node);
//             stack.push(parentNode);
//           }
//         }
//       });
//     }
//   }

//   return ancestors;
// }

export async function setSystemState(systemState: SystemState) {
  systemStateStore.set(systemState);
}

export async function graphHasNode(
  node: Node,
  graph_state: GraphState
): Promise<boolean | void> {

  const graph = graph_state.getGraph();
  const node_info = node.getNodeInfo();

  if (!graph) {
    await handleError({ name: "GraphDoesntExist" });
  } else {
    const node_info_list = graph.getNodesList();
    if (node_info) {
      //loop through node_info_list
      for (let i = 0; i < node_info_list.length; i++) {
        if ((node_info_list[i] = node_info)) {
          return true;
        }
      }
    }
  }
  return false;
}

// export async function addEdge(graph_state: GraphState): Promise<void> {
//   const system_state = await getSystemState();

//   const action_history = system_state
//     .getGraphState()
//     ?.getActionHistoryList() as GraphAction[];

//   const last_action = action_history[action_history.length - 1];

//   if (last_action) {
//     if (last_action.getAction() == GraphAction.Action.ADD) {
//       const last_acted_on = last_action.getEdge() as Edge;
//     }
//   }

//   graph_state.last_action = "add";
//   graph_state.acted_on = edge;

//   system_state.graph_state = graph_state;

//   setSystemState(system_state);
// }

export async function addNode(
  node: Node,
  graph_state: GraphState
): Promise<void> {
  const systemState = await getSystemState();
  // add the input and output variables to the graph state

  //check if the node already exists in the graph
  if (await !graphHasNode(node, graph_state)) {
    // Based on the definition of graphHasNode, we can assume that the graph is defined.
    console.log("Adding node to graph");
    const graph = systemGraphToGraphLib(graph_state);
    graph.setNode(
      node.getNodeInfo()?.getId() as string,
      node.getNodeInfo()?.getName() as string
    );
  } else {
    console.log("Node ", node, " is already in the graph, not adding it.");
    return;
  }

  const node_info = node.getNodeInfo() as GraphNodeInfo;

  const graph_action = new GraphAction();
  graph_action.setAction(GraphAction.Action.ADD);
  graph_action.setNode(node_info);

  const action_history = graph_state.getActionHistoryList();
  action_history.push(graph_action);
  graph_state.setActionHistoryList(action_history);

  systemState.setGraphState(graph_state);

  setSystemState(systemState);
}

// function for converting a process to a graph
export async function processToGraphVisualization(
  process: Process,
  graph_state: GraphState
): Promise<void> {
  await resetGraph();

  const graph = graph_state.getGraph() as Graph;
  const nodes = graph.getNodesList() as GraphNodeInfo[];

  //loop through the nodes
  for (let i = 0; i < nodes.length; i++) {
    const node = await getNode(nodes[i].getId() as string);
    if (node) {
      await addNode(node, graph_state);
    }
  }

  const topOrder: GraphNodeInfo[][] = await getAllTopologicalOrders(graph_state);

  // This function doesn't exist yet.
  findValidTopOrder(topOrder);
}

export function findValidTopOrder(
  topOrder: GraphNodeInfo[][]
): GraphNodeInfo[] {
  console.log(topOrder);
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

export function addVariablesToPrompt(
  prompt: string,
  variables: Map<string, string>
): string {
  let new_prompt = prompt;
  for (const [key, value] of variables) {
    new_prompt = new_prompt.replace(key, value);
  }
  return new_prompt;
}

export async function removeNode(id: string): Promise<void> {
  const systemState = await getSystemState();
  const node_info = await getNodeInfo(id);
  const node = await getNode(id);
  if (node_info) {
    const graph_state = systemState.getGraphState() as GraphState;
    const graph = graph_state.getGraph() as Graph;
    const node_array = graph.getNodesList() as GraphNodeInfo[];
    const remove_index = node_array.indexOf(node_info);
    node_array.splice(remove_index);
    graph.setNodesList(node_array);
    graph_state.setGraph(graph);

    const action_history = graph_state.getActionHistoryList();

    const latest_action = new GraphAction();
    latest_action.setAction(GraphAction.Action.REMOVE);
    latest_action.setNode(node.getNodeInfo() as GraphNodeInfo);

    action_history.push(latest_action);

    graph_state.setActionHistoryList(action_history);

    systemState.setGraphState(graph_state);

    await setSystemState(systemState);
  }
}

export async function removeEdge(
  remove_edge: Edge
): Promise<void> {
  const systemState = await getSystemState();

  const graph_state = systemState.getGraphState() as GraphState;
  const graph = graph_state.getGraph() as Graph;
  const edge_array = graph.getEdgesList() as Edge[];
  const remove_index = edge_array.indexOf(remove_edge);
  edge_array.splice(remove_index);
  graph.setEdgesList(edge_array);
  graph_state.setGraph(graph);

  const action_history = graph_state.getActionHistoryList();

  const latest_action = new GraphAction();
  latest_action.setAction(GraphAction.Action.REMOVE);
  latest_action.setEdge(remove_edge);

  action_history.push(latest_action);

  graph_state.setActionHistoryList(action_history);

  systemState.setGraphState(graph_state);

  await setSystemState(systemState);
}

export async function selectNode(id: string): Promise<void> {
  const system_state = await getSystemState();
  const graph_state = system_state.getGraphState() as GraphState;
  const graph = graph_state.getGraph();
  const nodes = graph?.getNodesList() as GraphNodeInfo[];

  const found_index = nodes.find((node_info) => {
    return node_info.getId() == id;
  }) as GraphNodeInfo;

  const graph_action = new GraphAction();

  graph_action.setAction(GraphAction.Action.SELECT);
  graph_action.setNode(found_index);

  const action_history = graph_state.getActionHistoryList();

  action_history.push(graph_action);

  graph_state.setActionHistoryList(action_history);

  system_state.setGraphState(graph_state);
  await setSystemState(system_state);

}

export async function selectEdge(edge: Edge): Promise<void> {
  const system_state = await getSystemState();
  const graph_state = system_state.getGraphState() as GraphState;
  const graph = graph_state.getGraph();
  const edges = graph?.getEdgesList() as Edge[];

  const found_index = edges.find((edge_info) => {
    return edge == edge_info;
  }) as Edge;

  const graph_action = new GraphAction();

  graph_action.setAction(GraphAction.Action.SELECT);
  graph_action.setEdge(found_index);

  const action_history = graph_state.getActionHistoryList();

  action_history.push(graph_action);

  graph_state.setActionHistoryList(action_history);

  system_state.setGraphState(graph_state);
  await setSystemState(system_state);

}

export async function resetLastAction(): Promise<void> {
  const systemState = await getSystemState();

  const graph_state = systemState.getGraphState() as GraphState;

  const graph_action = new GraphAction();

  graph_action.setAction(GraphAction.Action.NONE);
  graph_action.setEdge();
  graph_action.setNode();

  const action_history = graph_state.getActionHistoryList();

  action_history.push(graph_action);

  graph_state.setActionHistoryList(action_history);
  setSystemState(systemState);
}

// reset the graphState to a new empty graph
export async function resetGraph(): Promise<void> {

  const systemState = await getSystemState();

  const graph_state = systemState.getGraphState() as GraphState;

  const graph_action = new GraphAction();

  graph_action.setAction(GraphAction.Action.RESET);
  graph_action.setEdge();
  graph_action.setNode();

  const action_history = graph_state.getActionHistoryList();

  action_history.push(graph_action);

  graph_state.setActionHistoryList(action_history);
  setSystemState(systemState);
}
