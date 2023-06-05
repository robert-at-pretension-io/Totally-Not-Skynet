<script lang="ts">
  import { onMount, setContext } from "svelte";
  import cytoscape, { Core } from "cytoscape";
  import dagre from "cytoscape-dagre";
  import GraphStyles from "./GraphStyles.js";
  import systemStateStore from "stores/systemStateStore.js";
  import {
    selectNode,
    selectEdge
  } from "../helper_functions/graph";
  import graphlib from "graphlib";
  import type { SystemState } from "system_types/index.js";

  setContext("graphSharedState", {
    getCyInstance: () => cyInstance,
  });

  let refElement: HTMLElement | null = null;
  let cyInstance: Core | null = null;
  let g = new graphlib.Graph();
  let id_map = new Map();

  systemStateStore.subscribe((new_value : SystemState) => {
    let value = new_value.graphState;


    if (
      value.lastAction === "addNode" &&
      value.actedOn != null &&
      Array.isArray(value.actedOn)
    ) {
      // check if the node is already in the graph
      if (g.hasNode(value.actedOn[0])) {
        return;
      }
      id_map = id_map.set(value.actedOn[0], value.actedOn[1]);
      g.setNode(value.actedOn[0], value.actedOn[1]);
      if(cyInstance) {
        cyInstance.add({  data: { id: value.actedOn[0], label: value.actedOn[1] } });

        cyInstance
          .layout({
            name: "dagre",
          })
          .run();
        
      }
    } else if (
      value.lastAction === "addEdge" &&
      value.actedOn != null &&
      !Array.isArray(value.actedOn)
    ) {
      // check if the edge is already in the graph
      if (g.hasEdge(value.actedOn.v, value.actedOn.w)) {
        return;
      }
      g.setEdge(value.actedOn.v, value.actedOn.w, value.actedOn);
      if(cyInstance) {
        cyInstance.add({ data: { source: value.actedOn.v, target: value.actedOn.w } });
      }
    } else if (
      value.lastAction === "removeEdge" &&
      value.actedOn != null &&
      !Array.isArray(value.actedOn)
    ) {
      g.removeEdge(value.actedOn.v, value.actedOn.w);
      if(cyInstance) {
        cyInstance.remove(cyInstance.$id(value.actedOn.v).edgesTo(cyInstance.$id(value.actedOn.w)));
      }
    } else if (
      value.lastAction === "removeNode" &&
      value.actedOn != null &&
      Array.isArray(value.actedOn)
    ) {
      g.removeNode(value.actedOn[0]);
      if(cyInstance) {
        cyInstance.remove(cyInstance.$id(value.actedOn[0]));
      }
    } else if (value.lastAction === "resetGraph") {
      g = new graphlib.Graph(); // reset graph
      if(cyInstance) {
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
