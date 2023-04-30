<script lang="ts">
  /* 
This Svelte component is used to create a Cytoscape graph. 
It will listen to the graphStore and populate the graph with 
nodes and edges, as well as allow for selection of nodes and edges. 
It also includes a slot for additional components to be rendered 
within the graph. 
*/

  import { onMount, setContext } from "svelte";
  import cytoscape, { Core } from "cytoscape";
  import dagre from "cytoscape-dagre";
  import GraphStyles from "./GraphStyles.js";
  // import { Node, Edge, Graph } from '../system_types/graph';
  import { graphStore } from "../stores/graphStore";
  import {
    edges,
    nodes,
    resetLastAction,
    selectNode,
    selectEdge,
  } from "../helper_functions/graph";

  // Set context to graphSharedState, which can be accessed from other components
  setContext("graphSharedState", {
    getCyInstance: () => cyInstance,
  });

  // Initialize refElement as null and cyInstance as null
  let refElement: HTMLElement | null = null;
  let cyInstance: Core | null = null;

  // Listen to graphStore and add nodes and edges to the graph
  graphStore.subscribe((value) => {
    console.log("graphStore", value);
    if (cyInstance && value.lastAction === "addNode" && value.actedOn != null) {
      cyInstance.add({
        group: "nodes",
        data: { ...value.actedOn },
      });
      resetLastAction();
    } else if (
      cyInstance &&
      value.lastAction === "addEdge" &&
      value.actedOn != null
    ) {
      cyInstance.add({
        group: "edges",
        data: { ...value.actedOn },
      });
      resetLastAction();
    } else if (
      cyInstance &&
      value.lastAction === "updateNode" &&
      value.actedOn != null
    ) {
      cyInstance.$("#" + value.actedOn.id).css({
        label: value.actedOn.label,
      });
      cyInstance.$("#" + value.actedOn.id).data(value.actedOn.data);
    } else if (
      cyInstance &&
      value.lastAction === "updateEdge" &&
      value.actedOn != null
    ) {
      cyInstance.$("#" + value.actedOn.id).css({
        label: value.actedOn.label,
      });
      cyInstance.$("#" + value.actedOn.id).data(value.actedOn.data);
    } else if (
      cyInstance &&
      value.lastAction === "removeEdge" &&
      value.actedOn != null
    ) {
      cyInstance.$("#" + value.actedOn.id).remove();
      resetLastAction();
    }
  });

  // Get nodes and edges from the graphStore on component mount
  onMount(async () => {
    nodes().then((nodes) => {
      nodes.forEach((node) => {
        cyInstance?.add({
          group: "nodes",
          data: { ...node },
        });
      });
    });

    edges().then((edges) => {
      edges.forEach((edge) => {
        cyInstance?.add({
          group: "edges",
          data: { ...edge },
        });
      });
    });

    // Use the dagre plugin to layout the graph
    cytoscape.use(dagre);

    // Instantiate the cytoscape instance and set the styles
    cyInstance = cytoscape({
      container: refElement,
      style: GraphStyles,
    });

    // Layout the graph when a node or edge is added
    cyInstance.on("add", () => {
      if (cyInstance) {
        cyInstance
          .layout({
            name: "dagre",
          })
          .run();
      }
    });

    // Listen for node selection and call selectNode
    cyInstance.on("select", "node", (evt) => {
      const selectedNode = evt.target.data();
      selectNode(selectedNode.id);
    });

    // Listen for edge selection and call selectEdge
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
    left: 200px;
    height: 100%;
    width: 100%;
  }
</style>
