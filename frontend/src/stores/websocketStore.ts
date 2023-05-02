import { writable } from "svelte/store";

const websocket = new WebSocket("ws://localhost:8080");
const websocketStore = writable(websocket);

export default websocketStore;