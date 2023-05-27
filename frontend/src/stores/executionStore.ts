import { writable } from "svelte/store";
import type { Execution } from "../system_types";

// Create the store
export const executionStore = writable<Execution>(
  {
    local_variables: new Map<string, string>(),
    global_variables: new Map<string, string>(),
    topological_order: [],
    prompts: new Map<string, string>(),
    responses: new Map<string, string>()
  }
);
