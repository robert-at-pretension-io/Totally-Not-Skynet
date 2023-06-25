import { writable } from "svelte/store";
import { SystemState } from "../system_types";
import { newGraphState, NewExecutionContext} from "helper_functions/type_checker";
import { none } from "fp-ts/lib/Option";

const system_state: SystemState = {
  websocketReady: false,
  selectedNode: none,
  graphState: newGraphState(),
  websocket: new WebSocket("ws://157.245.243.205:8080"),
  executionContext: NewExecutionContext(),
  nodes: [],
};

const systemStateStore = writable(system_state);

export default systemStateStore;