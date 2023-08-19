<script lang="ts">
  import {
    GraphNodeInfo,
    Graph,
    Node,
    Edge,
    Process,
  } from "generated/system_types_pb.js";

  import * as helper_functions from "../../helper_functions/graph";

  let selected_nodes: GraphNodeInfo[] = [];
  let selected_edge: Edge | null = null;

  let graph = new Graph();
  let name = "";
  let description = "";

  // async function handleGraphError() {
  //   await handleError({ name: "GraphDoesntExist" });
  // }

  async function saveProcess() {
    // create an alert message if either name or description are null
    if (name === null || description === null) {
      alert("Please enter a name and description for the process");
      return;
    } else {
      const systemState = await helper_functions.getSystemState();
      let graph_state = systemState.getGraphState();
      let maybe_topological_order = await helper_functions.validateGraph(
        systemState
      );

      if (maybe_topological_order && graph_state != undefined) {
        let topological_order = maybe_topological_order as GraphNodeInfo[];

        // console.log("current_graph_string: " + current_graph_string);
        let process = new Process();

        process.setGraphState(graph_state);
        process.setInitialVariablesList([]);
        process.setTopologicalOrderList(topological_order);

        let new_node = new Node();

        new_node.setName(name);
        new_node.setDescription(description);
        new_node.setProcess(process);

        alert("todo: save process by sending websocket message");
      } else {
        alert("The process does not have a valid topological order :(");
      }
    }
  }
  function isSelected(node: GraphNodeInfo): boolean {
    // check to see if selected_nodes : Node[] contains node : Node
    return (
      selected_nodes.filter((val) => {
        val.getId() === node.getId();
      }).length > 0
    );
  }
  function removeNodes() {
    let current = graph.getNodesList();

    let new_nodes = current.filter((node: GraphNodeInfo) => {
      return !selected_nodes.includes(node);
    });

    graph.setNodesList(new_nodes);

    selected_nodes = [];
  }
  function addNodes() {
    let current_nodes = graph.getNodesList();
    selected_nodes.forEach((node: GraphNodeInfo) => {
      // check if current_nodes already contains node
      if (!current_nodes.includes(node)) {
        current_nodes.push(node);
      }
    });
    graph.setNodesList(current_nodes);
    selected_nodes = [];
  }
  function addEdge() {
    let current_edges = graph.getEdgesList();
    // add selected_edge : Edge to current_edges : Edge[]
    if (selected_edge != null) {
      current_edges.push(selected_edge);
    }
    graph.setEdgesList(current_edges);
  }
  function removeEdge() {
    let current_edges = graph.getEdgesList();
    // remove selected_edge : Edge from current_edges : Edge[]
    if (selected_edge != null) {
      current_edges = current_edges.filter((edge: Edge) => {
        return edge != selected_edge;
      });
    }
    graph.setEdgesList(current_edges);
  }

  function toggleNodeSelect(node: GraphNodeInfo) {
    if (isSelected(node)) {
      selected_nodes = selected_nodes.filter((val) => {
        val != node;
      });
    } else {
      selected_nodes.push(node);
    }
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
  {#each graph.getNodesList() as node (node.getId())}
    <li>
      <button
        class:selected={isSelected(node)}
        type="button"
        on:click={() => toggleNodeSelect(node)}>{node.getName()}</button
      >
    </li>
  {/each}
</ul>

<h3>Nodes to add:</h3>

{#each selected_nodes as node (node.getId())}
  <p>{node.getName()}</p>
{/each}
<button class="add-button" on:click={addNodes}>Add Node(s)</button>
<button class="remove-button" on:click={removeNodes}>Remove Node(s)</button>
<button class="add-button" on:click={addEdge}>Add Edge</button>
<button class="remove-button" on:click={removeEdge}>Remove Edge</button>
<button class="add-button" on:click={saveProcess}>Save Process</button>

<!-- <InteractWithActionsAndProcesses /> -->
