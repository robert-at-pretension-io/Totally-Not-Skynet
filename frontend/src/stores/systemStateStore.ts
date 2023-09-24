import { writable } from "svelte/store";
import { SystemState } from "../generated/system_types";
// import { initializeSystemState } from "helper_functions/misc";

const new_system_state: SystemState = new SystemState();

const systemStateStore = writable<SystemState>(new_system_state);

export default systemStateStore;
