import { getSystemState, setSystemState } from "./graph";



export async function setupWebsocketConnection() {
    new WebSocket("ws://138.197.70.163:8080");
    let system_state = await getSystemState();


}