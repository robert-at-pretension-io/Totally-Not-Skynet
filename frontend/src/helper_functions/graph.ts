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
  return new Promise((resolve, _reject) => {
    graphStore.subscribe((graphState: GraphState) => {
      resolve(graphState);
    });
  });
}

export async function getAiSystemState(): Promise<AiSystemState> {
  return new Promise((resolve, _reject) => {
    aiSystemStore.subscribe((aiSystemState: AiSystemState) => {
      resolve(aiSystemState);
    });
  });
}

// get the name of the action by using the id
export async function getNodeName(id: string): Promise<string | undefined> {
  const res: AiSystemState = await new Promise((resolve, _reject) => {
    aiSystemStore.subscribe((aiSystemState: AiSystemState) => {
      resolve(aiSystemState);
    });
  });
  const action = await res.actions.find(action => {
    return getId(action) == id;
  });
  if (action) {
    console.log("action name: " + action.name);
    return action.name;
  }
}

export function getId(actionOrProcess: Process | Action): string {
  return actionOrProcess._id.$oid;
}

export function setGraphState(graphState: GraphState) {
  graphStore.set(graphState);
}

export async function addNode(node_id: string): Promise<void> {
  const graphState = await getGraphState();
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

  const ai_system_state: AiSystemState = await new Promise((resolve, _reject) => {
    aiSystemStore.subscribe((ai_system_state: AiSystemState) => {
      resolve(ai_system_state);
    });
  });

  // verify that all of the steps have corresponding actions
  const graph = process.graph;
  const nodes = graph.nodes();

  // for each of the node ids stored in nodes, get the name of the action

  //loop through the nodes
  for (let i = 0; i < nodes.length; i++) {
    const name = await getNodeName(nodes[i]);
    if (name) {
      await addNode(nodes[i]);
    }
  }

  const my_edges = graph.edges();

  //loop through the edges
  for (let i = 0; i < my_edges.length; i++) {
    await addEdge(my_edges[i]);
  }

  // create a map from label to id
  // let label_to_id = new Map();

  // loop through the actions and make sure that they are all in the ai_system_state
  // for (let i = 0; i < nodes.length; i++) {
  //   let action_id = nodes[i];
  // check if the action is in the ai_system_state
  // for (let j = 0; j < ai_system_state.actions.length; j++) {
  //   let ai_system_action_id = ai_system_state.actions[j];
  //   if (ai_system_action.name == action) {
  //     // create a node

  //     let this_id = await getUniqueId();

  //     let node : Node= {
  //       id: this_id,
  //       type: "action",
  //       label: ai_system_action.name,
  //       data: ai_system_action,
  //     };

  // Get the name of the action by looking up the id within the system_state actions and set the l

  //loop through the nodes

  // if (i == 0) {
  //   // is the root node
  //   await addNode(node, true);
  // } else {
  //   await addNode(node,false);
  // }

  //   }

  // }

  // loop through the actions and create edges
  // for (let i = 0; i < actions.length - 1; i++) {
  // let edge = {
  //   id : await getUniqueId(),
  //   source: label_to_id.get(actions[i]),
  //   target: label_to_id.get(actions[i + 1]),
  //   label: "next",
  //   data: null,
  // };
  // await addEdge(edge);
  // }
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

export async function removeEdge(
  sourceId: string,
  targetId: string
): Promise<void> {
  const graphState = await getGraphState();
  // find the id of the edge to remove

  console.log("removing edge:", sourceId, targetId, " from graph");

  const edge = graphState.graph.edge(sourceId, targetId);
  graphState.graph.removeEdge(edge);

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
