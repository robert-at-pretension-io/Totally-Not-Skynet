<script lang="ts">
  import { onMount } from "svelte";

  import {
    NodeTypeNames,
    Node,
    CrudBundle,
    ValidateNodes,
    VerbTypeNames,
    Process,
  } from "generated/system_types";

  // proto from "../../generated/system_types";

  import systemStateStore from "stores/systemStateStore";

  import { writable } from "svelte/store";
  import { websocketStore } from "stores/websocketStore";
  import { sendWebsocketMessage } from "helper_functions/websocket";

  let selected_node_ids: string[] = [];

  const selected_node_ids_store = writable(selected_node_ids);

  let name = "";
  let description = "";
  let node_list: Node[] = [];

  export let process = new Process();
  // let node_info_list : GraphNodeInfo[] = [];

  // let key_list = Object.keys(NodeTypeNames);
  let key_list = Object.keys(NodeTypeNames).filter((key) => isNaN(Number(key)));

  // setup onmount:
  onMount(() => {
    node_list = $systemStateStore.nodes;
  });

  $: {
    node_list = $systemStateStore.nodes;
  }

  function isSelected(node: Node): boolean {
    let node_id = node.node_info.id as string;

    return $selected_node_ids_store.includes(node_id);
  }

  function toggleNodeSelect(node: Node) {
    let node_id = node.node_info.id as string;

    if (isSelected(node)) {
      selected_node_ids_store.update((val) =>
        val.filter((item) => item !== node_id)
      );
    } else {
      selected_node_ids_store.update((val) => [...val, node_id]);
    }
  }

  function sendNodes() {
    let filtered_nodes = node_list.filter((node: Node) => {
      return $selected_node_ids_store.includes(node.node_info.id as string);
    });

    console.log("sending filtered_nodes: ", filtered_nodes);

    let crud_message = new CrudBundle();

    crud_message.verb = VerbTypeNames.Post;

    let validate_nodes = new ValidateNodes();

    validate_nodes.description = description;
    validate_nodes.name = name;

    validate_nodes.nodes = filtered_nodes;

    crud_message.validate_nodes = validate_nodes;

    sendWebsocketMessage(crud_message, $websocketStore.websocket as WebSocket);

    $selected_node_ids_store = [];
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
        >{key_list[node.type_name]} : {node.node_info.name}</button
      >
    </li>
  {/each}
</ul>

<h3>Nodes to add:</h3>

{#each $selected_node_ids_store as id}
  <p>{id}</p>
{/each}

<button class="add-button" on:click={sendNodes}>Save Process</button>

<!-- <InteractWithActionsAndProcesses /> -->
