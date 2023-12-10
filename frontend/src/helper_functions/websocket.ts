import {
  Node,
  SystemState,
  Envelope,
  VerbTypes,
  Letter,
  Body,
} from "../../src/generated/system_types";

import systemStateStore from "stores/systemStateStore";

import { BinaryWriter } from "google-protobuf";
import { Identity } from "generated/system_types";

import { v4 as uuidv4 } from "uuid";
import { getNodes } from "./misc";

export function setupWebsocketConnection(): WebSocket {
  console.log("setting up websocket connection");

  const environment: string = process.env.ENVIRONMENT;

  console.log("The environment is: ", environment);

  const websocket_url =
    environment.toUpperCase() === "PRODUCTION"
      ? "wss://liminalnook.com/ws/"
      : "ws://localhost:8080";

  let websocket = new WebSocket(websocket_url);

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

    console.log("\n------------------\n");

    event.data.arrayBuffer().then((buffer: any) => {
      const u8Array = new Uint8Array(buffer);
      const response_envelope: Envelope = Envelope.deserializeBinary(u8Array);

      console.log("received message: ", response_envelope.toObject());

      let self_identity = new Identity();

      // get the SystemState from the SystemStateStore
      systemStateStore.subscribe((s: SystemState) => {
        self_identity = s.client_identity;
      });

      // console.log(self_identity.toObject());

      if (!response_envelope.has_receiver) {
        alert("This message does not have a receiver. This is not good.");
      }

      if (
        response_envelope.has_receiver &&
        response_envelope.receiver.id !== self_identity.id
      ) {
        alert(
          "Rerouting the message to the correct client. This message is not for me."
        );
      }

      if (
        response_envelope.has_receiver &&
        response_envelope.receiver.id === self_identity.id
      ) {
        // loop through the response_envelope.letters array
        response_envelope.letters.forEach((letter) => {
          console.log("letter: ", letter);
          // check the type of letter

          if (letter.verb === VerbTypes.Initiate) {
            if (letter.body.has_node) {
              // if it's a node, add it to the SystemState
              const add_node = letter.body.node as Node;

              if (add_node && typeof add_node.toObject === "function") {
                console.log("add_node: ", add_node.toObject());

                systemStateStore.update((n: SystemState) => {
                  n.local_nodes.push(add_node);

                  return n;
                });
              }
            }
          }

          if (letter.verb === VerbTypes.Acknowledge) {
            if (letter.body.has_identity) {
              const identity = letter.body.identity as Identity;
              console.log(
                "setting primary server identity: ",
                identity.toObject()
              );
              systemStateStore.update((s: SystemState) => {
                s.primary_backend = identity;
                return s;
              });

              getNodes();
            }
            if (letter.body.has_node) {
              //add it to the system state local_node list:
              console.log("Added node to local system state");
              systemStateStore.update((s) => {
                if (letter.body.node.node_content.has_process) {
                  s.selected_process = letter.body.node;

                }
                s.local_nodes.push(letter.body.node);
                return s;
              });
            }
            if (letter.body.has_execution_details) {
              console.log("EXECUTION_DETAILS");
              console.log(letter.body.execution_details.toObject());
              systemStateStore.update((s) => {
                s.execution_results.push(letter.body.execution_details);
                return s;
              });
            }
          }

          if (letter.verb === VerbTypes.Error) {
            alert("Backend returned an error. Check the console log");
            console.error("Error: ", letter.body);
          }
        });
      } else {
        console.log("This message is not for me.");
      }
    });

  });

  return websocket;
}

function sendWebsocketMessage(message: Envelope, websocket: WebSocket) {
  console.log("sending message: ", message.toObject());
  const message_string = message.serializeBinary();

  websocket.send(message_string);
}

export function sendEnvelope(
  websocket: WebSocket,
  letters: Letter[],
  sender: Identity = undefined,
  receiver: Identity = undefined
) {
  // raise an error and alert if the sender or receiver is not set
  if (!sender) {
    console.log("Sender not set. Defaulting to this client.");

    systemStateStore.subscribe((s: SystemState) => {
      sender = s.client_identity;
    });

    if (sender == undefined) {
      alert("this client identity not defined");
    } else {
      console.log("Setting client identity: ", sender.toObject());
    }
  }

  // same for the receiver:
  if (!receiver) {
    console.log("Receiver not set. Defaulting to the primary backend.");

    systemStateStore.subscribe((s: SystemState) => {
      receiver = s.primary_backend;
    });

    if (receiver == undefined) {
      console.log("primary backend not defined");
    } else {
      console.log("Setting receiver identity: ", receiver.toObject());
    }
  }

  const envelope = new Envelope();

  envelope.sender = sender;
  envelope.receiver = receiver;

  // create uuid for the valididation id
  const verification_id = uuidv4();

  envelope.verification_id = verification_id;

  envelope.letters = letters;

  sendWebsocketMessage(envelope, websocket);
}

export function selfIdentify(): Identity {
  const id = uuidv4();

  const identity = new Identity();
  identity.id = id;
  getExternalIP().then((ip) => {
    identity.ip_address = ip;
  });

  return identity;
}

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
