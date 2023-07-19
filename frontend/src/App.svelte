<script lang="ts">
  // import GraphComponent from "./components/GraphComponent.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  // import RightSidebar from "./components/RightSidebar.svelte";
  import GraphComponentGraphlib from "./components/GraphComponent_graphlib.svelte";

  import "../public/global.css";

  import { sendWebsocketMessage } from "helper_functions/graph";

  import type { CrudBundle, ResponseObject } from "./system_types";
  import {  RuntimeResponseObject } from "./system_types";

  import { fold } from "fp-ts/lib/Either";

  import { onMount } from "svelte";
  import systemStateStore from "stores/systemStateStore";

  import { PathReporter } from "io-ts/PathReporter";

  onMount(async () => {
    console.log("on mount");

    // start the websocket connection
    $systemStateStore.websocket.addEventListener("open", () => {
      let apiKey = localStorage.getItem("apiKey") || "Api Key";
      let mongo_uri = localStorage.getItem("mongo_uri") || "Mongo Uri";
      localStorage.setItem("apiKey", apiKey);
      localStorage.setItem("mongo_uri", mongo_uri);

      let user_settings: CrudBundle = {
        verb: "GET",
        object: {
          UserSettings: { openai_api_key: "", mongo_db_uri: "" },
        },
      };

      sendWebsocketMessage(user_settings);

      const initial_message: CrudBundle = {
        verb: "POST",
        object: {
          InitialMessage: {
            initial_message: "",
          },
        },
      };

      sendWebsocketMessage(initial_message);
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

            if (typeof response_object === "object" && response_object !== null  && "Node" in response_object) {
              const { Node } = response_object;

              // Now you can access Node.type_name to further check its subtype
              console.log(Node.type_name);  // Will log "Prompt", "Process", "Conditional", or "Command"

              $systemStateStore.nodes.push({ Node });
            }
            else {
              console.log("\n---------------\nresponse_object is not a node\n---------------\n");
            }
          }
        )(responseResult);
      } catch {
        console.log("Error parsing websocket message");
      }

    });
  });

</script>

<div class="app-container">
  <Sidebar />
  <GraphComponentGraphlib />
</div>
