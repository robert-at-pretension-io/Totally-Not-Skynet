import { sendWebsocketMessage } from "./websocket";
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
  password: string,
  client_identity: Identity,
  server_identity: Identity
) {

  console.log("websocket is ready... sending auth");

  const envelope = new Envelope();

  const contents = new Contents();

  const auth_content = new AuthenticationMessage();

  contents.verb = VerbTypes.Initiate;

  auth_content.client_email = email;
  auth_content.client_password = password;

  contents.authentication_message = auth_content;

  envelope.message_content = [contents];
  envelope.sender = client_identity;
  envelope.receiver = server_identity;

  systemStateStore.update((s) => {
    console.log("setting authenticated to true");
    s.authenticated = true;
    return s;
  });

  sendWebsocketMessage(envelope, websocket);

}
