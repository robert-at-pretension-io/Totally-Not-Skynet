<script lang="ts">
  import { onMount } from "svelte";
  import type { Process, Node } from "system_types";
  import {
    addEdge,
    addNode,
    getSystemState,
    removeSelectedEdge,
    removeSelectedNode,
    validateGraph,
  } from "../../helper_functions/graph";
  import { Edge, Graph } from "graphlib";
  import { json } from "graphlib";

  import systemStateStore from "stores/systemStateStore";

  let nodes: Node[] = [];
  let selectedNodes: Node[] = [];

  let name = "";
  let description = "";
  let current_graph: Graph = new Graph();

  $: {
    current_graph = $systemStateStore.graphState.graph;
    nodes = $systemStateStore.nodes;
  }

  onMount(async () => {
    current_graph = $systemStateStore.graphState.graph;
    nodes = $systemStateStore.nodes;
  });

  function localAddNodes() {
    // for each of the selected actions, add a node to the graph

    // For each of the selected nodes, check if they are already in the $systemStore.nodes

    // If they are already in the nodes... don't do anything

    // clear out the selected actions

    selectedNodes.forEach((node) => {
      if (!nodeContainedGlobally(node)) {
        $systemStateStore.nodes.push(node);
      }
    });
  }

  function localAddEdge() {
    // get the lastActedOn and actedOn from the graphStore
    let lastActedOn = null;
    let actedOn = null;

    lastActedOn = $systemStateStore.graphState.lastActedOn;
    actedOn = $systemStateStore.graphState.actedOn;

    // check that lastActedOn and actedOn are not null and are arrays
    if (lastActedOn !== null && actedOn !== null && lastActedOn.id) {
      // add an edge between the lastActedOn and actedOn
      let edge: Edge = { v: lastActedOn[0], w: actedOn[0] };

      addEdge(edge);
    } else {
      // console.log("lastActedOn or actedOn is null or not an array");
    }
  }

  async function saveProcess() {
    // create an alert message if either name or description are null
    if (name === null || description === null) {
      alert("Please enter a name and description for the process");
      return;
    } else {
      const systemState = await getSystemState();
      let maybe_topological_order = await validateGraph(systemState);

      if (maybe_topological_order) {
        let topological_order = maybe_topological_order as string[];
        let current_graph_string = JSON.stringify(json.write(current_graph));

        // console.log("current_graph_string: " + current_graph_string);
        let process: Process = {
          Process: {
            graph: current_graph_string,
            initial_variables: [],
            topological_order: topological_order,
          },
        };
        // console.log("sending process: " + JSON.stringify(process));
        console.log("sending process: ", process);
      } else {
        alert("The process does not have a valid topological order :(");
      }
    }
  }

  function isSelected(node: Node): boolean {
    // check to see if selectedNodes : Node[] contains node : Node
    return (
      selectedNodes.filter((val) => {
        val.Node._id.$oid === node.Node._id.$oid;
      }).length > 0
    );
  }

  function toggleSelect(node: Node) {
    // if the node is already in the selectedNodes then remove it, otherwise add it

    console.log("The nodes that are currently selected are:");
    selectedNodes.forEach((node: Node) => {
      console.log(node.Node.name);
    });

    let should_remove = isSelected(node);

    if (should_remove) {
      selectedNodes = selectedNodes.filter((val) => {
        val.Node._id.$oid != node.Node._id.$oid;
      });
    } else {
      selectedNodes.push(node);
    }

    console.log("After running toggleSelect, the nodes are:");
    selectedNodes.forEach((node: Node) => {
      console.log(node.Node.name);
    });
  }
</script>

<p>Please set a descriptive name for your process:</p>
<input type="text" bind:value={name} />
<p>
  Please set a description for your process, please talk about what purpose it
  serves:
</p>
<input type="text" bind:value={description} />

<p>
  Click the node buttons below to add them to the graph. Then click "Add Node(s)
  to see them populate on the graph."
</p>

<ul>
  {#each nodes as node (node.Node._id)}
    <li>
      <button
        class:selected={isSelected(node)}
        type="button"
        on:click={() => toggleSelect(node)}>{node.Node.name}</button
      >
    </li>
  {/each}
</ul>

<h3>Nodes to add:</h3>

{#each selectedNodes as node (node.Node._id)}
  <p>{node.Node.name}</p>
{/each}
<button class="add-button" on:click={localAddNodes}>Add Node(s)</button>
<button class="remove-button" on:click={removeSelectedNode}
  >Remove Node(s)</button
>
<button class="add-button" on:click={localAddEdge}>Add Edge</button>
<button class="remove-button" on:click={removeSelectedEdge}>Remove Edge</button>
<button class="add-button" on:click={saveProcess}>Save Process</button>

<!-- <InteractWithActionsAndProcesses /> -->
