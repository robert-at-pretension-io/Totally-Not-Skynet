import { writable } from "svelte/store";
import type { GraphState } from "../system_types";

// Create the store
export const graphStore = writable<GraphState>({
  graph: {
    nodes: [],
    edges: []
  },
  selected: {
    type: "Node",
    instance: {
      id: "",
      source: null,
      target: null,
      label: "",
      data: undefined
    },
    neighbors: null,
    outgoing: null,
    incoming: null
  },
  lastAction: "none",
  actedOn: null
});