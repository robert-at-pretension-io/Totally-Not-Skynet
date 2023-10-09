<script lang="ts">
    import systemStateStore from "stores/systemStateStore";
    import {
      GraphNodeInfo,
      Node,
      NodeTypeNames,
    } from "../../generated/system_types";
    import NodeInfoPanel from "./NodeInfoPanel.svelte";

    let ordered_nodes: Node[] = [];
    let topological_order: GraphNodeInfo[] = [];

    $: {
      if ($systemStateStore.selected_process.process) {
        // for each of the nodes in the selected_process graph, display the information about the node. Displayed in the order that they will be executed
        if (
          topological_order !==
                $systemStateStore.selected_process.process.topological_order
        ) {
          topological_order =
                    $systemStateStore.selected_process.process
                      .topological_order;
          ordered_nodes = topological_order.map((topo_node_info) =>
            $systemStateStore.nodes.find((node) => {
              node.node_info = topo_node_info;
            })
          );
        }
      }
    }
</script>

<!-- Using NodeInfoPanel to display each node's details -->
{#each ordered_nodes as node}
    <NodeInfoPanel {node} />
{/each}
