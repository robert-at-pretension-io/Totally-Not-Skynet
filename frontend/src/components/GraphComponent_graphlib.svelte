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
      
    } from "../helper_functions/graph";
    import graphlib from "graphlib";
  
    setContext("graphSharedState", {
      getCyInstance: () => cyInstance,
    });
  
    let refElement: HTMLElement | null = null;
    let cyInstance: Core | null = null;
    let g = new graphlib.Graph();
    let id_map = new Map();
    let lastAction = "";
  
    graphStore.subscribe(async (value) => {
      console.log("graphStore value: ", value);
      if (value.lastAction === "addNode" && value.actedOn != null && Array.isArray(value.actedOn)) {
        id_map = id_map.set(value.actedOn[0], value.actedOn[1]);
        console.log("id_map: ", id_map);
        g.setNode(value.actedOn[0], value.actedOn[1]);
      } else if (
        value.lastAction === "addEdge" 
        && value.actedOn != null
        && !Array.isArray(value.actedOn)
      ) {
        g.setEdge(value.actedOn.v, value.actedOn.w, value.actedOn);
      }
      else if (
        value.lastAction === "removeEdge" &&
        value.actedOn != null && !Array.isArray(value.actedOn)
      ) {
        console.log("Removing edge: ", value.actedOn.v, value.actedOn.w);
        g.removeEdge(value.actedOn.v, value.actedOn.w);
      }
      else if (
        value.lastAction === "removeNode" &&
        value.actedOn != null && Array.isArray(value.actedOn)
      ) {
        g.removeNode(value.actedOn[0]);
      }
      else if (
        value.lastAction === "resetGraph"
      ){
        g = new graphlib.Graph(); // reset graph
        resetLastAction();
      }
      lastAction = value.lastAction;
  
      // Now update cytoscape graph based on graphlib graph
      if (cyInstance && (lastAction === "addNode" || lastAction === "addEdge" || lastAction === "removeEdge" || lastAction === "removeNode")) {
        // show the id_map
        console.log("id_map: ", id_map);
        cyInstance.elements().remove(); // clear the cytoscape graph
        const elements : ElementDefinition[] = [];
        g.nodes().forEach(node => {
          console.log("Adding node: ", node, id_map.get(node));
          elements.push({ data: { id: node, label: id_map.get(node) }});
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
        console.log("add event fired, lastAction: ", lastAction);
        if (cyInstance && (lastAction === "addNode" || lastAction === "addEdge" 
        || lastAction === "removeEdge" || lastAction === "removeNode"
        )) {
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

<!-- <style>
  .graph {
    position: absolute;
    left: 400px;
    height: 100%;

    right: 300px;
  }
</style> -->
