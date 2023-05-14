import { writable } from "svelte/store";
import { SystemState } from "../system_types";
import { newAction, newProcess } from "helper_functions/type_checker";

// Replace 'ws://example.com' with your WebSocket server URL
const system_state : SystemState = {
  websocketReady: false,
  currentlySelected: "action" || "process" || "none",
  selectedAction: newAction(),
  selectedProcess: newProcess(),
};

const systemStateStore = writable(system_state);

export default systemStateStore;