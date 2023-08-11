<script lang="ts">
  import Sidebar from "./components/Sidebar.svelte";
  import GraphComponentGraphlib from "./components/GraphComponent_graphlib.svelte";
  import "../public/global.css";
  import { sendWebsocketMessage } from "helper_functions/graph";
  import * from "helper_functions/websocket"
  import type { CrudBundle, ResponseObject } from "./system_types";
  import { RuntimeResponseObject } from "./system_types";
  import { fold } from "fp-ts/lib/Either";
  import { onMount } from "svelte";
  import systemStateStore from "stores/systemStateStore";
  import { PathReporter } from "io-ts/PathReporter";

  onMount(async () => {
    // Check Authentication
    if (!$systemStateStore.authenticated) {
      // needs auth token
      if ($systemStateStore.websocket == null) {
        // startup websocket connection
      }
    }

  });
</script>

<div class="app-container">
  <Sidebar />
  <GraphComponentGraphlib />
</div>
