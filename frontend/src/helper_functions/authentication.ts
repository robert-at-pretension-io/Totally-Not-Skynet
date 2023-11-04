import { sendEnvelope, sendWebsocketMessage } from "./websocket";
import systemStateStore from "stores/systemStateStore";
import { v4 as uuidv4 } from "uuid";

import {
  AuthenticationMessage,
  Envelope,
  VerbTypes,
  Identity,
  Contents
} from "../generated/system_types";

export function authenticate(
  websocket: WebSocket,
  email: string,
  password: string
) {

  console.log("websocket is ready... sending auth");

  const contents = new Contents();

  const auth_content = new AuthenticationMessage();

  contents.verb = VerbTypes.Initiate;

  auth_content.client_email = email;
  auth_content.client_password = password;

  contents.authentication_message = auth_content;

  let client_identity: Identity;
  let server_identity: Identity;

  // Get the client and server identity from the systemStateStore
  systemStateStore.subscribe((s) => {
    client_identity = s.client_identity;
    server_identity = s.primary_backend;
  });

  sendEnvelope(websocket, client_identity, server_identity, [contents]);

  systemStateStore.update((s) => {
    console.log("setting authenticated to true");
    s.authenticated = true;
    return s;
  });

}
