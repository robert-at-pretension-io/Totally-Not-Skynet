<script lang="ts">
  import GraphComponent from "./components/GraphComponent.svelte";
  import Sidebar from "./components/Sidebar.svelte";

  import type {
    Graph,
    selectedGraphComponent,
    GraphState,
  } from "./system_types";
  import { setGraphState } from "./helper_functions/graph";
import {onMount} from "svelte";

let user_id = "";

onMount(async () => {
  // start the websocket connection 
  let ws = new WebSocket('ws://127.0.0.1:8080');

  ws.send('initializing_actions');

  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let result = '';

  for (let i = 0; i < length; i++) {
    const randomIndex = Math.floor(Math.random() * characters.length);
    result += characters.charAt(randomIndex);
  }

  user_id = result;

  ws.addEventListener('message', (event) => {
    console.log("Received message: ", event.data);
  })
})
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
