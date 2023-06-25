<script lang="ts">
  // import GraphComponent from "./components/GraphComponent.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  // import RightSidebar from "./components/RightSidebar.svelte";
  import GraphComponentGraphlib from "./components/GraphComponent_graphlib.svelte";

  import "../public/global.css";

  import type {
    Process,
  } from "./system_types";

  import { processToGraph } from "helper_functions/graph";

  import { onMount } from "svelte";
  import systemStateStore from "stores/systemStateStore";

  onMount(async () => {
    // start the websocket connection
    $systemStateStore.websocket.addEventListener("open", () => {
      let apiKey = localStorage.getItem("apiKey") || "Api Key";
      let mongo_uri = localStorage.getItem("mongo_uri") || "Mongo Uri";
      localStorage.setItem("apiKey", apiKey);
      localStorage.setItem("mongo_uri", mongo_uri);
      $systemStateStore.websocket.send(
        JSON.stringify({ openai_api_key: apiKey, mongo_db_uri: mongo_uri })
      );

      $systemStateStore.websocket.send(
        JSON.stringify({ initial_message: "initial message" })
      );
    });
    $systemStateStore.websocket.addEventListener("message", (event) => {
      console.log("websocket message received: ", event.data);
      let data: any;
      try {
        data = JSON.parse(event.data);
      } catch {
        console.log("Error parsing websocket message");
      }

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
