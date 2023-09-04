import * as graphlib from "graphlib";
import * as proto from "../generated/system_types_pb";

export function systemGraphToGraphLib(
  system_state: proto.SystemState
): graphlib.Graph {
  const graph_state = system_state.getGraphState() as proto.GraphState;
  const graph = graph_state.getGraph() as proto.Graph;

  const g = new graphlib.Graph();

  graph.getNodesList().forEach((node: proto.GraphNodeInfo) => {
    g.setNode(node.getId(), node.getName());
  });

  graph.getEdgesList().forEach((edge: proto.Edge) => {
    const source = edge.getSource()?.getId();
    const target = edge.getTarget()?.getId();
    if (source != undefined && target != undefined) {
      g.setEdge({ v: source, w: target });
    }
  });

  return g;
}

export async function handleError(_error: any) {
  console.log("handleError: ", _error);
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
  system_state: proto.SystemState
): Promise<proto.GraphNodeInfo[] | boolean> {
  const graph_state = system_state.getGraphState() as proto.GraphState;
  if (!graph_state) {
    await handleError({ name: "GraphDoesntExist" });
  } else {
    const test_orders: proto.GraphNodeInfo[][] = await getAllTopologicalOrders(
      system_state
    );
    console.log("test_orders: ", test_orders);
  }
  alert("Actually need to validate the graph");
  return true;
}

export function getAllTopologicalOrders(
  system_state: proto.SystemState
): proto.GraphNodeInfo[][] {
  // check that there is a single component (that the graph is connected) AND
  // that there are no cycles in the graph

  const graphlib_graph = systemGraphToGraphLib(system_state);

  if (
    !graphlib.alg.isAcyclic(graphlib_graph) ||
    graphlib.alg.components(graphlib_graph).length !== 1
  ) {
    return [];
  }

  return allTopologicalSorts(system_state);
}

export function returnSuccessorMap(
  system_state: proto.SystemState
  // graph_state: proto.GraphState
): Map<proto.GraphNodeInfo, proto.GraphNodeInfo[]> {
  const node_neightbors: Map<proto.GraphNodeInfo, proto.GraphNodeInfo[]> =
    new Map();
  const graphlib_graph = systemGraphToGraphLib(system_state);
  const my_nodes = graphlib_graph.nodes();

  for (let i = 0; i < my_nodes.length; i++) {
    const node = my_nodes[i];
    const neighbors = graphlib_graph.successors(node);
    if (neighbors) {
      const node_info = getNodeInfo(node, system_state);
      if (node_info) {
        const neighbors_node_info: proto.GraphNodeInfo[] = [];
        neighbors.forEach(async (neighbor) => {
          const neighbor_node_info = getNodeInfo(neighbor, system_state);
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

function allTopologicalSorts(
  system_state: proto.SystemState
): proto.GraphNodeInfo[][] {
  const graph_state = system_state.getGraphState() as proto.GraphState;
  const all_orderings: proto.GraphNodeInfo[][] = [];
  const graph = graph_state.getGraph() as proto.Graph;
  const successor_map = returnSuccessorMap(system_state);
  const start_nodes = returnStartNodes(system_state);
  const in_degree_map = returnAllIndegree(system_state);
  const visited: Map<proto.GraphNodeInfo, boolean> = new Map();

  const node_list = graph.getNodesList();

  node_list.forEach(async (node_info) => {
    visited.set(node_info, false);
  });

  function helper(
    node: proto.GraphNodeInfo,
    in_degree_map: Map<proto.GraphNodeInfo, number>,
    visited: Map<proto.GraphNodeInfo, boolean>,
    stack: proto.GraphNodeInfo[]
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

function returnStartNodes(
  system_state: proto.SystemState
): proto.GraphNodeInfo[] {

  const start_nodes: proto.GraphNodeInfo[] = [];

  const graphlib_graph = systemGraphToGraphLib(system_state);

  const sources = graphlib_graph.sources();

  sources.forEach(async (source_id: string) => {
    const val = await getNodeInfo(source_id, system_state);
    if (val) {
      start_nodes.push(val);
    }
  });
  return start_nodes;
}

export function getNode(id: string, system_state: proto.SystemState): proto.Node {

  const nodes = system_state.getNodesList();

  const node = nodes.find((node: proto.Node) => {
    const test_id = node.getNodeInfo();
    if (test_id) {
      return test_id.getId() == id;
    }
  });

  return node as proto.Node;
}

export function getNodeInfo(id: string, system_state: proto.SystemState): proto.GraphNodeInfo {
  const node_info = getNode(id, system_state);
  return node_info.getNodeInfo() as proto.GraphNodeInfo;
}

function returnAllIndegree(
  // graph_state: proto.GraphState
  system_state: proto.SystemState
): Map<proto.GraphNodeInfo, number> {

  const in_degree_map: Map<proto.GraphNodeInfo, number> = new Map();

  const graphlib_graph = systemGraphToGraphLib(system_state);

  graphlib_graph.nodes().forEach((source_id: string) => {
    const val = getNodeInfo(source_id, system_state);

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
//   graph_state: proto.GraphState
// ): Promise<Node[]> {
//   const graphlib_graph = systemGraphToGraphLib(graph_state);

//   const ancestors: proto.Node[] = [];
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

export function graphHasNode(
  node: proto.Node,
  system_state: proto.SystemState
): boolean | void {

  const graph_state = system_state.getGraphState() as proto.GraphState;

  console.log("graphHasNode function with inputs: ", node.toObject(), graph_state.toObject());

  const graph = graph_state.getGraph();
  const node_info = node.getNodeInfo();

  if (!graph) {
    return;
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

// export async function addEdge(graph_state: proto.GraphState): Promise<void> {
//   const system_state = await getSystemState();

//   const action_history = system_state
//     .getGraphState()
//     ?.getActionHistoryList() as proto.GraphAction[];

//   const last_action = action_history[action_history.length - 1];

//   if (last_action) {
//     if (last_action.getAction() == proto.GraphAction.Action.ADD) {
//       const last_acted_on = last_action.getEdge() as Edge;
//     }
//   }

//   graph_state.last_action = "add";
//   graph_state.acted_on = edge;

//   system_state.graph_state = graph_state;

//   setSystemState(system_state);
// }

export function addNode(
  node: proto.Node,
  // graph_state: proto.GraphState
  system_state: proto.SystemState
): proto.SystemState | undefined {

  console.log("addNode system_state: ", system_state.toObject());

  const graph_state = system_state.getGraphState() as proto.GraphState;

  console.log("add the addNode function with inputs: ", node.toObject(), graph_state.toObject());
  // const systemState = await getSystemState();
  // add the input and output variables to the graph state

  const has_node = graphHasNode(node, system_state);

  console.log("has_node: ", has_node);

  //check if the node already exists in the graph
  if (!has_node) {

    const node_info = node.getNodeInfo() as proto.GraphNodeInfo;

    const graph_action = new proto.GraphAction();
    graph_action.setAction(proto.GraphAction.Action.ADD);
    graph_action.setNode(node_info);

    const action_history = graph_state.getActionHistoryList();
    action_history.push(graph_action);
    graph_state.setActionHistoryList(action_history);

    system_state.setGraphState(graph_state);

    return system_state;

  } else {
    console.log("Node ", node, " is already in the graph, not adding it.");
    return;
  }

}

// function for converting a process to a graph
export function processToGraphVisualization(

  system_state: proto.SystemState
): void {
  resetGraph(system_state);

  const graph_state = system_state.getGraphState() as proto.GraphState;

  const graph = graph_state.getGraph() as proto.Graph;
  const nodes = graph.getNodesList() as proto.GraphNodeInfo[];

  //loop through the nodes
  for (let i = 0; i < nodes.length; i++) {
    const node = getNode(nodes[i].getId() as string, system_state);
    if (node) {
      addNode(node, system_state);
    }
  }

  const topOrder: proto.GraphNodeInfo[][] = getAllTopologicalOrders(
    system_state
  );

  // This function doesn't exist yet.
  findValidTopOrder(topOrder);
}

export function findValidTopOrder(
  topOrder: proto.GraphNodeInfo[][]
): proto.GraphNodeInfo[] {
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

export async function removeNode(id: string, system_state: proto.SystemState): Promise<proto.SystemState | void> {
  const node_info = await getNodeInfo(id, system_state);
  const node = await getNode(id, system_state);
  if (node_info) {
    const graph_state = system_state.getGraphState() as proto.GraphState;
    const graph = graph_state.getGraph() as proto.Graph;
    const node_array = graph.getNodesList() as proto.GraphNodeInfo[];
    const remove_index = node_array.indexOf(node_info);
    node_array.splice(remove_index);
    graph.setNodesList(node_array);
    graph_state.setGraph(graph);

    const action_history = graph_state.getActionHistoryList();

    const latest_action = new proto.GraphAction();
    latest_action.setAction(proto.GraphAction.Action.REMOVE);
    latest_action.setNode(node.getNodeInfo() as proto.GraphNodeInfo);

    action_history.push(latest_action);

    graph_state.setActionHistoryList(action_history);

    system_state.setGraphState(graph_state);

    return system_state;
  }
}

export async function removeEdge(remove_edge: proto.Edge, system_state: proto.SystemState): Promise<proto.SystemState> {

  const graph_state = system_state.getGraphState() as proto.GraphState;
  const graph = graph_state.getGraph() as proto.Graph;
  const edge_array = graph.getEdgesList() as proto.Edge[];
  const remove_index = edge_array.indexOf(remove_edge);
  edge_array.splice(remove_index);
  graph.setEdgesList(edge_array);
  graph_state.setGraph(graph);

  const action_history = graph_state.getActionHistoryList();

  const latest_action = new proto.GraphAction();
  latest_action.setAction(proto.GraphAction.Action.REMOVE);
  latest_action.setEdge(remove_edge);

  action_history.push(latest_action);

  graph_state.setActionHistoryList(action_history);

  system_state.setGraphState(graph_state);

  return system_state;
}

export async function selectNode(id: string, system_state: proto.SystemState): Promise<proto.SystemState> {

  const graph_state = system_state.getGraphState() as proto.GraphState;
  const graph = graph_state.getGraph();
  const nodes = graph?.getNodesList() as proto.GraphNodeInfo[];

  const found_index = nodes.find((node_info) => {
    return node_info.getId() == id;
  }) as proto.GraphNodeInfo;

  const graph_action = new proto.GraphAction();

  graph_action.setAction(proto.GraphAction.Action.SELECT);
  graph_action.setNode(found_index);

  const action_history = graph_state.getActionHistoryList();

  action_history.push(graph_action);

  graph_state.setActionHistoryList(action_history);

  system_state.setGraphState(graph_state);

  return system_state;
}

export async function selectEdge(edge: proto.Edge, system_state: proto.SystemState): Promise<proto.SystemState> {

  const graph_state = system_state.getGraphState() as proto.GraphState;
  const graph = graph_state.getGraph();
  const edges = graph?.getEdgesList() as proto.Edge[];

  const found_index = edges.find((edge_info) => {
    return edge == edge_info;
  }) as proto.Edge;

  const graph_action = new proto.GraphAction();

  graph_action.setAction(proto.GraphAction.Action.SELECT);
  graph_action.setEdge(found_index);

  const action_history = graph_state.getActionHistoryList();

  action_history.push(graph_action);

  graph_state.setActionHistoryList(action_history);

  system_state.setGraphState(graph_state);

  return system_state;
}

export async function resetLastAction(system_state: proto.SystemState): Promise<proto.SystemState> {

  const graph_state = system_state.getGraphState() as proto.GraphState;

  const graph_action = new proto.GraphAction();

  graph_action.setAction(proto.GraphAction.Action.NONE);
  graph_action.setEdge();
  graph_action.setNode();

  const action_history = graph_state.getActionHistoryList();

  action_history.push(graph_action);

  graph_state.setActionHistoryList(action_history);

  system_state.setGraphState(graph_state);

  return system_state;
}

// reset the proto.GraphState to a new empty graph
export function resetGraph(system_state: proto.SystemState): proto.SystemState {

  const graph_state = system_state.getGraphState() as proto.GraphState;

  const graph_action = new proto.GraphAction();

  graph_action.setAction(proto.GraphAction.Action.RESET);
  graph_action.setEdge();
  graph_action.setNode();

  const action_history = graph_state.getActionHistoryList();

  action_history.push(graph_action);

  graph_state.setActionHistoryList(action_history);

  system_state.setGraphState(graph_state);

  return system_state;
}
