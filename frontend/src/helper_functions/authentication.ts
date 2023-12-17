import { sendEnvelope } from "./websocket";
import systemStateStore from "stores/systemStateStore";
import { v4 as uuidv4 } from "uuid";

import {
  AuthenticationMessage,
  Envelope,
  VerbTypes,
  Identity,
  Letter,
  Body,
  Secrets
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

  const secret = new Secrets();

  secret.email = email;
  secret.password = password;

  auth_content.secrets = secret;

  const body = new Body();

  body.authentication_message = auth_content;

  letter.body = body;

  console.log("Sending authentication letter:", letter.toObject());

  sendEnvelope(websocket, [letter]);

  // Only authenticate if there is a session id

}
