<script lang="ts">
  import GraphComponent from "./components/GraphComponent.svelte";
  import Sidebar from "./components/Sidebar.svelte";

  import type {
    Graph,
    selectedGraphComponent,
    GraphState,
    Goal,
    InitializeProject,
    OpenaiKey,
    Action,
    Process,

    AiSystemState

  } from "./system_types";
  import { setGraphState } from "./helper_functions/graph";
import {onMount} from "svelte";
import websocketStore from "./stores/websocketStore";
let user_id = "";
import { aiSystemStore } from "stores/aiSystemStore";
import systemStateStore from "stores/systemStateStore";
import { processToGraph } from "helper_functions/graph";

onMount(async () => {
  // start the websocket connection 
  $websocketStore.addEventListener("open", () => {
    console.log("websocket connection opened");
    $websocketStore.send(JSON.stringify({initial_message: "initial message"}));
  });
  $websocketStore.addEventListener("message", (event) => {
    console.log("websocket message received: ", event.data);
    let data = JSON.parse(event.data);
    // check to see if the data has the shape of a Process or Action
    if (data.hasOwnProperty("description")) {
      let process: Process = data;
      aiSystemStore.update((state : AiSystemState) => {
        state.processes.push(process);
        return state;
      });
    } else if (data.hasOwnProperty("prompt")) {
      let action: Action = data;
      aiSystemStore.update((state : AiSystemState) => {
        state.actions.push(action);
        return state;
      });
    }
  });
});

async function handleProcessChange(process : Process) {
  console.log("selected process changed: ", process);
  await processToGraph(process);
}

$:  {
  let process = $systemStateStore.selectedProcess;
  console.log("process: ", process);
  if (process) {
    handleProcessChange(process);
  }
  
}

const graph: Graph = {
  nodes: [
    {
      id: "1",
      label: "Node 1",
      data: { someData: "value" },
    },
    {
      id: "2",
      label: "Node 2",
      data: { someOtherData: "value2" },
    },
    {
      id: "3",
      label: "Node 3",
      data: { yetAnotherData: "value3" },
    },
  ],
  edges: [
    {
      id: "a",
      source: "1",
      target: "2",
      label: "Edge 1",
    },
    {
      id: "b",
      source: "2",
      target: "3",
      label: "Edge 2",
    },
  ],
};

  const selectedComponent: selectedGraphComponent = {
    type: "Node",
    instance: graph.nodes[0],
    neighbors: graph.nodes,
    outgoing: graph.edges.filter((edge) => edge.source === graph.nodes[0].id),
    incoming: graph.edges.filter((edge) => edge.target === graph.nodes[0].id),
  };

  let graphState: GraphState = {
    graph: graph,
    selected: selectedComponent,
    lastAction: "none",
    actedOn: null,
  };

  setGraphState(graphState);
</script>

<Sidebar />
<GraphComponent />
