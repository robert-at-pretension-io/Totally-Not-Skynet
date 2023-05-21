import { writable } from "svelte/store";

const websocket = new WebSocket("ws://157.245.243.205:8080");
const websocketStore = writable(websocket);

export default websocketStore;