<script lang="ts">
  import { onMount, setContext } from "svelte";
  import cytoscape, { Core } from "cytoscape";
  import dagre from "cytoscape-dagre";
  import GraphStyles from "./GraphStyles.js";
  import systemStateStore from "stores/systemStateStore.js";
  import { selectNode, selectEdge } from "../helper_functions/graph";
  import graphlib from "graphlib";
  import type { SystemState, Edge, GraphNodeInfo } from "system_types/index.js";

  setContext("graphSharedState", {
    getCyInstance: () => cyInstance,
  });

  let refElement: HTMLElement | null = null;
  let cyInstance: Core | null = null;
  let g = new graphlib.Graph();

  systemStateStore.subscribe((new_value: SystemState) => {
    let value = new_value.graphState;

    if (
      value !== null &&
      value.lastAction === "addNode" &&
      value.actedOn != null
    ) {
      // check if the node is already in the graph
      let node = value.actedOn as GraphNodeInfo;
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
      value.lastAction === "addEdge" &&
      value.actedOn != null
    ) {
      let edge = value.actedOn as Edge;
      // check if the edge is already in the graph
      if (g.hasEdge(edge.v, edge.w)) {
        return;
      }

      g.setEdge(edge.v, edge.w, value.actedOn);
      if (cyInstance) {
        cyInstance.add({
          data: { source: edge.v, target: edge.w },
        });
      }
    } else if (
      value !== null &&
      value.lastAction === "removeEdge" &&
      value.actedOn != null
    ) {
      let edge = value.actedOn as Edge;

      g.removeEdge(edge.v, edge.w);
      if (cyInstance) {
        cyInstance.remove(
          cyInstance.$id(edge.v).edgesTo(cyInstance.$id(edge.w))
        );
      }
    } else if (
      value != null &&
      value.lastAction === "removeNode" &&
      value.actedOn != null
    ) {
      let node = value.actedOn as GraphNodeInfo;
      g.removeNode(node.name);
      if (cyInstance) {
        cyInstance.remove(cyInstance.$id(node.id));
      }
    } else if (value != null && value.lastAction === "resetGraph") {
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
