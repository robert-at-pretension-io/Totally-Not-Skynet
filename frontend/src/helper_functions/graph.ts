import * as graphlib from "graphlib";
import * as proto from "../generated/system_types";

import { NodeTypes } from "../generated/system_types";

// import { NodeTypes } from './path/to/your/enum';  // Import your NodeTypes enum

// export const stylesMap: { [key: string]: { [styleKey: string]: string } } = {
//   [NodeTypes.PROMPT]: {
//     "background-color": "#ff0000",
//     "border-color": "#00ff00",
//   },
//   [NodeTypes.PROCESS]: {
//     "background-color": "#0000ff",
//     "border-color": "#ffff00",
//   },
//   [NodeTypes.CONDITIONAL]: {
//     "background-color": "#ff00ff",
//     "border-color": "#ff8800",
//   },
//   [NodeTypes.COMMAND]: {
//     "background-color": "#00ffff",
//     "border-color": "#8800ff",
//   },
// };

// export const generateDynamicStyles = (): Array<any> => {
//   return Object.keys(stylesMap).map((key) => {
//     return {
//       selector: `.${key.toLowerCase()}`,
//       style: stylesMap[key],
//     };
//   });
// };

export function systemGraphToGraphLib(
  system_state: proto.SystemState
): graphlib.Graph {
  const graph = system_state.graph as proto.Graph;
  // const graph = graph_state.graph as proto.Graph;

  const g = new graphlib.Graph();

  graph.nodes_info.forEach((node: proto.GraphNodeInfo) => {
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

  alert("REIMPLEMENT THIS USING PROTO BUF");
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

export function getNode(
  id: string,
  system_state: proto.SystemState
): proto.Node {
  const nodes = system_state.local_nodes;

  const node = nodes.find((node: proto.Node) => {
    const test_id = node.node_info;
    if (test_id && id) {
      return test_id.id == id;
    } else {
      console.log("one of the nodes doesn't have an id");
    }
  });

  return node as proto.Node;
}

export function getNodeInfo(
  id: string,
  system_state: proto.SystemState
): proto.GraphNodeInfo | undefined {
  const node_info_list: proto.GraphNodeInfo[] = [];

  system_state.local_nodes?.forEach((node: proto.Node) => {
    node_info_list.push(node.node_info as proto.GraphNodeInfo);
  });

  // return the node where node_info.get_id() == id
  const node_info = node_info_list?.find((node_info: proto.GraphNodeInfo) => {
    return node_info.id == id;
  });
  return node_info;
}

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

export function selectNode(
  id: proto.GraphNodeInfo,
  system_state: proto.SystemState
): proto.SystemState {
  console.log("Entering selectNode function with ID:", id);

  const selected_nodes = system_state.selected_nodes;

  selected_nodes.push(id);

  system_state.selected_nodes = selected_nodes;

  return system_state;
}

export function selectEdge(
  edge: proto.Edge,
  system_state: proto.SystemState
): proto.SystemState {
  const graph = system_state.graph as proto.Graph;

  const edges = graph?.edges as proto.Edge[];
  console.log("Current Edges List:", edges);

  const found_index = edges.find((edge_info) => {
    console.log("Checking edge:", edge_info);
    return (
      edge.source.id == edge_info.source.id &&
      edge.target.id == edge_info.target.id
    );
  }) as proto.Edge;

  if (found_index) {
    const selected_edges = system_state.selected_edges;

    selected_edges.push(found_index);

    system_state.selected_edges = [found_index];
  }
  return system_state;
}
