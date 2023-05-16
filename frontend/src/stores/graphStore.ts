import { writable } from "svelte/store";
import type { GraphState } from "../system_types";
import { Graph } from "@dagrejs/graphlib";

// Create the store
export const graphStore = writable<GraphState>({
  graph: new Graph(),
  lastAction: "none",
  actedOn: null,
  name: null
});