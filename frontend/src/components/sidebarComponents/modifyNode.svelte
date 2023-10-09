<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import {
    GraphNodeInfo,
    Node,
    NodeTypeNames,
  } from "../../generated/system_types";

  let selectedNode: Node | null;

  // Subscribe to the graphStore to get the latest values
  let nodes: Node[] = [];

  let typeName: NodeTypeNames = NodeTypeNames.PROMPT;

  let key_list = Object.keys(NodeTypeNames).filter((key) => isNaN(Number(key)));

  console.log("keylist is:", key_list);

  let num_array = Array.from({ length: key_list.length }, (_, i) => i);

  let node_options = new Array<Node>();

  $: {
    nodes = $systemStateStore.nodes;
    // set the node_options to the nodes with the nodetype of typeName
    node_options = nodes.filter((node) => node.type_name === typeName);
  }

  // Function to handle dropdown change events
  function onDropdownChange() {
    // match on the cases of selectedNode.type_name:
    switch (selectedNode.type_name) {
    case NodeTypeNames.PROMPT:
      console.log("Selected node is a prompt");
      break;
    case NodeTypeNames.PROCESS:
      $systemStateStore.selected_process = selectedNode;
      console.log("Selected node is a process");
      break;
    case NodeTypeNames.CONDITIONAL:
      console.log("Selected node is a conditional");
      break;
    case NodeTypeNames.COMMAND:
      console.log("Selected node is a command");
      break;
    default:
      console.log("Selected node is not a valid node type");
    }
  }
</script>

<p>Select a node type to modify:</p>
<select bind:value={typeName}>
  {#each num_array as array_index}
    <option value={array_index}>{key_list[array_index]}</option>
  {/each}
</select>

<p>Selected type: {key_list[typeName]}</p>

<!-- Dropdown menu for actions -->
<select bind:value={selectedNode} on:change={() => onDropdownChange()}>
  <option value="">Select a node</option>
  {#each node_options as node}
    <option value={node}
      >{key_list[node.type_name]} : {node.node_info.name}</option
    >
  {/each}
</select>
