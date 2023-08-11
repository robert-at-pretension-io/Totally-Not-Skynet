import { sendWebsocketMessage } from "./graph";
import { CrudBundle } from "system_types";


export async function authenticate(email: string, password: string) {


    const initial_message: CrudBundle = {
        verb: "POST",
        object: {
            InitialMessage: {
                initial_message: "",
                client_email: email,
                client_password: password
            },
        },
    };
    sendWebsocketMessage(initial_message);
}

const initial_message: CrudBundle = {
    verb: "POST",
    object: {
        InitialMessage: {
            initial_message: "",
        },
    },
};
sendWebsocketMessage(initial_message);