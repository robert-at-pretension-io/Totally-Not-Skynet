<script lang="ts">
  import { onMount } from "svelte";
  // import { onMount } from "svelte";
  // import {
  //   GraphNodeInfo,
  //   Graph,
  //   Node,
  //   Edge,
  //   Process,
  // } from  "@generated/system_types_pb.js";

  import * as proto from "../../generated/system_types_pb";

  import * as helper_functions from "../../helper_functions/graph";

  import systemStateStore from "stores/systemStateStore";

  let selected_node_ids: string[] = [];

  import { writable } from "svelte/store";

  const selected_node_ids_store = writable(selected_node_ids);

  let name = "";
  let description = "";
  let node_list: proto.Node[] = [];

  let key_list = Object.keys(proto.NodeTypeNames);

  let system_state: proto.SystemState;

  // setup onmount:
  onMount(() => {
    system_state = $systemStateStore;
    node_list = $systemStateStore.getNodesList();
  });

  $: {
    system_state = $systemStateStore;

    node_list = $systemStateStore.getNodesList();
  }

  function saveProcess() {
    // create an alert message if either name or description are null
    if (name === null || description === null) {
      alert("Please enter a name and description for the process");
      return;
    } else {
      let graph_state = system_state.getGraphState();
      let maybe_topological_order =
        helper_functions.validateGraph(system_state);

      if (maybe_topological_order && graph_state != undefined) {
        let topological_order =
          maybe_topological_order as proto.GraphNodeInfo[];

        let process = new proto.Process();

        process.setGraphState(graph_state);
        process.setInitialVariablesList([]);
        process.setTopologicalOrderList(topological_order);

        let new_node = new proto.Node();

        let graph_node_info = new proto.GraphNodeInfo();
        graph_node_info.setName(name);

        new_node.setNodeInfo(graph_node_info);
        new_node.setDescription(description);
        new_node.setProcess(process);

        alert("todo: save process by sending websocket message");
      } else {
        alert("The process does not have a valid topological order :(");
      }
    }
  }
  function isSelected(node: proto.Node): boolean {
    let node_id = node.getNodeInfo()?.getId() as string;

    return $selected_node_ids_store.includes(node_id);
  }
  function removeNodes() {
    let graph_state = system_state.getGraphState();
    let action_list = graph_state?.getActionHistoryList();

    // check if the last action was a select edge action

    let last_action: proto.GraphAction;
    let last_action_type: proto.GraphAction.ActionMap[keyof proto.GraphAction.ActionMap];

    if (action_list != undefined && action_list.length > 0) {
      last_action = action_list[action_list.length - 1];

      last_action_type = last_action.getAction();

      console.log("remove node action:", last_action.toObject());

      if (last_action_type == proto.GraphAction.Action.SELECT) {
        if (last_action.hasNode()) {
          let last_node = last_action.getNode();
          if (last_node) {
            $systemStateStore = helper_functions.removeNode(
              last_node.getId() as string,
              system_state
            );
          }
        }
      }
    }
  }

  function addNodes() {
    console.log(
      "The currently selected node ids are:",
      $selected_node_ids_store
    );

    let filtered_nodes = node_list.filter((node: proto.Node) => {
      return $selected_node_ids_store.includes(
        node.getNodeInfo()?.getId() as string
      );
    });

    for (const node of filtered_nodes) {
      try {
        $systemStateStore = helper_functions.addNode(node, $systemStateStore);
      } catch (error) {
        console.error("Error in addNode:", error);
      }
    }

    selected_node_ids_store.set([]);
  }

  function addEdge() {
    let graph_state = system_state.getGraphState();
    let action_list = graph_state?.getActionHistoryList();

    // check if the last action was a select edge action

    let last_action: proto.GraphAction;
    let two_actions_ago: proto.GraphAction;
    let last_action_type: proto.GraphAction.ActionMap[keyof proto.GraphAction.ActionMap];
    let two_actions_ago_type: proto.GraphAction.ActionMap[keyof proto.GraphAction.ActionMap];

    if (action_list != undefined && action_list.length > 1) {
      last_action = action_list[action_list.length - 1];
      two_actions_ago = action_list[action_list.length - 2];
      console.log("Last action:", last_action.toObject());

      last_action_type = last_action.getAction();
      two_actions_ago_type = two_actions_ago.getAction();
      console.log("Action type:", last_action_type);

      if (
        last_action_type == proto.GraphAction.Action.SELECT &&
        two_actions_ago_type == proto.GraphAction.Action.SELECT
      ) {
        if (last_action.hasNode() && two_actions_ago.hasNode()) {
          let last_node = last_action.getNode();
          let two_actions_ago_node = two_actions_ago.getNode();
          if (last_node && two_actions_ago_node) {
            let add_edge = new proto.Edge();
            add_edge.setSource(two_actions_ago_node);
            add_edge.setTarget(last_node);

            $systemStateStore = helper_functions.addEdge(
              add_edge,
              system_state
            );
          }
        }
      }
    }

    // add selected_edge : Edge to current_edges : Edge[]
  }
  function removeEdge() {
    console.log("Attempting to remove the edge:");
    let graph_state = system_state.getGraphState();
    let action_list = graph_state?.getActionHistoryList();

    // check if the last action was a select edge action

    let last_action: proto.GraphAction;
    let last_action_type: proto.GraphAction.ActionMap[keyof proto.GraphAction.ActionMap];

    if (action_list != undefined && action_list.length > 0) {
      last_action = action_list[action_list.length - 1];
      console.log("Last action:", last_action.toObject());

      last_action_type = last_action.getAction();
      console.log("Action type:", last_action_type);

      if (last_action_type == proto.GraphAction.Action.SELECT) {
        if (last_action.hasEdge()) {
          let selected_edge = last_action.getEdge();
          if (selected_edge) {
            $systemStateStore = helper_functions.removeEdge(
              selected_edge,
              system_state
            );
          }
        }
      }
    }
  }

  function toggleNodeSelect(node: proto.Node) {
    let node_id = node.getNodeInfo()?.getId() as string;

    if (isSelected(node)) {
      selected_node_ids_store.update((val) =>
        val.filter((item) => item !== node_id)
      );
    } else {
      selected_node_ids_store.update((val) => [...val, node_id]);
    }
  }
</script>

<p>Please set a descriptive name for your process:</p>
<input type="text" bind:value={name} />
<p>
  Please set a description for your process, please talk about what purpose it
  serves:
</p>
<input type="text" bind:value={description} />

<p>
  Click the node buttons below to add them to the graph. Then click "Add Node(s)
  to see them populate on the graph."
</p>

<ul>
  {#each node_list as node}
    <li>
      <button
        class:selected={isSelected(node)}
        type="button"
        on:click={() => toggleNodeSelect(node)}
        >{key_list[node.getTypeName()]} : {node
          .getNodeInfo()
          ?.getName()}</button
      >
    </li>
  {/each}
</ul>

<h3>Nodes to add:</h3>

{#each $selected_node_ids_store as id}
  <p>{id}</p>
{/each}

<button class="add-button" on:click={addNodes}>Add Node(s)</button>
<button class="remove-button" on:click={removeNodes}>Remove Node(s)</button>
<button class="add-button" on:click={addEdge}>Add Edge</button>
<button class="remove-button" on:click={removeEdge}>Remove Edge</button>
<button class="add-button" on:click={saveProcess}>Save Process</button>

<!-- <InteractWithActionsAndProcesses /> -->
