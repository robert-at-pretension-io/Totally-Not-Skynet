import { writable } from "svelte/store";
import * as proto from 

const system_state: proto.skynet.types.SystemState = new proto.skynet.types.SystemState();

const systemStateStore = writable(system_state);

export default systemStateStore;
