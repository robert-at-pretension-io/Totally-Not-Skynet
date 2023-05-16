<script lang="ts">
    import { onMount, setContext } from "svelte";
    import cytoscape, { Core, ElementDefinition } from "cytoscape";
    import dagre from "cytoscape-dagre";
    import GraphStyles from "./GraphStyles.js";
    import { graphStore } from "../stores/graphStore";
    import {
      selectNode,
      resetLastAction,
      selectEdge,
      getNodeName,
      
    } from "../helper_functions/graph";
    import graphlib from "graphlib";
  
    setContext("graphSharedState", {
      getCyInstance: () => cyInstance,
    });
  
    let refElement: HTMLElement | null = null;
    let cyInstance: Core | null = null;
    let g = new graphlib.Graph();
    let id_map = new Map();
  
    graphStore.subscribe((value) => {
      console.log("graphStore value: ", value);
      if (value.lastAction === "addNode" && value.actedOn != null && typeof value.actedOn === "string") {
        g.setNode(value.actedOn, value.actedOn);
        id_map.set(value.actedOn, value.name);
        resetLastAction();
      } else if (
        value.lastAction === "addEdge" &&
        value.actedOn != null && typeof value.actedOn === "object"
      ) {
        g.setEdge(value.actedOn.v, value.actedOn.w, value.actedOn);
        resetLastAction();
      } else if (
        value.lastAction === "removeEdge" &&
        value.actedOn != null
      ) {
        g.removeEdge(value.actedOn.v, value.actedOn.w);
        resetLastAction();
      }
      else if (
        value.lastAction === "removeNode" &&
        value.actedOn != null
      ) {
        g.removeNode(value.actedOn);
        resetLastAction();
      }
      else if (
        value.lastAction === "resetGraph"
      ){
        g = new graphlib.Graph(); // reset graph
        resetLastAction();
      }
  
      // Now update cytoscape graph based on graphlib graph
      if (cyInstance) {
        cyInstance.elements().remove(); // clear the cytoscape graph
        const elements : ElementDefinition[] = [];
        g.nodes().forEach(node => {
          elements.push({ data: { id: node, name: id_map.get(node) }});
        });
        g.edges().forEach(edge => {
          elements.push({ data: { source: edge.v, target: edge.w,  } });
        });
        cyInstance.add(elements); // add new elements
      }
    });
  
    onMount(async () => {
      
      cytoscape.use(dagre);
  
      cyInstance = cytoscape({
        container: refElement,
        style: GraphStyles,
      });
  
      cyInstance.on("add", () => {
        if (cyInstance) {
          cyInstance
            .layout({
              name: "dagre",
            })
            .run();
        }
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

<style>
  .graph {
    position: absolute;
    left: 300px;
    height: 100%;

    right: 300px;
  }
</style>
