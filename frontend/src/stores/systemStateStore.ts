import { writable } from "svelte/store";
import { SystemState } from "../system_types";

const system_state: SystemState = {
  authenticated: false,
  websocket_read: false,
  selected_node: null,
  graph_state: null,
  websocket: null,
  execution_context: null,
  nodes: []
};

const systemStateStore = writable(system_state);

export default systemStateStore;