<script lang="ts">
  import GraphComponent from "./components/GraphComponent.svelte";
  import Sidebar from "./components/Sidebar.svelte";

  import type {
    Graph,
    selectedGraphComponent,
    GraphState,
    Action,
    Process,

    AiSystemState

  } from "./system_types";
  import { setGraphState } from "./helper_functions/graph";
import {onMount} from "svelte";
import websocketStore from "./stores/websocketStore";
import { aiSystemStore } from "stores/aiSystemStore";
import systemStateStore from "stores/systemStateStore";
import { processToGraph } from "helper_functions/graph";
import {populateVariables} from "helper_functions/validation";

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
    if (Object.prototype.hasOwnProperty.call(data, "description")) {
      let process: Process = data;
      aiSystemStore.update((state : AiSystemState) => {
        state.processes.push(process);
        return state;
      });
    } else if (Object.prototype.hasOwnProperty.call(data, "prompt")) {
      let action: Action = data;
      aiSystemStore.update((state : AiSystemState) => {
        let variables = populateVariables(action);
        console.log("variables: ", variables);
        // check to see that the variables stored in the action are valid
        let compareThese = action.variables;
        console.log("compareThese: ", compareThese);

        let set1 = new Set(variables);
        console.log("set1: ", set1);
        let set2 = new Set(compareThese);
        console.log("set2: ", set2);
        let union = new Set([...set1, ...set2]);
        console.log("union: ", union);

        if ( union.size !== set1.size || union.size !== set2.size) {
          console.log("invalid variables");
          action.variables = variables;
          $websocketStore.send(JSON.stringify({"action": action}));
          return state;
        }

        state.actions.push(action);
        return state;
      });
    }
    else if (Object.prototype.hasOwnProperty.call(data, "create_action")) {
      let action : Action = data.create_action;
      aiSystemStore.update((state : AiSystemState) => {
        state.actions.push(action);
        return state;
      });
    }
    else if (Object.prototype.hasOwnProperty.call(data, "create_process")){
      let process : Process = data.create_process;
      aiSystemStore.update((state : AiSystemState) => {
        state.processes.push(process);
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
