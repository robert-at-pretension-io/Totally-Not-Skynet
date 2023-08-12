import { pipe } from "fp-ts/function";
import { fold } from "fp-ts/Either";
import { PathReporter } from "io-ts/lib/PathReporter";
import { getSystemState, setSystemState } from "./graph";
import { ResponseObject, RuntimeResponseObject, SystemState } from "system_types";

export async function setupWebsocketConnection() {
  let websocket = new WebSocket("ws://138.197.70.163:8080");
  const system_state = await getSystemState();

  // start the websocket connection
  websocket.addEventListener("open", async () => {
    // setup message processor
    websocket = await setupWebsocketMessageHandler(websocket, system_state);

    system_state.websocket_read = true;
    system_state.websocket = websocket;
    await setSystemState(system_state);

  });
}

export async function setupWebsocketMessageHandler(websocket: WebSocket, system_state: SystemState): Promise<WebSocket> {
  websocket.addEventListener("message", (event) => {
    console.log("websocket message received: ", event.data);
    let data: any;
    try {
      data = JSON.parse(event.data);

      const responseResult = RuntimeResponseObject.decode(data);

      pipe(
        responseResult,
        fold(
          (errors: any) => {
            console.log("Error decoding websocket message: ", errors);
            console.error(PathReporter.report(responseResult));
          },
          async (response_object: ResponseObject) => {
            // if response_object is a node then add it to the system state store
            if (
              typeof response_object === "object" &&
              response_object !== null &&
              "Node" in response_object
            ) {
              const { Node } = response_object;
              console.log(Node.type_name); // Will log "Prompt", "Process", "Conditional", or "Command"
              system_state.nodes.push({ Node });
            } else {
              console.log(
                "\n---------------\nresponse_object is not a node\n---------------\n"
              );
            }
            await setSystemState(system_state);
          }
        )
      );
    } catch {
      console.log("Error parsing websocket message");
    }
  });
  return websocket;

}