import { sendWebsocketMessage } from "./websocket";

import {
  AuthenticationMessage,
  CrudBundle,
  VerbTypeNames,
} from "../generated/system_types_pb";

import { getSystemState, setSystemState } from "./graph";

export async function authenticate(
  websocket: WebSocket,
  email: string,
  password: string
) {
  let system_state = await getSystemState();

  if (system_state.getWebsocketReady()) {
    let auth_bundle = new CrudBundle();
    auth_bundle.setVerb(VerbTypeNames.POST);

    let auth_content = new AuthenticationMessage();

    auth_content.setClientEmail(email);
    auth_content.setClientPassword(password);

    auth_bundle.setAuthenticationMessage(auth_content);

    await sendWebsocketMessage(auth_bundle, websocket);
  }
}
