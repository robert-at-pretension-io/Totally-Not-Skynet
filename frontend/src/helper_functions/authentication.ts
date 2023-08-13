import { sendWebsocketMessage } from "./graph";
import { CrudBundle } from "system_types";
import { getSystemState, setSystemState } from "./graph";


export async function authenticate(email: string, password: string) {

    let system_state = await getSystemState();

    if (system_state.websocket_ready) {
        const authMessage: CrudBundle = {
            verb: "POST",
            object: {
                AuthenticationMessage: {
                    client_email: email,
                    client_password: password
                },
            },
        };
        await sendWebsocketMessage(authMessage);
    }

}


