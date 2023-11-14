<script lang="ts">
  import { onMount } from "svelte";

  import { v4 as uuidv4 } from "uuid";

  import {
    NodeTypes,
    Node,
    // MessageBundle,
    // VerbTypeNames,
    Process,
    GraphNodeInfo,
    NodesToProcess,
    Letter,
    Body,
    VerbTypes,
  } from "generated/system_types";

  import systemStateStore from "stores/systemStateStore";

  import { writable } from "svelte/store";
  import { websocketStore } from "stores/websocketStore";
  import { sendEnvelope } from "helper_functions/websocket";

  let name = "";
  let description = "";
  let node_list: Node[] = [];
  let selected_node_list: Node[] = [];
  let error = "";

  let key_list = Object.keys(NodeTypes).filter((key) => isNaN(Number(key)));

  // setup onmount:
  onMount(() => {
    node_list = $systemStateStore.local_nodes;
  });

  $: {
    node_list = $systemStateStore.local_nodes;
  }

  function isSelected(node: Node): boolean {
    return selected_node_list.includes(node);
  }

  function toggleNodeSelect(node: Node) {
    if (isSelected(node)) {
      selected_node_list = selected_node_list.filter(
        (selected_node) => selected_node !== node
      );
    } else {
      selected_node_list = [...selected_node_list, node];
    }
  }

  function sendNodes() {
    if (!name.trim() || !description.trim()) {
      error = "Both name and description are required!";
      return; // Return early to stop execution if validation fails
    } else {
      error = "";
    }
    alert("sendNodes feature is not yet implemented!");
    console.log("sending selected_node_list: ", selected_node_list);

    let graph_node_info = new GraphNodeInfo();

    graph_node_info.name = name;
    graph_node_info.description = description;
    graph_node_info.id = uuidv4();

    let nodes_to_process = new NodesToProcess();

    nodes_to_process.containing_node_info = graph_node_info;

    nodes_to_process.nodes = selected_node_list;

    let websocket = $websocketStore.websocket as WebSocket;

    let letter = new Letter();

    let body = new Body();

    body.nodes_to_process = nodes_to_process;

    letter.body = body;
    letter.verb = VerbTypes.Validate;

    sendEnvelope(websocket, [letter]);

    selected_node_list = [];
    description = "";
    name = "";
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
        >{key_list[node.node_type]} : {node.node_info.name}</button
      >
    </li>
  {/each}
</ul>

<h3>Nodes to add:</h3>

{#each selected_node_list as node}
  <li>
    <p>{key_list[node.node_type]} : {node.node_info.name}</p>
  </li>
{/each}

{#if error}
  <p class="error">{error}</p>
{/if}

<button class="add-button" on:click={sendNodes}>Save Process</button>

<style>
  .error {
    color: red;
  }
</style>
