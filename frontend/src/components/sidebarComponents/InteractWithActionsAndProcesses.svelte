<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import { Option, none } from "fp-ts/Option";
  import type { Node } from "system_types";

  let selectedNode: Option<Node> = none;

  // Subscribe to the graphStore to get the latest values
  let nodes: Node[] = [];

  $: {
    nodes = $systemStateStore.nodes;
    selectedNode = $systemStateStore.selectedNode;
  }

  // Function to handle dropdown change events
  function onDropdownChange() {
    $systemStateStore.selectedNode = selectedNode;

    console.log("selectedNode: ", selectedNode);
  }
</script>

<!-- Dropdown menu for actions -->
<select bind:value={selectedNode} on:change={() => onDropdownChange()}>
  <option value="">Select a node</option>
  {#each nodes as node}
    <option value={node}>{node.name}</option>
  {/each}
</select>
