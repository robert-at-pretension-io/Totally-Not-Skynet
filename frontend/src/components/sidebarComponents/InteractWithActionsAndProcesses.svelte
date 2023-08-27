<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import { Node } from "../../generated/system_types_pb";

  let selectedNode: Node | null;

  // Subscribe to the graphStore to get the latest values
  let nodes: Node[] = [];

  $: {
    nodes = $systemStateStore.getNodesList();
    selectedNode = $systemStateStore.getSelectedNodeList()[0];
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
      >{node.getNodeInfo()?.getName()} : {node.getNodeInfo()?.getName()}</option
    >
  {/each}
</select>
