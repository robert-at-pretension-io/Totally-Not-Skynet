<script lang="ts">
  import Sidebar from "./components/Sidebar.svelte";
  import GraphComponentGraphlib from "./components/GraphComponent_graphlib.svelte";
  import "../public/global.css";
  import { setupWebsocketConnection } from "helper_functions/websocket";
  import { onMount } from "svelte";
  import systemStateStore from "stores/systemStateStore";

  onMount(async () => {
    if (!$systemStateStore.websocketReady) {
      // startup websocket connection
      await setupWebsocketConnection();
    }
  });
</script>

<div class="app-container">
  <!-- Show the following component if the system is not authenticated-->

  <AuthPage />

  <!-- Show the following two components if the system is authenticated-->
  <Sidebar />
  <GraphComponentGraphlib />
</div>
