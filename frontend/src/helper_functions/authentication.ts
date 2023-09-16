import { sendWebsocketMessage } from "./websocket";

import {
  AuthenticationMessage,
  CrudBundle,
  VerbTypeNames,
} from "../generated/system_types_pb";

export function authenticate(
  websocket: WebSocket,
  email: string,
  password: string,
) {

  console.log("websocket is ready... sending auth");
  const auth_bundle = new CrudBundle();
  auth_bundle.setVerb(VerbTypeNames.POST);

  const auth_content = new AuthenticationMessage();

  auth_content.setClientEmail(email);
  auth_content.setClientPassword(password);

  auth_bundle.setAuthenticationMessage(auth_content);

  sendWebsocketMessage(auth_bundle, websocket);

}
