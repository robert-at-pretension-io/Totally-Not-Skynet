<script lang="ts">
  import GraphComponent from "./components/GraphComponent.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import RightSidebar from "./components/RightSidebar.svelte";

  import type {
    Graph,
    selectedGraphComponent,
    GraphState,
    Action,
    Process,

    AiSystemState

  } from "./system_types";

  import { isAction, isProcess } from "helper_functions/type_checker";

  import { setGraphState } from "./helper_functions/graph";
import {onMount} from "svelte";
import websocketStore from "./stores/websocketStore";
import { aiSystemStore } from "stores/aiSystemStore";
import systemStateStore from "stores/systemStateStore";
import { processToGraph } from "helper_functions/graph";
import {populateInputVariables, populateOutputVariables} from "helper_functions/validation";

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
    if (isProcess(data)) {
      let process: Process = data;
      aiSystemStore.update((state : AiSystemState) => {
        console.log("Adding process to state:");
        state.processes.push(process);
        return state;
      });
    } else if (isAction(data)) {
      let action: Action = data;
      aiSystemStore.update((state : AiSystemState) => {
        console.log("Adding action to state:");
        let input_variables = populateInputVariables(action);
        console.log("input_variables: ", input_variables);
        let output_variables = populateOutputVariables(action);
        console.log("output_variables: ", output_variables);
        // check to see that the variables stored in the action are valid
        let compareThese = action.input_variables;
        let compareThese2 = action.output_variables;

        let set1 = new Set(input_variables);
        let set2 = new Set(compareThese);
        let union = new Set([...set1, ...set2]);

        let set3 = new Set(output_variables);
        let set4 = new Set(compareThese2);
        let union2 = new Set([...set3, ...set4]);

        let invalid = false;

        // This ensures that the input variables are always up-to-date
        if ( union.size !== set1.size || union.size !== set2.size) {
          console.log("invalid input variables");
          action.input_variables = input_variables;
          invalid = true;
        }
        // This ensures that the output variables are always up-to-date
        if ( union2.size !== set3.size || union2.size !== set4.size) {
          console.log("invalid output variables");
          action.output_variables = output_variables;
          invalid = true;
        }

        if (invalid) {
          $websocketStore.send(JSON.stringify({"action": action}));
          return state;
        }
        // check if the action is already in the state by looking at the name
        let actionIndex = state.actions.findIndex((a) => a.name === action.name);
        if (actionIndex === -1) {
          state.actions.push(action);
        } else {
          state.actions[actionIndex] = action;
        }
        
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

  handleProcessChange(process);
  
}

const graph: Graph = {
  nodes: [
  ],
  edges: [

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
    root_node_id: "",
  };

  setGraphState(graphState);
</script>

<Sidebar />
<GraphComponent />
<RightSidebar />
