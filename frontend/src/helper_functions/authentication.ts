import { sendWebsocketMessage } from "./websocket";

import {
  AuthenticationMessage,
  CrudBundle,
  VerbTypeNames,
  SystemState
} from "../generated/system_types_pb";


export async function authenticate(
  websocket: WebSocket,
  email: string,
  password: string,
  system_state: SystemState
) {


  console.log(JSON.stringify(system_state.toObject()));

  if (system_state.getWebsocketReady()) {
    console.log("websocket is ready... sending auth");
    let auth_bundle = new CrudBundle();
    auth_bundle.setVerb(VerbTypeNames.POST);

    let auth_content = new AuthenticationMessage();

    auth_content.setClientEmail(email);
    auth_content.setClientPassword(password);

    auth_bundle.setAuthenticationMessage(auth_content);

    await sendWebsocketMessage(auth_bundle, websocket);
  }
}
