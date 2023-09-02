<script lang="ts">
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

  let node_id_list: string[] = [];

  let selected_edge: proto.Edge | null = null;

  let graph = new proto.Graph();
  let name = "";
  let description = "";
  let node_list: proto.Node[] = [];

  let key_list = Object.keys(proto.NodeTypeNames);

  $: {
    // Whenever the system state changes, update the nodes_available
    node_list = $systemStateStore.getNodesList();

    node_id_list = $selected_node_ids_store;

    console.log("The node_id_list is now: " + node_id_list);
  }

  async function saveProcess() {
    // create an alert message if either name or description are null
    if (name === null || description === null) {
      alert("Please enter a name and description for the process");
      return;
    } else {
      const systemState = await helper_functions.getSystemState();
      let graph_state = systemState.getGraphState();
      let maybe_topological_order = await helper_functions.validateGraph(
        systemState
      );

      if (maybe_topological_order && graph_state != undefined) {
        let topological_order =
          maybe_topological_order as proto.GraphNodeInfo[];

        // console.log("current_graph_string: " + current_graph_string);
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
    // check to see if selected_nodes : Node[] contains node : Node

    console.log(
      " Checking to see if node " +
        node.getNodeInfo()?.getId() +
        " is contained in " +
        $selected_node_ids_store
    );

    let node_id = node.getNodeInfo()?.getId() as string;

    return $selected_node_ids_store.includes(node_id);
  }
  function removeNodes() {
    let current = graph.getNodesList();

    let new_nodes = current.filter((node: proto.GraphNodeInfo) => {
      return !selected_node_ids.includes(node.getId());
    });

    graph.setNodesList(new_nodes);

    selected_node_ids = [];
  }
  // function addNodes() {
  //   let current_nodes = graph.getNodesList();
  //   selected_node_ids.forEach((node_id: string) => {
  //     // check if current_nodes already contains node
  //     if (!current_nodes.includes(node)) {
  //       current_nodes.push(node);
  //     }
  //   });
  //   graph.setNodesList(current_nodes);
  //   selected_nodes = [];
  // }
  function addEdge() {
    let current_edges = graph.getEdgesList();
    // add selected_edge : Edge to current_edges : Edge[]
    if (selected_edge != null) {
      current_edges.push(selected_edge);
    }
    graph.setEdgesList(current_edges);
  }
  function removeEdge() {
    let current_edges = graph.getEdgesList();
    // remove selected_edge : Edge from current_edges : Edge[]
    if (selected_edge != null) {
      current_edges = current_edges.filter((edge: proto.Edge) => {
        return edge != selected_edge;
      });
    }
    graph.setEdgesList(current_edges);
  }

  function toggleNodeSelect(node: proto.Node) {
    let node_id = node.getNodeInfo()?.getId() as string;

    if (isSelected(node)) {
      selected_node_ids_store.update((val) =>
        val.filter((item) => item !== node_id)
      );
      console.log("removing node: " + node_id);
    } else {
      selected_node_ids_store.update((val) => [...val, node_id]);
      console.log("adding node");
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

<!-- <button class="add-button" on:click={addNodes}>Add Node(s)</button> -->
<button class="remove-button" on:click={removeNodes}>Remove Node(s)</button>
<button class="add-button" on:click={addEdge}>Add Edge</button>
<button class="remove-button" on:click={removeEdge}>Remove Edge</button>
<button class="add-button" on:click={saveProcess}>Save Process</button>

<!-- <InteractWithActionsAndProcesses /> -->
