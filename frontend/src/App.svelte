<script lang="ts">
  import Sidebar from "./components/Sidebar.svelte";
  import GraphComponentGraphlib from "./components/GraphComponent_graphlib.svelte";
  import "../public/global.css";
  import { setupWebsocketConnection } from "helper_functions/websocket";
  import { onMount } from "svelte";
  import systemStateStore from "stores/systemStateStore";
  import { websocketStore } from "stores/websocketStore";
  import AuthPage from "./components/AuthPage.svelte";
  import { SystemState } from "generated/system_types_pb";
  import Loading from "./components/Loading.svelte";

  let authenticated = false;
  let websocket: WebSocket;
  let system_state: SystemState;
  let websocket_ready = false;

  onMount(async () => {
    // subscribe to system state:
    system_state = $systemStateStore;

    if (!$systemStateStore.getWebsocketReady()) {
      // startup websocket connection
      [websocket, system_state] = await setupWebsocketConnection(system_state);
      console.log("websocket: ", websocket);
      websocketStore.set({ websocket: websocket });
    }
  });

  $: {
    console.log(
      "System State Changed (App.svelte): " +
        JSON.stringify($systemStateStore.toObject())
    );
    authenticated = $systemStateStore.getAuthenticated();
    if ($systemStateStore.getWebsocketReady()) {
      websocket_ready = true;
      console.log("Websocket Ready to send Messages!");
    }
  }
</script>

<!-- Show the following component if the system is not authenticated-->
{#if !authenticated}
  {#if websocket_ready}
    <AuthPage />
  {:else}
    <Loading />
  {/if}
{/if}

{#if authenticated}
  <div class="app-container">
    <!-- Show the following two components if the system is authenticated-->
    <Sidebar />
    <GraphComponentGraphlib />
  </div>
{/if}
