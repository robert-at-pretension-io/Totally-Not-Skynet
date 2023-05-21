import { writable } from "svelte/store";

const websocket = new WebSocket("ws://0.0.0.0:8080");
const websocketStore = writable(websocket);

export default websocketStore;