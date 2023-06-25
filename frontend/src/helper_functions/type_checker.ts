import { Graph } from "@dagrejs/graphlib";
import { GraphState,  ExecutionContext } from "system_types";

export function newGraphState(): GraphState {
  return {
    graph: new Graph(), // replace with correct way to create a new Graph object
    lastAction: "none",
    actedOn: null,
    lastActedOn: null,
    name: null,
    global_variables: new Map<string, string>(),
  };
}

export function NewExecutionContext(): ExecutionContext {
  return {
    local_variables: new Map<string, string>(),
    global_variables: new Map<string, string>(),
    topological_order: [],
    topological_order_names: [],
    current_node: null,
    prompts: new Map<string, string>(),
    responses: new Map<string, string>(),
  };
}