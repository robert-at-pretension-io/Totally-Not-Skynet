import { writable } from "svelte/store";
import { SystemState } from "../generated/system_types_pb";
import { initializeSystemState } from "helper_functions/misc";

const system_state: SystemState = initializeSystemState();

const systemStateStore = writable(system_state);

export default systemStateStore;
