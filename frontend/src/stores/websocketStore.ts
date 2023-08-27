import { writable } from "svelte/store";

const websocket: { websocket: null | WebSocket } = { websocket: null };

export const websocketStore = writable(websocket);
