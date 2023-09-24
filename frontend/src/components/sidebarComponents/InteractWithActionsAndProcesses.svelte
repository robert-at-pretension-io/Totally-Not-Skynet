<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import {
    GraphNodeInfo,
    Node,
    NodeTypeNames,
  } from "../../generated/system_types";

  let selectedNode: GraphNodeInfo | null;

  // Subscribe to the graphStore to get the latest values
  let nodes: Node[] = [];

  let key_list = Object.keys(NodeTypeNames);

  $: {
    nodes = $systemStateStore.nodes;
    selectedNode = $systemStateStore.selected_node[0];
  }

  // Function to handle dropdown change events
  function onDropdownChange() {
    alert("should implement onDropdownChange");
  }
</script>

<!-- Dropdown menu for actions -->
<select bind:value={selectedNode} on:change={() => onDropdownChange()}>
  <option value="">Select a node</option>
  {#each nodes as node}
    <option value={node}
      >{key_list[node.type_name]} : {node.node_info.name}</option
    >
  {/each}
</select>
