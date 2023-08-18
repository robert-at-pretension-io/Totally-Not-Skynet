import { writable } from "svelte/store";
import { SystemState } from "generated/system_types_pb.js";

const system_state: SystemState = new SystemState();

const systemStateStore = writable(system_state);

export default systemStateStore;