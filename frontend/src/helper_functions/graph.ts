import * as graphlib from "graphlib";
import * as proto from "../generated/system_types";

import { NodeTypeNames } from "generated/system_types_pb";

// import { NodeTypeNames } from './path/to/your/enum';  // Import your NodeTypeNames enum

export const stylesMap: { [key: string]: { [styleKey: string]: string } } = {
  [NodeTypeNames.PROMPT]: {
    "background-color": "#ff0000",
    "border-color": "#00ff00"
  },
  [NodeTypeNames.PROCESS]: {
    "background-color": "#0000ff",
    "border-color": "#ffff00"
  },
  [NodeTypeNames.CONDITIONAL]: {
    "background-color": "#ff00ff",
    "border-color": "#ff8800"
  },
  [NodeTypeNames.COMMAND]: {
    "background-color": "#00ffff",
    "border-color": "#8800ff"
  }
};

export const generateDynamicStyles = (): Array<any> => {
  return Object.keys(stylesMap).map((key) => {
    return {
      selector: `.${key.toLowerCase()}`, 
      style: stylesMap[key]
    };
  });
};

export function systemGraphToGraphLib(
  system_state: proto.SystemState
): graphlib.Graph {
  const graph = system_state.graph as proto.Graph;
  // const graph = graph_state.graph as proto.Graph;

  const g = new graphlib.Graph();

  graph.nodes.forEach((node: proto.GraphNodeInfo) => {
    g.setNode(node.id, node.name);
  });

  graph.edges.forEach((edge: proto.Edge) => {
    const source = edge.source?.id;
    const target = edge.target?.id;
    if (source != undefined && target != undefined) {
      g.setEdge({ v: source, w: target });
    }
  });

  return g;
}

export function handleError(_error: any) {
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

// export function validateGraph(
//   system_state: proto.SystemState
// ): proto.GraphNodeInfo[] | boolean {
//   const graph_state = system_state.graph_state as proto.GraphState;
//   if (!graph_state) {
//     // await handleError({ name: "GraphDoesntExist" });
//     console.log("Graph doesn't exist");
//     return false;
//   } else {
//     const test_orders: proto.GraphNodeInfo[][] = getAllTopologicalOrders(
//       system_state
//     );
//     console.log("test_orders: ", test_orders);
//     alert("For the time being, just return the first topological order");
//     if (test_orders.length >= 0) {
//       return test_orders[0];

//     }
//     else {
//       console.log("There are no topologic");
//       return false;
//     }
//   }

// }

// export function getAllTopologicalOrders(
//   system_state: proto.SystemState
// ): proto.GraphNodeInfo[][] {
//   // check that there is a single component (that the graph is connected) AND
//   // that there are no cycles in the graph

//   const graphlib_graph = systemGraphToGraphLib(system_state);

//   if (
//     !graphlib.alg.isAcyclic(graphlib_graph) ||
//     graphlib.alg.components(graphlib_graph).length !== 1
//   ) {
//     return [];
//   }

//   return allTopologicalSorts(system_state);
// }

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

// function allTopologicalSorts(
//   system_state: proto.SystemState
// ): proto.GraphNodeInfo[][] {
//   const graph_state = system_state.graph_state as proto.GraphState;
//   const all_orderings: proto.GraphNodeInfo[][] = [];
//   const graph = graph_state.graph as proto.Graph;
//   const successor_map = returnSuccessorMap(system_state);
//   const start_nodes = returnStartNodes(system_state);
//   const in_degree_map = returnAllIndegree(system_state);
//   const visited: Map<proto.GraphNodeInfo, boolean> = new Map();

//   const node_list = graph.getNodesList();

//   node_list.forEach(async (node_info) => {
//     visited.set(node_info, false);
//   });

//   function helper(
//     node: proto.GraphNodeInfo,
//     in_degree_map: Map<proto.GraphNodeInfo, number>,
//     visited: Map<proto.GraphNodeInfo, boolean>,
//     stack: proto.GraphNodeInfo[]
//   ): void {
//     visited.set(node, true);

//     stack.push(node);

//     if (stack.length === node_list.length) {
//       all_orderings.push([...stack]);
//     } else {
//       const successors = successor_map.get(node);

//       if (successors) {
//         successors.forEach((successor) => {
//           const count = in_degree_map.get(successor);
//           const is_visited = visited.get(successor);

//           if (count && is_visited != undefined) {
//             const new_count = count - 1;
//             in_degree_map.set(successor, new_count);

//             if (new_count == 0 && !is_visited) {
//               helper(successor, in_degree_map, visited, stack);
//             }

//             in_degree_map.set(successor, count);
//           }
//         });
//       }
//     }

//     visited.set(node, false);
//     stack.pop();
//   }

//   start_nodes.forEach((node) => {
//     helper(node, in_degree_map, visited, []);
//   });

//   return all_orderings;
// }

// function returnStartNodes(
//   system_state: proto.SystemState
// ): proto.GraphNodeInfo[] {

//   const start_nodes: proto.GraphNodeInfo[] = [];

//   const graphlib_graph = systemGraphToGraphLib(system_state);

//   const sources = graphlib_graph.sources();

//   sources.forEach(async (source_id: string) => {
//     const val = await getNodeInfo(source_id, system_state);
//     if (val) {
//       start_nodes.push(val);
//     }
//   });
//   return start_nodes;
// }

export function getNode(id: string, system_state: proto.SystemState): proto.Node {

  const nodes = system_state.nodes;

  const node = nodes.find((node: proto.Node) => {
    const test_id = node.node_info;
    if (test_id && id) {
      return test_id.id == id;
    }
    else {
      console.log("one of the nodes doesn't have an id");
    }
  });

  return node as proto.Node;
}

export function getNodeInfo(id: string, system_state: proto.SystemState): proto.GraphNodeInfo | undefined {
  const node_info_list: proto.GraphNodeInfo[] = [];

  system_state.nodes?.forEach((node: proto.Node) => {
    node_info_list.push(node.node_info as proto.GraphNodeInfo);
  });

  // return the node where node_info.get_id() == id
  const node_info = node_info_list?.find((node_info: proto.GraphNodeInfo) => {
    return node_info.id == id;
  }
  );
  return node_info;
}

// function returnAllIndegree(
//   // graph_state: proto.GraphState
//   system_state: proto.SystemState
// ): Map<proto.GraphNodeInfo, number> {

//   const in_degree_map: Map<proto.GraphNodeInfo, number> = new Map();

//   const graphlib_graph = systemGraphToGraphLib(system_state);

//   graphlib_graph.nodes().forEach((source_id: string) => {
//     const val = getNodeInfo(source_id, system_state);

//     let count = 0;
//     const maybe_count = graphlib_graph.predecessors(source_id);

//     if (maybe_count) {
//       count = maybe_count.length;
//     }

//     if (val) {
//       in_degree_map.set(val, count);
//     }
//   });
//   return in_degree_map;
// }

// export function graphHasNode(
//   node: proto.Node,
//   system_state: proto.SystemState
// ): boolean | void {

//   const graph_state = system_state.graph_state as proto.GraphState;

//   const graph = graph_state.graph;
//   const node_info = node.getNodeInfo();

//   if (!graph) {
//     console.log("Graph doesn't exist");
//     false;
//   } else {
//     const node_info_list = graph.getNodesList();
//     if (node_info) {

//       console.log("The node info is: ", node_info.toObject());

//       console.log("The node info list is: ", node_info_list);
//       //loop through node_info_list
//       for (let i = 0; i < node_info_list.length; i++) {
//         if ((node_info_list[i] = node_info)) {
//           return true;
//         }
//       }
//     }
//   }
//   return false;
// }

// export function graphHasEdge(
//   edge: proto.Edge,
//   system_state: proto.SystemState
// ): boolean | void {

//   const graph_state = system_state.graph_state as proto.GraphState;

//   const graph = graph_state.graph;

//   if (!graph) {
//     console.log("Graph doesn't exist");
//     false;
//   } else {
//     const edge_list = graph.getEdgesList();

//     if (edge_list.includes(edge)) {

//       return true;
//     }

//   }
//   return false;
// }

// export function addNode(
//   node: proto.Node,
//   // graph_state: proto.GraphState
//   system_state: proto.SystemState
// ): proto.SystemState {

//   console.log("addNode system_state: ", system_state.toObject());

//   let graph_state = system_state.graph_state as proto.GraphState;

//   if (!graph_state) {
//     graph_state = new proto.GraphState();
//   }

//   // console.log("add the addNode function with inputs: ", node.toObject(), graph_state.toObject());
//   // const systemState = await getSystemState();
//   // add the input and output variables to the graph state

//   const node_list = graph_state.graph?.getNodesList();

//   if (node_list == undefined) {
//     alert("node_list is undefined");
//   }

//   const has_node = node_list?.includes(node.getNodeInfo() as proto.GraphNodeInfo);

//   console.log("has_node: ", has_node);

//   //check if the node already exists in the graph
//   if (!has_node) {

//     const node_info = node.getNodeInfo() as proto.GraphNodeInfo;

//     const graph_action = new proto.GraphAction();
//     graph_action.setAction(proto.GraphAction.Action.ADD);
//     graph_action.setNode(node_info);

//     const action_history = graph_state.getActionHistoryList();
//     action_history.push(graph_action);
//     graph_state.setActionHistoryList(action_history);

//     const graph = graph_state.graph as proto.Graph;
//     const node_array = graph.getNodesList() as proto.GraphNodeInfo[];
//     node_array.push(node_info);
//     graph.setNodesList(node_array);

//     system_state.setGraphState(graph_state);

//   } else {
//     console.log("Node ", node, " is already in the graph, not adding it.");
//   }

//   return system_state;

// }

// export function addEdge(
//   edge: proto.Edge,
//   // graph_state: proto.GraphState
//   system_state: proto.SystemState
// ): proto.SystemState {

//   console.log("addEdge system_state: ", system_state.toObject());

//   const graph_state = system_state.graph_state as proto.GraphState;

//   const source = edge.getSource();
//   const target = edge.getTarget();

//   console.log("Attempting to add edge: ", edge.toObject());

//   const edge_list = graph_state.graph?.getEdgesList() as proto.Edge[];

//   console.log("edge_list: ", edge_list);
//   console.log("edge_list?.includes(edge): ", edge_list?.includes(edge));

//   if (source && target && !graphHasEdge(edge, system_state)) {

//     const graph_action = new proto.GraphAction();
//     graph_action.setAction(proto.GraphAction.Action.ADD);
//     graph_action.setNode();
//     graph_action.setEdge(edge);

//     const action_history = graph_state.getActionHistoryList();
//     action_history.push(graph_action);
//     graph_state.setActionHistoryList(action_history);

//     edge_list?.push(edge);
//     const graph = graph_state.graph as proto.Graph;
//     graph.setEdgesList(edge_list);
//     graph_state.setGraph(graph);

//     system_state.setGraphState(graph_state);

//     return system_state;

//   } else {
//     console.log("Edge ", edge, " is already in the graph, not adding it.");
//     return system_state;
//   }

// }

// function for converting a process to a graph
// export function processToGraphVisualization(

//   system_state: proto.SystemState
// ): void {
//   resetGraph(system_state);

//   const graph_state = system_state.graph_state as proto.GraphState;

//   const graph = graph_state.graph as proto.Graph;
//   const nodes = graph.getNodesList() as proto.GraphNodeInfo[];

//   //loop through the nodes
//   for (let i = 0; i < nodes.length; i++) {
//     const node = getNode(nodes[i].getId() as string, system_state);
//     if (node) {
//       addNode(node, system_state);
//     }
//   }

//   const topOrder: proto.GraphNodeInfo[][] = getAllTopologicalOrders(
//     system_state
//   );

//   // This function doesn't exist yet.
//   findValidTopOrder(topOrder);
// }

// export function findValidTopOrder(
//   topOrder: proto.GraphNodeInfo[][]
// ): proto.GraphNodeInfo[] {
//   console.log(topOrder);
//   console.log("REPLACE ME WITH REAL VALID TOPOLOGICAL ORDER");
//   return [];
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

// export function removeNode(id: string, system_state: proto.SystemState): proto.SystemState {
//   const node_info = getNodeInfo(id, system_state);
//   if (node_info) {

//     const latest_action = new proto.GraphAction();
//     latest_action.setAction(proto.GraphAction.Action.REMOVE);
//     latest_action.setNode(node_info);

//     const graph_state = system_state.graph_state as proto.GraphState;
//     const graph = graph_state.graph as proto.Graph;
//     const node_array = graph.getNodesList() as proto.GraphNodeInfo[];
//     const remove_index = node_array.indexOf(node_info);
//     node_array.splice(remove_index);
//     graph.setNodesList(node_array);
//     graph_state.setGraph(graph);

//     const action_history = graph_state.getActionHistoryList();

//     action_history.push(latest_action);

//     graph_state.setActionHistoryList(action_history);

//     system_state.setGraphState(graph_state);

//     return system_state;

//   }
//   else {
//     console.log("Node ", id, " doesn't exist, not removing it.");
//     return system_state;
//   }

// }

// export function removeEdge(remove_edge: proto.Edge, system_state: proto.SystemState): proto.SystemState {

//   const graph_state = system_state.graph_state as proto.GraphState;
//   const graph = graph_state.graph as proto.Graph;
//   const edge_array = graph.getEdgesList() as proto.Edge[];
//   // const remove_index = edge_array.indexOf(remove_edge);

//   //find the index where the edge has the source that is the same as the remove_edge
//   // and the target that is the same as the remove_edge
//   const remove_index = edge_array.findIndex((edge: proto.Edge) => {
//     const source = edge.getSource();
//     const target = edge.getTarget();
//     if (source && target) {
//       return source.getId() == remove_edge.getSource()?.getId() && target.getId() == remove_edge.getTarget()?.getId();
//     }
//   });

//   console.log("remove_index: ", remove_index);

//   edge_array.splice(remove_index);
//   graph.setEdgesList(edge_array);
//   graph_state.setGraph(graph);

//   const action_history = graph_state.getActionHistoryList();

//   const latest_action = new proto.GraphAction();
//   latest_action.setAction(proto.GraphAction.Action.REMOVE);
//   latest_action.setEdge(remove_edge);

//   action_history.push(latest_action);

//   graph_state.setActionHistoryList(action_history);

//   system_state.setGraphState(graph_state);

//   return system_state;
// }

export function selectNode(id: proto.GraphNodeInfo, system_state: proto.SystemState): proto.SystemState {

  console.log("Entering selectNode function with ID:", id);

  const selected_nodes = system_state.selected_node;

  selected_nodes.push(id);

  system_state.selected_node = selected_nodes;

  return system_state;
}

export function selectEdge(edge: proto.Edge, system_state: proto.SystemState): proto.SystemState {

  const graph = system_state.graph as proto.Graph;

  const edges = graph?.edges as proto.Edge[];
  console.log("Current Edges List:", edges);

  const found_index = edges.find((edge_info) => {
    console.log("Checking edge:", edge_info);
    return edge.source.id == edge_info.source.id && edge.target.id == edge_info.target.id;
  }) as proto.Edge;

  if (found_index) {
    const selected_edges = system_state.selected_edge;

    selected_edges.push(found_index);

    system_state.selected_edge = [found_index];
  }
  return system_state;
}

// export function resetLastAction(system_state: proto.SystemState): proto.SystemState {

//   const graph_state = system_state.graph_state as proto.GraphState;

//   const graph_action = new proto.GraphAction();

//   graph_action.setAction(proto.GraphAction.Action.NONE);
//   graph_action.setEdge();
//   graph_action.setNode();

//   const action_history = graph_state.getActionHistoryList();

//   action_history.push(graph_action);

//   graph_state.setActionHistoryList(action_history);

//   system_state.setGraphState(graph_state);

//   return system_state;
// }

// reset the proto.GraphState to a new empty graph
// export function resetGraph(system_state: proto.SystemState): proto.SystemState {

//   const graph_state = system_state.graph_state as proto.GraphState;

//   const graph_action = new proto.GraphAction();

//   graph_action.setAction(proto.GraphAction.Action.RESET);
//   graph_action.setEdge();
//   graph_action.setNode();

//   const action_history = graph_state.getActionHistoryList();

//   action_history.push(graph_action);

//   graph_state.setActionHistoryList(action_history);

//   system_state.setGraphState(graph_state);

//   return system_state;
// }
