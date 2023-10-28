
import { Node, SystemState, Envelope, VerbTypes, Contents } from "../../src/generated/system_types";

import systemStateStore from "stores/systemStateStore";

import { BinaryWriter } from "google-protobuf";
import { Json } from "io-ts-types";
import { Identity } from "generated/system_types";

import { v4 as uuidv4 } from "uuid";

export function setupWebsocketConnection(): WebSocket {
  console.log("setting up websocket connection");
  // let websocket = new WebSocket("ws://138.197.70.163:8080");
  alert("Change websocket to external environment");
  let websocket = new WebSocket("ws://127.0.0.1:8080");

  // start the websocket connection
  websocket.addEventListener("open", () => {
    console.log("websocket connection opened");
    // need to prepare the handler of incoming messages before sending the first message (in case the rust server is TOO FAST 😎)
    websocket = setupWebsocketMessageHandler(websocket);
  });

  console.log("Event listener setup:");

  return websocket;
}

export function setupWebsocketMessageHandler(websocket: WebSocket): WebSocket {
  console.log("setting up websocket message handler");

  websocket.addEventListener("message", (event) => {
    // console.log("websocket message received: ", event.data);

    event.data.arrayBuffer().then((buffer: any) => {
      console.log("buffer: ", buffer);
      const u8Array = new Uint8Array(buffer);
      console.log("u8Array: ", u8Array);
      const response_envelope: Envelope = Envelope.deserializeBinary(u8Array);

      let self_identity = new Identity();

      // get the SystemState from the SystemStateStore
      systemStateStore.subscribe((s: SystemState) => {
        self_identity = s.client_identity;
      });

      console.log(self_identity.toObject());

      if (response_envelope.has_receiver && response_envelope.receiver.id !== self_identity.id) {
        alert("Rerouting the message to the correct client. This message is not for me.");
      }

      if (response_envelope.has_receiver && response_envelope.receiver.id === self_identity.id) {
        // loop through the response_envelope.message_content array
        response_envelope.message_content.forEach((message_content) => {
          console.log("message_content: ", message_content);
          // check the type of message_content

          if (message_content.verb === VerbTypes.Initiate) {
            if (message_content.has_node) {
              // if it's a node, add it to the SystemState
              const add_node = message_content.node as Node;

              if (add_node && typeof add_node.toObject === "function") {
                console.log("add_node: ", add_node.toObject());

                systemStateStore.update(
                  (n: SystemState) => {

                    n.nodes.push(add_node);

                    return n;
                  }
                );

              }
            }
          }

          if (message_content.verb === VerbTypes.Acknowledge) {
            if (message_content.has_identity) {
              const identity = message_content.identity as Identity;
              console.log("server identity: ", identity);
              systemStateStore.update((s: SystemState) => {
                s.primary_backend = identity;
                return s;
              });
            }
          }

        });
      }

    });

    //       console.log("response_object: ", response_object);
    //       const res = response_object;
    //       console.log("res: ", response_object.toObject());

    //       if (res.me)

    //         switch (res) {
    //           case "node": {
    //             console.log("NODE");
    //             const add_node = response_object.node as Node;

    //             if (add_node && typeof add_node.toObject === "function") {
    //               console.log("add_node: ", add_node.toObject());

    //               systemStateStore.update(
    //                 (n: SystemState) => {

    //                   n.nodes.push(add_node);

    //                   return n;
    //                 }
    //               );

    //             }
    //             break;
    //           }
    //           case "authentication_message":
    //             console.log("AUTHENTICATION_MESSAGE");
    //             break;
    //           case "user_settings":
    //             console.log("USER_SETTINGS");
    //             break;
    //           case "validate_nodes_response": {
    //             const graph_container = response_object.validate_nodes_response as ValidateNodesResponse;
    //             const process = graph_container.process as Node;

    //             console.log("process: ", process.toObject());

    //             systemStateStore.update((n: SystemState) => {
    //               n.nodes.push(add_node);

    //               return n;
    //             });
    //           }
    //             break;
    //         }
    //       case "authentication_message":
    //       console.log("AUTHENTICATION_MESSAGE");
    //       break;
    //       case "user_settings":
    //       console.log("USER_SETTINGS");
    //       break;
    //       case "validate_nodes_response":
    //       {
    //         const graph_container =
    //           response_object.validate_nodes_response as ValidateNodesResponse;
    //         const process = graph_container.process as Node;

    //         console.log("process: ", process.toObject());

    //         systemStateStore.update((n: SystemState) => {
    //           n.selected_process = process;
    //           n.nodes.push(process);
    //           return n;
    //         });
    //       }
    //       break;
    //       case "execution_response":
    //       console.log("EXECUTION_RESPONSE");
    //       break;
    //       case "none":
    //       console.log("OBJECT_NOT_SET");
    //       break;
    //       default:
    //       console.log("default");
    //       alert(
    //         "Fallen through response object switch statement... This is not good."
    //       );
    //       break;
    //     }
    //     });
    // });

  });

  selfIdentify(websocket);

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

export function selfIdentify(websocket: WebSocket) {
  const id = uuidv4();

  const identity = new Identity();
  identity.id = id;
  getExternalIP().then((ip) => {
    identity.ip_address = ip;
  });

  systemStateStore.update((s) => {
    console.log("self-identify");
    s.client_identity = identity;
    return s;
  });

  const envelope = new Envelope();
  envelope.sender = identity;

  const contents = new Contents();

  contents.verb = VerbTypes.Initiate;

  contents.identity = identity;

  envelope.message_content = [contents];

  sendWebsocketMessage(envelope, websocket);

}

// import axios from "axios";

async function getExternalIP() {
  // try {
  //   const response = await axios.get("http://api.ipify.org");
  //   console.log(`My external IP address is: ${response.data}`);
  //   return response.data;
  // } catch (error) {
  //   console.error(`Error fetching IP address: ${error}`);
  // }
  return "placeholder_frontend_ip";
}
