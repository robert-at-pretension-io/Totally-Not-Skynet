<<<<<<< Updated upstream
import { Node, SystemState, Graph } from "../../src/generated/system_types";
=======

import { Node, SystemState, Envelope } from "../../src/generated/system_types";
>>>>>>> Stashed changes

import systemStateStore from "stores/systemStateStore";

import { BinaryWriter } from "google-protobuf";
import { Json } from "io-ts-types";

export function setupWebsocketConnection(): WebSocket {
  console.log("setting up websocket connection");
  // let websocket = new WebSocket("ws://138.197.70.163:8080");
  alert("Change websocket to external environment");
  let websocket = new WebSocket("ws://127.0.0.1:8080");

  // start the websocket connection
  websocket.addEventListener("open", () => {

    console.log("websocket connection opened");
    // setup message processor
    websocket = setupWebsocketMessageHandler(websocket);
  });

  return websocket;
}

export function setupWebsocketMessageHandler(websocket: WebSocket): WebSocket {
  console.log("setting up websocket message handler");

  websocket.addEventListener("message", (event) => {
    console.log("websocket message received: ", event.data);

    event.data.arrayBuffer().then((buffer: any) => {
      console.log("buffer: ", buffer);
      const u8Array = new Uint8Array(buffer);
      console.log("u8Array: ", u8Array);
<<<<<<< Updated upstream
      const response_object: ResponseObject =
        ResponseObject.deserializeBinary(u8Array);
=======
      const response_object: Envelope = Envelope.deserializeBinary(u8Array);
>>>>>>> Stashed changes

      console.log("response_object: ", response_object);
      const res = response_object;
      console.log("res: ", response_object.toObject());

      if (res.me)

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
          const process = graph_container.process as Node;

          console.log("process: ", process.toObject());

<<<<<<< Updated upstream
          systemStateStore.update((n: SystemState) => {
            n.nodes.push(add_node);

            return n;
          });
        }
        break;
      }
      case "authentication_message":
        console.log("AUTHENTICATION_MESSAGE");
        break;
      case "user_settings":
        console.log("USER_SETTINGS");
        break;
      case "validate_nodes_response":
        {
          const graph_container =
              response_object.validate_nodes_response as ValidateNodesResponse;
          const process = graph_container.process as Node;

          console.log("process: ", process.toObject());

          systemStateStore.update((n: SystemState) => {
            n.selected_process = process;
            n.nodes.push(process);
            return n;
          });
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
=======
          systemStateStore.update(
            (n: SystemState) => {
              n.selected_process = process;
              n.nodes.push(process);
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
>>>>>>> Stashed changes
    });
  });

  return websocket;
}

export function sendWebsocketMessage(
  message: Envelope,
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
