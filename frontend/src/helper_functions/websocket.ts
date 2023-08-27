import { stringToUint8Array } from "./misc";

import * as proto from "../../src/generated/system_types_pb";

import systemStateStore from "stores/systemStateStore";

export function setupWebsocketConnection(
  system_state: proto.SystemState
): [WebSocket, proto.SystemState] {
  console.log("setting up websocket connection");
  let websocket = new WebSocket("ws://138.197.70.163:8080");

  // start the websocket connection
  websocket.addEventListener("open", () => {
    console.log("websocket connection opened");
    // setup message processor

    system_state.setWebsocketReady(true);
    systemStateStore.set(system_state); // <-- update your Svelte store
  });

  websocket = setupWebsocketMessageHandler(websocket);

  console.log("returning websocket");

  return [websocket, system_state];
}

export function setupWebsocketMessageHandler(websocket: WebSocket): WebSocket {
  websocket.addEventListener("message", (event) => {
    console.log("websocket message received: ", event.data);
    let data: any;
    try {
      data = event.data;

      const u8Array = stringToUint8Array(data);

      const response_object = proto.ResponseObject.deserializeBinary(u8Array);

      const res = response_object.getObjectCase();

      alert(
        "Need to handle switch statement for websocket message processing --> Adding object into local system state."
      );

      switch (res) {
      case proto.ResponseObject.ObjectCase.NODE:
        console.log("NODE");
        break;
      case proto.ResponseObject.ObjectCase.AUTHENTICATION_MESSAGE:
        console.log("AUTHENTICATION_MESSAGE");
        break;
      case proto.ResponseObject.ObjectCase.USER_SETTINGS:
        console.log("USER_SETTINGS");
        break;
      case proto.ResponseObject.ObjectCase.EXECUTION_RESPONSE:
        console.log("EXECUTION_RESPONSE");
        break;
      case proto.ResponseObject.ObjectCase.OBJECT_NOT_SET:
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
  message: proto.CrudBundle,
  websocket: WebSocket
) {
  console.log("sending websocket message: ", message);
  const message_string = message.serializeBinary();
  websocket.send(message_string);
}
