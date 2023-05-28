import { writable } from "svelte/store";
import { SystemState } from "../system_types";
import { newAction, newProcess, newGraphState, NewExecutionContext } from "helper_functions/type_checker";

// Replace 'ws://example.com' with your WebSocket server URL
const system_state : SystemState = {
  websocketReady: false,
  currentlySelected: "action" || "process" || "none",
  selectedAction: newAction(),
  selectedProcess: newProcess(),
  graphState: newGraphState(),
  websocket:  new WebSocket("ws://157.245.243.205:8080"),
  executionContext : NewExecutionContext(),
  aiSystemState: {
    actions: [],
    processes: [],
    messages: [],
  }
};

const systemStateStore = writable(system_state);

export default systemStateStore;