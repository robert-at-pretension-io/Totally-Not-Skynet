<script lang="ts">
  import { onMount } from "svelte";
  import { aiSystemStore } from "stores/aiSystemStore";
  import type { Action, Process } from "system_types";
  import {
    addEdge,
    addNode,
    removeSelectedEdge,
    removeSelectedNode,
    topologicalSort,
  } from "../../helper_functions/graph";
  import { graphStore } from "../../stores/graphStore";
  import { Graph } from "graphlib";
  import websocketStore from "stores/websocketStore";

  import InteractWithActionsAndProcesses from "./InteractWithActionsAndProcesses.svelte";

  import { json } from "graphlib";

  let actions: Action[] = [];
  let selectedActions: Action[] = [];

  let name = "";
  let description = "";
  let current_graph: Graph = new Graph();

  graphStore.subscribe((value) => {
    current_graph = value.graph;
  });

  onMount(async () => {
    aiSystemStore.subscribe((value) => {
      actions = value.actions;

      // loop through the actions and print their values:
      // for (let i = 0; i < actions.length; i++) {
      //   console.log("Action " + i + ": " + JSON.stringify(actions[i]));
      // }
    });
  });

  function localAddNodes() {
    // for each of the selected actions, add a node to the graph
    selectedActions.forEach((action) => {
      // console.log("local");
      addNode(action._id.$oid);
    });

    // clear out the selected actions
    selectedActions = [];
  }

  function localAddEdge() {
    // get the lastActedOn and actedOn from the graphStore
    let lastActedOn = null;
    let actedOn = null;

    graphStore.subscribe((value) => {
      lastActedOn = value.lastActedOn;
      actedOn = value.actedOn;
    });

    // check that lastActedOn and actedOn are not null and are arrays
    if (
      lastActedOn !== null &&
      actedOn !== null &&
      Array.isArray(lastActedOn) &&
      Array.isArray(actedOn)
    ) {
      // add an edge between the lastActedOn and actedOn
      let edge = { v: lastActedOn[0], w: actedOn[0] };

      addEdge(edge);
    } else {
      // console.log("lastActedOn or actedOn is null or not an array");
    }
  }

  function saveProcess() {
    // create a process object

    console.log(
      "saveProcess with input name and description: ",
      name,
      description
    );

    // create an alert message if either name or description are null
    if (name === null || description === null) {
      alert("Please enter a name and description for the process");
      return;
    } else {
      let topologicalOrder: string[] = topologicalSort(current_graph);

      console.log(
        "current_graph: " +
          JSON.stringify(current_graph) +
          "\nWith topological order: " +
          topologicalOrder
      );

      let current_graph_string = json.write(current_graph);

      console.log("current_graph_string: " + current_graph_string);
      let process: Process = {
        _id: { $oid: "" },
        name: name,
        description: description,
        graph: current_graph_string,
        topological_order: topologicalOrder,
      };
      console.log("sending process: " + JSON.stringify(process));
      $websocketStore.send(JSON.stringify({ create_process: process }));
      selectedActions = [];
    }
  }

  function toggleSelect(action: Action) {
    // console.log("toggleSelect called on action: ", action);
    const index = selectedActions.findIndex(
      (a) => a._id.$oid === action._id.$oid
    );
    if (index !== -1) {
      // action is currently selected, remove it
      selectedActions = selectedActions.filter(
        (a) => a._id.$oid !== action._id.$oid
      );
    } else {
      // action is not currently selected, add it
      selectedActions = [...selectedActions, action];
    }
    // console.log("selectedActions after toggleSelect:", selectedActions);
  }

  function isSelected(action: Action) {
    // console.log("isSelected called on action: ", action);
    let is_selected = selectedActions.some(
      (a) => a._id.$oid === action._id.$oid
    );
    // console.log("selectedActions during isSelected:", selectedActions);
    // console.log("isSelected returning: ", is_selected);
    return is_selected;
  }
</script>

<!--  name: string;
  graph: Graph;
  description: string;
};-->

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
  {#each actions as action (action._id)}
    <li>
      <button
        class:selected={isSelected(action)}
        type="button"
        on:click={() => toggleSelect(action)}>{action.name}</button
      >
    </li>
  {/each}
</ul>

<h3>Nodes to add:</h3>

{#each selectedActions as action (action._id)}
  <p>{action.name}</p>
{/each}
<button class="add-button" on:click={localAddNodes}>Add Node(s)</button>
<button class="remove-button" on:click={removeSelectedNode}
  >Remove Node(s)</button
>
<button class="add-button" on:click={localAddEdge}>Add Edge</button>
<button class="remove-button" on:click={removeSelectedEdge}>Remove Edge</button>
<button class="add-button" on:click={saveProcess}>Save Process</button>

<InteractWithActionsAndProcesses />
