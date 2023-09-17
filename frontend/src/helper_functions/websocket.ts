
import { SystemState, ResponseObject, CrudBundle, Node, ValidateNodesResponse } from "../../src/generated/system_types_pb";

import systemStateStore from "stores/systemStateStore";

export function setupWebsocketConnection(
): WebSocket {
  console.log("setting up websocket connection");
  let websocket = new WebSocket("ws://138.197.70.163:8080");

  // start the websocket connection
  websocket.addEventListener("open", () => {
    console.log("websocket connection opened");
    // setup message processor
    websocket = setupWebsocketMessageHandler(websocket);

  });

  return websocket;
}

export function setupWebsocketMessageHandler(
  websocket: WebSocket
): WebSocket {

  console.log("setting up websocket message handler");

  websocket.addEventListener("message", (event) => {
    console.log("websocket message received: ", event.data);

    event.data.arrayBuffer().then((buffer: any) => {
      console.log("buffer: ", buffer);
      const u8Array = new Uint8Array(buffer);
      console.log("u8Array: ", u8Array);
      const response_object: ResponseObject = ResponseObject.deserializeBinary(u8Array);
      console.log("response_object: ", response_object);
      const res = response_object.getObjectCase();
      console.log("res: ", res);

      switch (res) {
      case ResponseObject.ObjectCase.NODE: {
        console.log("NODE");
        const add_node = response_object.getNode() as Node;

        if (add_node && typeof add_node.toObject === "function") {
          console.log("add_node: ", add_node.toObject());

          systemStateStore.update(
            (n: SystemState) => {

              const nodes = n.getNodesList();
              nodes.push(add_node);
              n.setNodesList(nodes);
              return n;
            }
          );

        }
        break;
      }
      case ResponseObject.ObjectCase.AUTHENTICATION_MESSAGE:
        console.log("AUTHENTICATION_MESSAGE");
        break;
      case ResponseObject.ObjectCase.USER_SETTINGS:
        console.log("USER_SETTINGS");
        break;
      case ResponseObject.ObjectCase.VALIDATE_NODES_RESPONSE: {
        const graph_container = response_object.getValidateNodesResponse() as ValidateNodesResponse;
        const graph = graph_container.getGraph();
        systemStateStore.update(
          (n: SystemState) => {
            n.setGraph(graph);
            return n;
          }
        );
      }
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
    });
  });

  return websocket;
}

export function sendWebsocketMessage(
  message: CrudBundle,
  websocket: WebSocket
) {
  console.log("sending websocket message: ", message);
  const message_string = message.serializeBinary();
  console.log("serialized message is: ", message_string);

  websocket.send(message_string);
}
