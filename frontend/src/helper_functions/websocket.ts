import { getSystemState, setSystemState } from "./graph";
import { CrudBundle } from "system_types";



export async function setupWebsocketConnection(email: string, pass: string) {
    let websocket = new WebSocket("ws://138.197.70.163:8080");
    let system_state = await getSystemState();


    // start the websocket connection
    websocket.addEventListener("open", () => {

    });

    $systemStateStore.websocket.addEventListener("message", (event) => {
        console.log("websocket message received: ", event.data);
        let data: any;
        try {
            data = JSON.parse(event.data);

            let responseResult = RuntimeResponseObject.decode(data);
            fold(
                (errors) => {
                    console.log("Error decoding websocket message: ", errors);
                    console.error(PathReporter.report(responseResult));
                },
                (response_object: ResponseObject) => {
                    // if response_object is a node then add it to the system state store

                    if (
                        typeof response_object === "object" &&
                        response_object !== null &&
                        "Node" in response_object
                    ) {
                        const { Node } = response_object;

                        // Now you can access Node.type_name to further check its subtype
                        console.log(Node.type_name); // Will log "Prompt", "Process", "Conditional", or "Command"

                        $systemStateStore.nodes.push({ Node });
                    } else {
                        console.log(
                            "\n---------------\nresponse_object is not a node\n---------------\n"
                        );
                    }
                }
            )(responseResult);
        } catch {
            console.log("Error parsing websocket message");
        }
    });

}