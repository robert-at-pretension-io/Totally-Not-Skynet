<script lang="ts">
  import { onMount, setContext } from "svelte";
  import cytoscape, { Core } from "cytoscape";
  import systemStateStore from "stores/systemStateStore.js";
  import * as helper_functions from "../helper_functions/graph";
  import * as proto from "../generated/system_types";
  import dagre from "cytoscape-dagre";
  // import { generateDynamicStyles } from "../helper_functions/graph";

  let current_graph: proto.Graph = new proto.Graph();

  onMount(() => {
    console.log("Graph Component Mounted");

    cytoscape.use(dagre);

    // if ($systemStateStore.graph != undefined) {
    //   current_graph = $systemStateStore.graph as proto.Graph;
    // } else {
    //   current_graph = new proto.Graph();
    // }

    // const dynamicStyles = generateDynamicStyles(); // Generate the dynamic styles

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
        // ...dynamicStyles,
      ],
    });

    cyInstance.on("select", "node", (evt) => {
      console.log("event: ", evt);

      const selectedNode = evt.target.data();

      console.log("selectedNode: ", selectedNode);

      $systemStateStore.local_nodes.find((node) => {
        if (node.node_info.id == selectedNode.id) {
          $systemStateStore.selected_nodes.push(node.node_info);
        }
      });
    });

    cyInstance.on("select", "edge", (evt) => {
      console.log("event: ", evt);

      let edge = new proto.Edge();

      let source = new proto.GraphNodeInfo();
      let target = new proto.GraphNodeInfo();

      source.id = evt.target.data().source;
      target.id = evt.target.data().target;

      edge.source = source;
      edge.target = target;

      console.log("selectedEdge: ", evt.target.data());

      $systemStateStore = helper_functions.selectEdge(edge, $systemStateStore);
    });

    cyInstance.on("unselect", "node", function (evt) {
      const node = evt.target;
      console.log("deselected " + node.id());

      let selected_list = $systemStateStore.selected_nodes;

      // remove the node the graphNodeInfo from the selected_list where the id is the same as the node.id()

      selected_list = selected_list.filter(
        (graphNodeInfo: proto.GraphNodeInfo) => {
          return graphNodeInfo.id == node.id();
        }
      );

      $systemStateStore.selected_nodes = selected_list;
    });

    // Listen to unselect event on any edge
    cyInstance.on("unselect", "edge", function (evt) {
      const edge = evt.target;
      console.log(
        "deselected " + edge.data().source + " -> " + edge.data().target
      );
    });
    // if (current_graph !== undefined) {
    //   draw_graph(cyInstance, current_graph);
    // }
  });

  $: {
    // Whenever the systemState.graph changes, we will change the cytoscape graph. It might be good to check if the graph has actually changed rather than always re-draw

    let test_graph =
      $systemStateStore.selected_process?.node_content.process.graph;

    // check that the test_graph is different from the current_graph
    if (test_graph != current_graph && test_graph != undefined) {
      current_graph = test_graph;
      draw_graph(cyInstance, current_graph);
    }
  }

  setContext("graphSharedState", {
    getCyInstance: () => cyInstance,
  });

  let refElement: HTMLElement | null = null;
  let cyInstance: Core | null = null;

  function draw_graph(cyInstance, current_graph) {
    // clear the cytoscape graph
    cyInstance?.elements().remove();

    // add the nodes to the cytoscape graph
    let nodes = current_graph.nodes_info;

    nodes.forEach((node_info) => {
      if (node_info) {
        cyInstance?.add({
          data: {
            id: node_info.id,
            label: node_info.name,
          },
        });
      }
    });

    // add the edges to the cytoscape graph
    let edges = current_graph.edges;

    edges.forEach((edge: proto.Edge) => {
      let source = edge.source;
      let target = edge.target;

      if (source && target) {
        cyInstance?.add({
          data: {
            source: source.id,
            target: target.id,
          },
        });
      }
    });

    // cyInstance?.fit();

    cyInstance
      ?.layout({
        name: "dagre",
        fit: true,
      })
      .run();
  }
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
