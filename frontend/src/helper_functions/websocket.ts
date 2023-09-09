
import { SystemState, ResponseObject, CrudBundle, Node } from "../../src/generated/system_types_pb";

import systemStateStore from "stores/systemStateStore";

export async function setupWebsocketConnection(
  system_state: SystemState
): Promise<[WebSocket, SystemState]> {
  console.log("setting up websocket connection");
  let websocket = new WebSocket("ws://138.197.70.163:8080");

  // start the websocket connection
  websocket.addEventListener("open", () => {
    console.log("websocket connection opened");
    // setup message processor

    system_state.setWebsocketReady(true);
    systemStateStore.set(system_state); // <-- update your Svelte store
  });

  websocket = await setupWebsocketMessageHandler(websocket);

  console.log("returning websocket");

  return [websocket, system_state];
}

export async function setupWebsocketMessageHandler(
  websocket: WebSocket
): Promise<WebSocket> {
  websocket.addEventListener("message", (event) => {
    console.log("websocket message received: ", event.data);

    event.data.arrayBuffer().then(async (buffer: any) => {
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

          // import { SystemState } from "../../src/generated/system_types_pb";
          // import systemStateStore from "stores/systemStateStore";

          // const system_state = systemStateStore.
          //   console.log("system_state: ", system_state.toObject());

          // let node_list = [];

          systemStateStore.subscribe((system_state: SystemState) => {
            console.log("Type of system_state:", typeof system_state);
            console.log("Keys of system_state:", Object.keys(system_state));

            if (system_state && typeof system_state.toObject === "function") {

              console.log("system_state: ", system_state.toObject());
              const node_list = system_state.getNodesList();
              console.log("node_list: ", node_list);

              system_state.setNodesList(node_list);

              systemStateStore.set(system_state);
            }
          });

          // node_list.push(add_node);

          // console.log("node_list after push: ", node_list);

          // systemStateStore.update(
          //   (n: SystemState) => {

          //     console.log("n: ", n);

          //     const m = n as SystemState;

          //     console.log("m: ", m);

          //     const nodes = m.getNodesList();
          //     nodes.push(add_node);
          //     m.setNodesList(nodes);
          //     return m;
          //   }
          // );

          // console.log("system_state: ", system_state.toObject());

          // const nodes = system_state.getNodesList();

          // nodes.push(add_node);
          // system_state.setNodesList(nodes);
          // await setSystemState(system_state);
        }
        break;
      }
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
    });
  });

  return websocket;
}

export async function sendWebsocketMessage(
  message: CrudBundle,
  websocket: WebSocket
) {
  console.log("sending websocket message: ", message);
  const message_string = message.serializeBinary();
  websocket.send(message_string);
}
