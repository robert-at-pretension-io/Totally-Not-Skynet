
import { ResponseObject, Node, SystemState, ValidateNodesResponse, Graph, CrudBundle } from "../../src/generated/system_types";

import systemStateStore from "stores/systemStateStore";

import { BinaryWriter } from "google-protobuf";
import { Json } from "io-ts-types";

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
      const res = response_object.object;
      console.log("res: ", res);

      switch (res) {
      case "node": {
        console.log("NODE");
        const add_node = response_object.node as Node;

        if (add_node && typeof add_node.toObject === "function") {
          console.log("add_node: ", add_node.toObject());

          systemStateStore.update(
            (n: SystemState) => {

              n.nodes.push(add_node);

              return n;
            }
          );

        }
        break;
      }
      case "authentication_message":
        console.log("AUTHENTICATION_MESSAGE");
        break;
      case "user_settings":
        console.log("USER_SETTINGS");
        break;
      case "validate_nodes_response": {
        const graph_container = response_object.validate_nodes_response as ValidateNodesResponse;
        const graph = graph_container.graph as Graph;
        systemStateStore.update(
          (n: SystemState) => {
            n.graph = graph;
            return n;
          }
        );
      }
        break;
      case "execution_response":
        console.log("EXECUTION_RESPONSE");
        break;
      case "none":
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
  console.log("sending websocket message: ", message.toObject());
  const message_string = message.serializeBinary();

  // const writer = new BinaryWriter();

  // message.serialize(writer);
  // const result = writer.getResultBase64String();

  // // message.serialize

  // // const messageArray = Array.from(message_string);

  console.log("serialized message is: ", message_string);
  // console.log("other serialize message", result);

  // // message_string.buffer

  // console.log("deserialized message is: ", CrudBundle.deserializeBinary(message_string).toObject());

  // console.log("other deserialzed message:", CrudBundle.deser)

  // const string = JSON.stringify(message.toObject());

  websocket.send(message_string);
}
