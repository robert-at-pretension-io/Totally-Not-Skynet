import { sendEnvelope } from "./websocket";
import systemStateStore from "stores/systemStateStore";
import { v4 as uuidv4 } from "uuid";

import {
  AuthenticationMessage,
  Envelope,
  VerbTypes,
  Identity,
  Letter,
  Body
} from "../generated/system_types";

export function authenticate(
  websocket: WebSocket,
  email: string,
  password: string
) {

  console.log("websocket is ready... sending auth");

  const letter = new Letter();

  const auth_content = new AuthenticationMessage();

  letter.verb = VerbTypes.Initiate;

  auth_content.client_email = email;
  auth_content.client_password = password;

  const body = new Body();

  body.authentication_message = auth_content;

  letter.body = body;

  sendEnvelope(websocket, [letter]);

  systemStateStore.update((s) => {
    console.log("setting authenticated to true");
    s.authenticated = true;
    return s;
  });

}
