<script lang="ts">
  import Sidebar from "./components/Sidebar.svelte";
  import GraphComponentGraphlib from "./components/GraphComponent_graphlib.svelte";
  import { setupWebsocketConnection } from "helper_functions/websocket";
  import { onMount } from "svelte";
  import systemStateStore from "stores/systemStateStore";
  import { websocketStore } from "stores/websocketStore";
  import AuthPage from "./components/AuthPage.svelte";
  import { SystemState } from "./generated/system_types_pb";
  import Loading from "./components/Loading.svelte";
  // import { initializeSystemState } from "helper_functions/misc";

  console.log("Script started");

  let authenticated = false;
  let websocket: WebSocket;
  let system_state: SystemState;
  let websocket_ready = false;

  onMount(async () => {
    console.log("onMount triggered");

    system_state = $systemStateStore;
    console.log("Initial system_state:", system_state);

    // let intialized_system = initializeSystemState(system_state);
    // console.log("Initialized system:", intialized_system);

    // systemStateStore.set(intialized_system);

    if (!system_state.getWebsocketReady()) {
      console.log("Websocket not ready. Initializing...");
      [websocket, system_state] = await setupWebsocketConnection(system_state);
      console.log("Websocket initialized:", websocket);
      websocketStore.set({ websocket });
      websocket_ready = true;
      $systemStateStore = system_state;
    }
  });

  $: {
    console.log("Reactive statement triggered");
    authenticated = system_state?.getAuthenticated();
    console.log("Authenticated state:", authenticated);

    if (system_state?.getWebsocketReady()) {
      console.log("Websocket ready");
      websocket_ready = true;
    }
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
