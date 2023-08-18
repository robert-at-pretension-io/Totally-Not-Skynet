<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import {
    Node
  } from "generated/system_types_pb.js";

  let selectedNode: Node | null;

  // Subscribe to the graphStore to get the latest values
  let nodes: Node[] = [];

  $: {
    nodes = $systemStateStore.nodes;
    selectedNode = $systemStateStore.selected_node;
  }

  // Function to handle dropdown change events
  function onDropdownChange() {
    $systemStateStore.selected_node = selectedNode;
  }
</script>

<!-- Dropdown menu for actions -->
<select bind:value={selectedNode} on:change={() => onDropdownChange()}>
  <option value="">Select a node</option>
  {#each nodes as node}
    <option value={node}>{node.Node.type_name} : {node.Node.name}</option>
  {/each}
</select>
