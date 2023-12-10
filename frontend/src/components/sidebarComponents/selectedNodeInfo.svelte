<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import { GraphNodeInfo, Node, NodeTypes } from "../../generated/system_types";

  let selected_nodes: Node[];

  $: {
    let nodes = $systemStateStore.local_nodes;

    let selected_node_info = $systemStateStore.selected_nodes;

    console.log("selected_node_info", selected_node_info);

    // Assuming 'node.node_info' has an 'id' property to uniquely identify the node
    // Filter nodes to only include those whose 'id' is found in the 'selected_node_info' array
    selected_nodes = nodes.filter((node) =>
      selected_node_info.some(
        (node_info) => node_info.id === node.node_info.id,
      ),
    );

    console.log("selected_nodes", selected_nodes);
  }
</script>

{#each selected_nodes as node}
  {node}
{/each}
