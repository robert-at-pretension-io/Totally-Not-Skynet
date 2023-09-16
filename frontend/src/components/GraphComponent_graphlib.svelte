<script lang="ts">
  import { onMount, setContext } from "svelte";
  import cytoscape, { Core } from "cytoscape";
  import dagre from "cytoscape-dagre";
  import systemStateStore from "stores/systemStateStore.js";
  import * as helper_functions from "../helper_functions/graph";
  import * as proto from "../generated/system_types_pb";

  import * as graphlib from "graphlib";

  onMount(() => {
    console.log("Graph Component Mounted");

    cytoscape.use(dagre);

    cyInstance = cytoscape({
      container: refElement,
      style: [
        {
          selector: "node",
          style: {
            width: "label",
            height: "label",
            "font-size": "14px",
            "font-weight": "bold",
            content: "data(label)",
            "text-valign": "center",
            "text-wrap": "wrap",
            "text-max-width": "100px",
            "background-color": "#fff",
            "border-color": "#000",
            "border-width": "1px",
            "border-style": "solid",
          },
        },
        {
          selector: "edge",
          style: {
            "curve-style": "bezier",
            "target-arrow-shape": "triangle",
            "line-color": "#000",
            "target-arrow-color": "#000",
            width: "2px",
          },
        },
      ],
    });

    layout = cyInstance.layout({
      name: "dagre" /* or whatever layout you're using */,
    });

    cyInstance.on("select", "node", (evt) => {
      console.log("event: ", evt);

      const selectedNode = evt.target.data();

      console.log("selectedNode: ", selectedNode);

      $systemStateStore = helper_functions.selectNode(
        selectedNode.id,
        $systemStateStore
      );
    });

    cyInstance.on("select", "edge", (evt) => {
      console.log("event: ", evt);

      let edge = new proto.Edge();

      let source = new proto.GraphNodeInfo();
      let target = new proto.GraphNodeInfo();

      source.setId(evt.target.data().source);
      target.setId(evt.target.data().target);

      edge.setSource(source);
      edge.setTarget(target);

      console.log("selectedEdge: ", evt.target.data());

      $systemStateStore = helper_functions.selectEdge(edge, $systemStateStore);
    });
  });

  setContext("graphSharedState", {
    getCyInstance: () => cyInstance,
  });

  let refElement: HTMLElement | null = null;
  let cyInstance: Core | null = null;
  let g = new graphlib.Graph();

  let layout: cytoscape.Layouts;
</script>

<div class="graph" bind:this={refElement}>
  {#if cyInstance}
    <slot />
  {/if}
</div>

<style>
  .graph {
    grid-column: 2;
    height: 100%;
  }
</style>
