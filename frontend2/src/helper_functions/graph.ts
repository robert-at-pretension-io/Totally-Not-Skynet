import type { Node, Edge, GraphState } from "../system_types";
import { graphStore } from "../stores/graphStore";

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
