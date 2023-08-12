import { writable } from "svelte/store";
import { SystemState } from "../system_types";

const system_state: SystemState = {
  authenticated: false,
  websocketReady: false,
  selectedNode: null,
  graphState: null,
  websocket: null,
  executionContext: null,
  nodes: []
};

const systemStateStore = writable(system_state);

export default systemStateStore;