<script lang="ts">
  import Sidebar from "./components/Sidebar.svelte";
  import GraphComponentGraphlib from "./components/GraphComponent_graphlib.svelte";
  import "../public/global.css";
  import { setupWebsocketConnection } from "helper_functions/websocket";
  import { onMount } from "svelte";
  import systemStateStore from "stores/systemStateStore";
  import { websocketStore } from "stores/websocketStore";
  import AuthPage from "./components/AuthPage.svelte";

  let authenticated = false;
  let websocket: WebSocket;

  onMount(async () => {
    if (!$systemStateStore.getWebsocketReady()) {
      // startup websocket connection
      websocket = await setupWebsocketConnection();
      websocketStore.set(websocket);
    }
  });

  $: {
    authenticated = $systemStateStore.getAuthenticated();
    if ($systemStateStore.getWebsocketReady()) {
      console.log("Websocket Ready to send Messages!");
    }
  }
</script>

<!-- Show the following component if the system is not authenticated-->
{#if !authenticated}
  <AuthPage />
{/if}

{#if authenticated}
  <div class="app-container">
    <!-- Show the following two components if the system is authenticated-->
    <Sidebar />
    <GraphComponentGraphlib />
  </div>
{/if}
