<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import { GraphNodeInfo, Node, NodeTypes } from "../../generated/system_types";

  let selectedNode: Node | null;

  // Subscribe to the graphStore to get the latest values
  let nodes: Node[] = [];

  let typeName: NodeTypes = NodeTypes.PROMPT;

  let key_list = Object.keys(NodeTypes).filter((key) => isNaN(Number(key)));

  console.log("keylist is:", key_list);

  let num_array = Array.from({ length: key_list.length }, (_, i) => i);

  let node_options = new Array<Node>();

  $: {
    nodes = $systemStateStore.local_nodes;
    // set the node_options to the nodes with the nodetype of typeName
    node_options = nodes.filter((node) => node.node_type === typeName);
  }

  // Function to handle dropdown change events
  function onDropdownChange() {
    // match on the cases of selectedNode.type_name:
    switch (selectedNode.node_type) {
    case NodeTypes.PROMPT:
      console.log("Selected node is a prompt");
      break;
    case NodeTypes.PROCESS:
      $systemStateStore.selected_process = selectedNode;
      console.log("Selected node is a process");
      break;
    case NodeTypes.CONDITIONAL:
      console.log("Selected node is a conditional");
      break;
    case NodeTypes.COMMAND:
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
      >{key_list[node.node_type]} : {node.node_info.name}</option
    >
  {/each}
</select>
