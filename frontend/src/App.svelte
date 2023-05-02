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
    Process
  } from "./system_types";
  import { setGraphState } from "./helper_functions/graph";
import {onMount} from "svelte";
import websocketStore from "./stores/websocketStore";
let user_id = "";

onMount(async () => {
  // start the websocket connection 
  $websocketStore.addEventListener("open", () => {
    console.log("websocket connection opened");
    $websocketStore.send(JSON.stringify({initial_message: "initial message"}));
  });
  $websocketStore.addEventListener("message", (event) => {
    console.log("websocket message received");
    let data = JSON.parse(event.data);
    // check to see if the data has the shape of a Process or Action
    if (data.hasOwnProperty("description")) {
      let process: Process = data;
      console.log(process);
    } else if (data.hasOwnProperty("prompt")) {
      let action: Action = data;
      console.log(action);
    }
  });
});
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
