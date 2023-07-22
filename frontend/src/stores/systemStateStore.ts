import { writable } from "svelte/store";
import { SystemState } from "../system_types";
import { newGraphState, NewExecutionContext } from "helper_functions/type_checker";

const system_state: SystemState = {
  websocketReady: false,
  selectedNode: null,
  graphState: newGraphState(),
  websocket: new WebSocket("ws://157.245.243.205:8080"),
  executionContext: NewExecutionContext(),
  nodes: [],
};

const systemStateStore = writable(system_state);

export default systemStateStore;