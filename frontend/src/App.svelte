<script lang="ts">
  // import GraphComponent from "./components/GraphComponent.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  // import RightSidebar from "./components/RightSidebar.svelte";
  import GraphComponentGraphlib from "./components/GraphComponent_graphlib.svelte";

  import "../public/global.css";

  import { sendWebsocketMessage } from "helper_functions/graph";

  import type { CrudBundle, Node } from "./system_types";
  import {RuntimeCrudBundle } from "./system_types";

  import { fold } from "fp-ts/lib/Either";

  import { onMount } from "svelte";
  import systemStateStore from "stores/systemStateStore";

  onMount(async () => {

    console.log("on mount");

    // start the websocket connection
    $systemStateStore.websocket.addEventListener("open", () => {
      let apiKey = localStorage.getItem("apiKey") || "Api Key";
      let mongo_uri = localStorage.getItem("mongo_uri") || "Mongo Uri";
      localStorage.setItem("apiKey", apiKey);
      localStorage.setItem("mongo_uri", mongo_uri);

      let user_settings: CrudBundle = {
        verb: "POST",
        object: {
          UserSettings:
          {openai_api_key: "",
            mongo_db_uri: "",}
        },
      };

      sendWebsocketMessage(user_settings);

      const initial_message: CrudBundle = {
        verb: "POST",
        object: {
          InitialMessage: {
            initial_message: "",
          }
        },
      };

      sendWebsocketMessage(initial_message);
    });
    $systemStateStore.websocket.addEventListener("message", (event) => {
      console.log("websocket message received: ", event.data);
      let data: any;
      try {
        data = JSON.parse(event.data);

        let validationResult = RuntimeCrudBundle.decode(data);

        fold(
          (errors) => {
            console.log("Error decoding websocket message: ", errors);
          },
          (node: Node) => {
            $systemStateStore.nodes.push(node);
          }
        )(validationResult);
      } catch {
        console.log("Error parsing websocket message");
      }

      // if the websocket message is a node then add it to the system state store
    });
  });

  // async function handleProcessChange(process: Process) {
  //   // console.log("selected process changed: ", process);
  //   await processToGraph(process);
  // }

  // $: {
  //   let process = $systemStateStore.selectedProcess;
  //   if (
  //     lastSelectedProcess == null ||
  //     (process && process.name !== lastSelectedProcess.name)
  //   ) {
  //     handleProcessChange(process);
  //     lastSelectedProcess = process;
  //   }
  // }
</script>

<div class="app-container">
  <Sidebar />
  <GraphComponentGraphlib />
</div>
