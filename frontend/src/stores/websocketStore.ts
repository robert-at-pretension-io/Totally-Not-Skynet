import { writable } from "svelte/store";

const websocket: any = null;

export const websocketStore = writable(websocket);
