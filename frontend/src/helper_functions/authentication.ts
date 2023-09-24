import { sendWebsocketMessage } from "./websocket";
import systemStateStore from "stores/systemStateStore";

import {
  AuthenticationMessage,
  CrudBundle,
  VerbTypeNames,
} from "../generated/system_types";

export function authenticate(
  websocket: WebSocket,
  email: string,
  password: string,
) {

  console.log("websocket is ready... sending auth");
  const auth_bundle = new CrudBundle();
  auth_bundle.verb = VerbTypeNames.Post;

  const auth_content = new AuthenticationMessage();

  auth_content.client_email = email;
  auth_content.client_password = password;

  auth_bundle.authentication_message = auth_content;

  systemStateStore.update((s) => {
    console.log("setting authenticated to true");
    s.authenticated = true;
    return s;
  });

  sendWebsocketMessage(auth_bundle, websocket);

}
