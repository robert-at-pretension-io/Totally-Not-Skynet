import { writable } from "svelte/store";
import type { AiSystemState } from "../system_types";


// Create the store
export const aiSystemStore = writable<AiSystemState>(
  {
    actions: [],
    messages: [],
    processes: []
  }
);

