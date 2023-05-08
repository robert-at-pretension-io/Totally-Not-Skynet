import type { Node, Edge, GraphState, AiSystemState } from "../system_types";
import { graphStore } from "../stores/graphStore";
import { Process } from "../system_types";
import { aiSystemStore } from "../stores/aiSystemStore";

// Define the getter and setter

export async function getGraphState(): Promise<GraphState> {
  return new Promise((resolve, _reject) => {
    graphStore.subscribe((graphState: GraphState) => {
      resolve(graphState);
    });
  });
}

export function setGraphState(graphState: GraphState) {
  graphStore.set(graphState);
}

export async function addNode(node: Node): Promise<void> {
  const graphState = await getGraphState();
  graphState.graph.nodes.push(node);
  graphState.lastAction = "addNode";
  graphState.actedOn = node;
  setGraphState(graphState);
}

// function for converting a process to a graph
export async function processToGraph(process: Process): Promise<void> {
  await resetGraph();
  
  let ai_system_state : AiSystemState= await new Promise((resolve, _reject) => {
    aiSystemStore.subscribe((ai_system_state: AiSystemState) => {
      resolve(ai_system_state);
    });
  });

  console.log(ai_system_state);

  // verify that all of the steps have corresponding actions
  let actions = process.steps;

  // create a map from label to id
  let label_to_id = new Map();

  // loop through the actions and make sure that they are all in the ai_system_state
  for (let i = 0; i < actions.length; i++) {
    let action = actions[i];
    // check if the action is in the ai_system_state
    for (let j = 0; j < ai_system_state.actions.length; j++) {
      let ai_system_action = ai_system_state.actions[j];
      if (ai_system_action.name == action) {
        // create a node

        let this_id = await getUniqueId();

        let node = {
          id: this_id,
          label: ai_system_action.name,
          data: ai_system_action,
        };

        label_to_id.set(ai_system_action.name, this_id);

        await addNode(node);
      }
    }

  }

  // loop through the actions and create edges
  for (let i = 0; i < actions.length - 1; i++) {
    let edge = {
      id : await getUniqueId(),
      source: label_to_id.get(actions[i]),
      target: label_to_id.get(actions[i + 1]),
      label: "next",
      data: null,
    };
    await addEdge(edge);
  }

}

export async function updateNode(
  id: string,
  label: string,
  data: any
): Promise<void> {
  const graphState = await getGraphState();
  const node = graphState.graph.nodes.find((node) => node.id === id);
  if (node) {
    node.label = label;
    node.data = data;
    graphState.lastAction = "updateNode";
    graphState.actedOn = node;
    setGraphState(graphState);
  }
}

export async function updateEdge(
  id: string,
  label: string,
  data: any
): Promise<void> {
  const graphState = await getGraphState();
  const edge = graphState.graph.edges.find((edge) => edge.id === id);
  if (edge) {
    edge.label = label;
    edge.data = data;
    graphState.lastAction = "updateEdge";
    graphState.actedOn = edge;
    setGraphState(graphState);
  }
}

export async function addEdge(edge: Edge): Promise<void> {
  const graphState = await getGraphState();
  graphState.graph.edges.push(edge);
  graphState.lastAction = "addEdge";
  graphState.actedOn = edge;
  setGraphState(graphState);
}

export async function removeNode(id: string): Promise<void> {
  const graphState = await getGraphState();
  graphState.graph.nodes = graphState.graph.nodes.filter(
    (node) => node.id !== id
  );
  graphState.lastAction = "removeNode";
  graphState.actedOn = { id: id };
  setGraphState(graphState);
}

export async function removeEdge(
  sourceId: string,
  targetId: string
): Promise<void> {
  const graphState = await getGraphState();
  // find the id of the edge to remove

  console.log("removing edge:", sourceId, targetId, "from graph");

  const edge = graphState.graph.edges.find(
    (edge) => edge.source === sourceId && edge.target === targetId
  );
  if (edge) {
    graphState.graph.edges = graphState.graph.edges.filter(
      (edge) => edge.source !== sourceId && edge.target !== targetId
    );
    graphState.lastAction = "removeEdge";
    graphState.actedOn = { id: edge.id, source: sourceId, target: targetId };
    setGraphState(graphState);
  }
}

export async function selectNode(id: string): Promise<void> {
  const graphState = await getGraphState();
  const selectedNode = graphState.graph.nodes.find((node) => node.id === id);
  if (selectedNode && graphState.selected) {
    graphState.lastAction = "selectNode";
    graphState.actedOn = selectedNode;
    graphState.selected.instance = selectedNode;
    graphState.selected.type = "Node";
    graphState.selected.neighbors = graphState.graph.nodes.filter(
      (node) => node.id !== id
    );
    graphState.selected.outgoing = graphState.graph.edges.filter(
      (edge) => edge.source === id
    );
    graphState.selected.incoming = graphState.graph.edges.filter(
      (edge) => edge.target === id
    );
    setGraphState(graphState);
  }
}

export async function selectEdge(
  sourceId: string,
  targetId: string
): Promise<void> {
  const graphState = await getGraphState();
  const selectedEdge = graphState.graph.edges.find(
    (edge) => edge.source === sourceId && edge.target === targetId
  );
  if (selectedEdge && graphState.selected) {
    graphState.lastAction = "selectEdge";
    graphState.actedOn = selectedEdge;
    graphState.selected.instance = selectedEdge;
    graphState.selected.type = "Edge";
    graphState.selected.neighbors = graphState.graph.nodes.filter(
      (node) => node.id !== sourceId && node.id !== targetId
    );
    graphState.selected.outgoing = graphState.graph.edges.filter(
      (edge) => edge.source === sourceId
    );
    graphState.selected.incoming = graphState.graph.edges.filter(
      (edge) => edge.target === targetId
    );
    setGraphState(graphState);
  }
}

export async function resetLastAction(): Promise<void> {
  const graphState = await getGraphState();
  graphState.lastAction = "none";
  graphState.actedOn = null;
  setGraphState(graphState);
}

export async function nodes(): Promise<Node[]> {
  const graphState = await getGraphState();
  return graphState.graph.nodes;
}

export async function edges(): Promise<Edge[]> {
  const graphState = await getGraphState();
  return graphState.graph.edges;
}

function generateRandomId(): string {
  return Math.random().toString(36).substr(2, 9);
}

export async function getUniqueId(): Promise<string> {
  const graphState = await getGraphState();
  let id = "";
  do {
    id = generateRandomId();
  } while (
    graphState.graph.nodes.some((node) => node.id === id) ||
    graphState.graph.edges.some((edge) => edge.id === id)
  );
  return id;
}

// reset the graphState to a new empty graph
export async function resetGraph(): Promise<void> {
  
  const graphState = await getGraphState();
  graphState.graph = { nodes: [], edges: [] };
  graphState.lastAction = "resetGraph";
  graphState.actedOn = null;
  setGraphState(graphState);

}