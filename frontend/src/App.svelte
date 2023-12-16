<script lang="ts">
  import Sidebar from "./components/Sidebar.svelte";
  import GraphComponentGraphlib from "./components/GraphComponent_graphlib.svelte";
  import { setupWebsocketConnection } from "helper_functions/websocket";
  import { onMount } from "svelte";
  import systemStateStore from "stores/systemStateStore";
  import { websocketStore } from "stores/websocketStore";
  import AuthPage from "./components/AuthPage.svelte";
  import { SystemState } from "./generated/system_types";
  import Loading from "./components/Loading.svelte";
  import { initializeSystemState } from "helper_functions/misc";

  console.log("Script started");
  let websocket: WebSocket;
  let system_state: SystemState;
  let websocket_ready = false;
  let authenticated: boolean;

  onMount(() => {
    console.log("onMount triggered");

    $systemStateStore = initializeSystemState($systemStateStore);

    system_state = $systemStateStore;
    console.log("Initial system_state:", system_state);

    if (!websocket_ready) {
      console.log("Websocket not ready. Initializing...");
      websocket = setupWebsocketConnection();
      console.log("Websocket initialized:", websocket);
      websocketStore.set({ websocket });
      websocket_ready = true;
      $systemStateStore = system_state;
    }
  });

  $: {
    console.log("auth state", $systemStateStore.authenticated);
    authenticated = $systemStateStore.authenticated;
  }
</script>

{#if !authenticated}
  {#if websocket_ready}
    <AuthPage />
  {:else}
    <Loading />
  {/if}
{/if}

{#if authenticated}
  <div class="app-container">
    <Sidebar />
    <GraphComponentGraphlib />
  </div>
{/if}

<style>
  .app-container {
    display: grid;
    grid-template-columns: 25vw 1fr;
  }
</style>
