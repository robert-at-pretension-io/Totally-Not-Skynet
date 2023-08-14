import { writable } from "svelte/store";
import { SystemState } from "../system_types";

const system_state: SystemState = {
  authenticated: false,
  websocket_ready: false,
  selected_node: undefined,
  graph_state: {
    graph: { nodes: [], edges: [] }
  },
  nodes: []
};

const systemStateStore = writable(system_state);

export default systemStateStore;