<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import {
    GraphNodeInfo,
    Node,
    NodeTypeNames,
  } from "../../generated/system_types";
  import { onMount } from "svelte";

  let ordered_nodes: Node[] = [];
  let nodes: Node[] = [];
  let topological_order: GraphNodeInfo[] = [];

  let selected_process: Node | undefined = undefined;

  let key_list = Object.keys(NodeTypeNames).filter((key) => isNaN(Number(key)));

  onMount(() => {
    console.log("ExecuteNode mounted");
    if ($systemStateStore.selected_process !== undefined) {
      topological_order =
        $systemStateStore.selected_process.process.topological_order;
      selected_process = $systemStateStore.selected_process;
    }
    nodes = $systemStateStore.nodes;
  });

  $: {
    if ($systemStateStore.selected_process !== undefined) {
      topological_order =
        $systemStateStore.selected_process.process.topological_order;

      selected_process = $systemStateStore.selected_process;

      // get the nodes from the systemStateStore
      nodes = $systemStateStore.nodes;

      // sort the nodes by their topological order
      // the topological order is the order in which the nodes should be executed, it stores the ids contained in the nodes array
      ordered_nodes = topological_order.map((node) => {
        return nodes.find((n) => n.node_info.id === node.id);
      });
    }
  }
</script>

{#each ordered_nodes as node (node.node_info.id)}
  <h2><strong>{key_list[node.type_name]}</strong> : {node.node_info.name}</h2>
  {node.node_info.description}
  <br />
  <p><strong>Input Variables:</strong> {node.input_variables.join(", ")}</p>
  <p><strong>Output Variables:</strong> {node.output_variables.join(", ")}</p>
  <p><strong>Node Content:</strong> {node.node_content}</p>

  <hr />
{/each}
