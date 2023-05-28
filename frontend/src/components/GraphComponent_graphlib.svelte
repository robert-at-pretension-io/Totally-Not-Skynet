<script lang="ts">
  import { onMount, setContext } from "svelte";
  import cytoscape, { Core, ElementDefinition } from "cytoscape";
  import dagre from "cytoscape-dagre";
  import GraphStyles from "./GraphStyles.js";
  import systemStateStore from "stores/systemStateStore.js";
  import {
    selectNode,
    resetLastAction,
    selectEdge,
    getGlobalVariableNames,
    checkEdgeVariables,
  } from "../helper_functions/graph";
  import graphlib from "graphlib";
  import { SystemState } from "system_types/index.js";

  setContext("graphSharedState", {
    getCyInstance: () => cyInstance,
  });

  let refElement: HTMLElement | null = null;
  let cyInstance: Core | null = null;
  let g = new graphlib.Graph();
  let id_map = new Map();
  let lastAction = "";

  systemStateStore.subscribe(async (new_value : SystemState) => {
    let value = new_value.graphState;
    if (
      value.lastAction === "addNode" &&
      value.actedOn != null &&
      Array.isArray(value.actedOn)
    ) {
      id_map = id_map.set(value.actedOn[0], value.actedOn[1]);
      // console.log("id_map: ", id_map);
      g.setNode(value.actedOn[0], value.actedOn[1]);
    } else if (
      value.lastAction === "addEdge" &&
      value.actedOn != null &&
      !Array.isArray(value.actedOn)
    ) {
      // Get the nodes of the edge to be added
      // need to get the node id

      const sourceNode = value.actedOn.v;
      const targetNode = value.actedOn.w;

      // Get the global variables
      let globalVariables = getGlobalVariableNames();

      // Check if the output variables of source node (sourceAction) are compatible with the input variables of target node (targetAction)
      const isValidEdge = await checkEdgeVariables(
        sourceNode,
        targetNode,
        globalVariables,
        g
      );

      if (isValidEdge) {
        // Only add edge if it passes the constraint check
        g.setEdge(value.actedOn.v, value.actedOn.w, value.actedOn);
      } else {
        // Otherwise, inform the user or handle invalid edge situation
        alert(
          "Invalid edge. The output variables of the source node are incompatible with the input variables of the target node."
        );
        console.error(
          "Invalid edge. The output variables of the source node are incompatible with the input variables of the target node."
        );
      }
    } else if (
      value.lastAction === "removeEdge" &&
      value.actedOn != null &&
      !Array.isArray(value.actedOn)
    ) {
      // console.log("Removing edge: ", value.actedOn.v, value.actedOn.w);
      g.removeEdge(value.actedOn.v, value.actedOn.w);
    } else if (
      value.lastAction === "removeNode" &&
      value.actedOn != null &&
      Array.isArray(value.actedOn)
    ) {
      g.removeNode(value.actedOn[0]);
    } else if (value.lastAction === "resetGraph") {
      g = new graphlib.Graph(); // reset graph
      resetLastAction();
    }
    lastAction = value.lastAction;

    // Now update cytoscape graph based on graphlib graph

    if (
      cyInstance &&
      (lastAction === "addNode" ||
        lastAction === "addEdge" ||
        lastAction === "removeEdge" ||
        lastAction === "removeNode")
    ) {
      // console.log("Updating cytoscape graph");

      // show the id_map
      // console.log("id_map: ", id_map);
      cyInstance.elements().remove(); // clear the cytoscape graph
      const elements: ElementDefinition[] = [];
      g.nodes().forEach((node) => {
        // console.log("Adding node: ", node, id_map.get(node));
        elements.push({ data: { id: node, label: id_map.get(node) } });
      });
      g.edges().forEach((edge) => {
        // console.log("Adding edge: ", edge);
        // await printEdge(edge);
        elements.push({ data: { source: edge.v, target: edge.w } });
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
      // console.log("add event fired, lastAction: ", lastAction);
      if (
        cyInstance &&
        (lastAction === "addNode" ||
          lastAction === "addEdge" ||
          lastAction === "removeEdge" ||
          lastAction === "removeNode")
      ) {
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
