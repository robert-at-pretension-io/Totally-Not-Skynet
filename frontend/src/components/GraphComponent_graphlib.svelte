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
    getActionById,
    getAncestorNodes,
    getGlobalVariableNames,
    getNodeName,
  } from "../helper_functions/graph";
  import graphlib, { Graph } from "graphlib";
  import { Action } from "system_types/index.js";

  setContext("graphSharedState", {
    getCyInstance: () => cyInstance,
  });

  let refElement: HTMLElement | null = null;
  let cyInstance: Core | null = null;
  let g = new graphlib.Graph();
  let id_map = new Map();
  let lastAction = "";
  let lastActionValid = false;

  async function checkEdgeVariables(
    sourceNode: string,
    targetNode: string,
    globalVariables: string[]
  ): Promise<boolean> {
    let sourceName = await getNodeName(sourceNode);
    let targetName = await getNodeName(targetNode);
    console.log(
      "Checking edge variables between nodes ",
      sourceName,
      " and ",
      targetName
    );

    // Get the input variables of target action
    let targetAction = await getActionById(targetNode);
    if (targetAction == null) {
      console.log("targetAction is null");
      return false;
    }
    const targetInputVariables = targetAction.input_variables;
    console.log("Target Action input variables: ", targetInputVariables);

    // Get the output variables of source node
    let sourceAction = await getActionById(sourceNode);
    if (sourceAction == null) {
      console.log("sourceAction is null");
      return false;
    }
    const sourceOutputVariables = sourceAction.output_variables;
    console.log("Source Action output variables: ", sourceOutputVariables);

    // Get all ancestor nodes of the target node
    const ancestorNodes = await getAncestorNodes(targetNode, g);
    console.log("Ancestor Nodes of the target node: ", ancestorNodes);

    // Collect the output variables of all ancestor nodes
    const ancestorOutputVariables = ancestorNodes.flatMap(
      (node) => node.output_variables
    );
    console.log(
      "Output variables of the ancestor nodes: ",
      ancestorOutputVariables
    );

    // Combine the output variables of the source node, the ancestor nodes, and the global variables
    const allValidInputs = [
      ...sourceOutputVariables,
      ...ancestorOutputVariables,
      ...globalVariables,
    ];
    console.log("All valid inputs: ", allValidInputs);

    // Ensure every input variable of the target node exists in the combined array of valid input variables
    const isValid = targetInputVariables.every((variable) =>
      allValidInputs.includes(variable)
    );
    console.log("Are all target input variables valid? ", isValid);

    return isValid;
  }

  graphStore.subscribe(async (value) => {
    // console.log("graphStore value: ", value);
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
        globalVariables
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
        // You could also set a variable 'lastActionValid' to false here and use it to show the error in your UI
        lastActionValid = false;
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
      // show the id_map
      // console.log("id_map: ", id_map);
      cyInstance.elements().remove(); // clear the cytoscape graph
      const elements: ElementDefinition[] = [];
      g.nodes().forEach((node) => {
        // console.log("Adding node: ", node, id_map.get(node));
        elements.push({ data: { id: node, label: id_map.get(node) } });
      });
      g.edges().forEach((edge) => {
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
