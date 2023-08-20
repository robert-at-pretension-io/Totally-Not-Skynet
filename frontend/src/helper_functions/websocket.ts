import { getSystemState, setSystemState } from "./graph";
import { CrudBundle, ResponseObject } from "generated/system_types_pb.js";

import { stringToUint8Array } from "./misc";

export async function setupWebsocketConnection(): Promise<WebSocket> {
  let websocket = new WebSocket("ws://138.197.70.163:8080");
  const system_state = await getSystemState();

  // start the websocket connection
  websocket.addEventListener("open", async () => {
    // setup message processor
    websocket = await setupWebsocketMessageHandler(websocket);

    system_state.setWebsocketReady(true);
    await setSystemState(system_state);
  });

  return websocket;
}

export async function setupWebsocketMessageHandler(
  websocket: WebSocket
): Promise<WebSocket> {
  websocket.addEventListener("message", (event) => {
    console.log("websocket message received: ", event.data);
    let data: any;
    try {
      data = event.data;

      const u8Array = stringToUint8Array(data);

      const response_object = ResponseObject.deserializeBinary(u8Array);

      const res = response_object.getObjectCase();

      alert(
        "Need to handle switch statement for websocket message processing --> Adding object into local system state."
      );

      switch (res) {
      case ResponseObject.ObjectCase.NODE:
        console.log("NODE");
        break;
      case ResponseObject.ObjectCase.AUTHENTICATION_MESSAGE:
        console.log("AUTHENTICATION_MESSAGE");
        break;
      case ResponseObject.ObjectCase.USER_SETTINGS:
        console.log("USER_SETTINGS");
        break;
      case ResponseObject.ObjectCase.EXECUTION_RESPONSE:
        console.log("EXECUTION_RESPONSE");
        break;
      case ResponseObject.ObjectCase.OBJECT_NOT_SET:
        console.log("OBJECT_NOT_SET");
        break;
      default:
        console.log("default");
        alert(
          "Fallen through response object switch statement... This is not good."
        );
        break;
      }
    } catch {
      console.log("Error parsing websocket message");
    }
  });

  return websocket;
}

export async function sendWebsocketMessage(
  message: CrudBundle,
  websocket: WebSocket
) {
  console.log("sending websocket message: ", message);
  const message_string = JSON.stringify(message);
  websocket.send(message_string);
}
