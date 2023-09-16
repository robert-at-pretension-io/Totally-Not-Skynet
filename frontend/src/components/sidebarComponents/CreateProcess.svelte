<script lang="ts">
  import { onMount } from "svelte";

  import * as proto from "../../generated/system_types_pb";

  import systemStateStore from "stores/systemStateStore";

  import { writable } from "svelte/store";

  let selected_node_ids: string[] = [];

  const selected_node_ids_store = writable(selected_node_ids);

  let name = "";
  let description = "";
  let node_list: proto.Node[] = [];
  // let node_info_list : proto.GraphNodeInfo[] = [];

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

  function isSelected(node: proto.Node): boolean {
    let node_id = node.getNodeInfo()?.getId() as string;

    return $selected_node_ids_store.includes(node_id);
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

  function sendNodes() {
    let filtered_nodes = node_list.filter((node: proto.Node) => {
      return $selected_node_ids_store.includes(
        node.getNodeInfo()?.getId() as string
      );
    });

    alert(
      "Send the filtered nodes to the backend with the ValidateNodes message"
    );
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

<button class="add-button" on:click={sendNodes}>Save Process</button>

<!-- <InteractWithActionsAndProcesses /> -->
