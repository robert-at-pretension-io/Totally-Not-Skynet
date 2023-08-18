<script lang="ts">
  import { onMount, setContext } from "svelte";
  import cytoscape, { Core } from "cytoscape";
  import dagre from "cytoscape-dagre";
  import GraphStyles from "./GraphStyles.js";
  import systemStateStore from "stores/systemStateStore.js";
  import { selectNode, selectEdge } from "../helper_functions/graph";

  import {
    SystemState,
    Edge,
    GraphNodeInfo,
  } from "generated/system_types_pb.js";
  import * as graphlib from "graphlib";

  setContext("graphSharedState", {
    getCyInstance: () => cyInstance,
  });

  let refElement: HTMLElement | null = null;
  let cyInstance: Core | null = null;
  let g = new graphlib.Graph();

  systemStateStore.subscribe((new_value: SystemState) => {
    let value = new_value.graph_state;

    if (
      value !== null &&
      value.last_action === "addNode" &&
      value.acted_on != null
    ) {
      // check if the node is already in the graph
      let node = value.acted_on as GraphNodeInfo;
      if (g.hasNode(node.name)) {
        return;
      }
      g.setNode(node.id, node.name);
      if (cyInstance) {
        cyInstance.add({
          data: { id: node.id, label: node.name },
        });

        cyInstance
          .layout({
            name: "dagre",
          })
          .run();
      }
    } else if (
      value !== null &&
      value.last_action === "addEdge" &&
      value.acted_on != null
    ) {
      let edge = value.acted_on as Edge;
      // check if the edge is already in the graph
      if (g.hasEdge(edge.source, edge.target)) {
        return;
      }

      g.setEdge(edge.source, edge.target);
      if (cyInstance) {
        cyInstance.add({
          data: { source: edge.source, target: edge.target },
        });
      }
    } else if (
      value !== null &&
      value.last_action === "removeEdge" &&
      value.acted_on != null
    ) {
      let edge = value.acted_on as Edge;

      g.removeEdge(edge.source, edge.target);
      if (cyInstance) {
        cyInstance.remove(
          cyInstance.$id(edge.source).edgesTo(cyInstance.$id(edge.target))
        );
      }
    } else if (
      value != null &&
      value.last_action === "removeNode" &&
      value.acted_on != null
    ) {
      let node = value.acted_on as GraphNodeInfo;
      g.removeNode(node.name);
      if (cyInstance) {
        cyInstance.remove(cyInstance.$id(node.id));
      }
    } else if (value != null && value.last_action === "resetGraph") {
      g = new graphlib.Graph(); // reset graph
      if (cyInstance) {
        cyInstance.elements().remove();
      }
    }
  });

  onMount(async () => {
    cytoscape.use(dagre);

    cyInstance = cytoscape({
      container: refElement,
      style: GraphStyles,
    });

    cyInstance.on("select", "node", (evt) => {
      const selectedNode = evt.target.data();
      selectNode(selectedNode.id);
    });

    cyInstance.on("select", "edge", (event) => {
      selectEdge(event.target.data().source, event.target.data().target);
    });
  });
</script>

<div class="graph" bind:this={refElement}>
  {#if cyInstance}
    <slot />
  {/if}
</div>
