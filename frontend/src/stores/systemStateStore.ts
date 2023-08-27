import { writable } from "svelte/store";
import * as proto from "../generated/system_types_pb.js";

const system_state: proto.SystemState = new proto.SystemState();

const systemStateStore = writable(system_state);

export default systemStateStore;
