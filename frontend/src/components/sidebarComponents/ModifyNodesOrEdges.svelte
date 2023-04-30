<script lang="ts">
  import { onMount, setContext } from "svelte";
  import {
    addEdge,
    getGraphState,
    getUniqueId,
    resetLastAction,
    updateEdge,
    updateNode,
  } from "../../helper_functions/graph";
  import type { GraphState } from "../../system_types";
  import { graphStore } from "../../stores/graphStore";
  import { Core } from "cytoscape";

  let label = "";
  let data = "";
  let target = "";

  let graphState: GraphState = {
    selected: null,
    graph: {
      nodes: [],
      edges: [],
    },
    lastAction: "none",
    actedOn: null,
  };

  setContext("graphSharedState", {
    getCyInstance: () => cyInstance,
  });

  let cyInstance: Core | null = null;

  graphStore.subscribe((value) => {
    if (value.lastAction === "selectNode") {
      graphState = value;
      resetLastAction();
    } else if (value.lastAction === "selectEdge") {
      graphState = value;
      resetLastAction();
    }
  });

  onMount(async () => {
    graphState = await getGraphState();
  });
</script>

<div>
  {#if graphState.selected != null}
    {#if graphState.selected.type === "Edge"}
      <h2>Selected Edge:</h2>
      <p>ID: {graphState.selected.instance.id}</p>
      <p>Source: {graphState.selected.instance.source}</p>
      <p>Target: {graphState.selected.instance.target}</p>
    {:else if graphState.selected.type === "Node"}
      <h2>Selected Node:</h2>
      <p>ID: {graphState.selected.instance.id}</p>
      <p>Data: {JSON.stringify(graphState.selected.instance.data)}</p>
    {/if}
  {:else}
    <h2>No Node or Edge Selected</h2>
  {/if}
</div>

<div>
  {#if graphState.selected != null}
    {#if graphState.selected.type === "Edge"}
      <h3>Update Edge</h3>
      <label for="label">Label:</label>
      <input type="text" id="label" bind:value={label} />
      <label for="data">Data:</label>
      <input type="text" id="data" bind:value={data} />
      <button
        on:click={() =>
          updateEdge(graphState.selected.instance.id, label, data)}
        >Update Edge</button
      >
    {:else if graphState.selected.type === "Node"}
      <h3>Update Node</h3>
      <label for="label">Label:</label>
      <input type="text" id="label" bind:value={label} />
      <label for="data">Data:</label>
      <input type="text" id="data" bind:value={data} />
      <button
        on:click={() =>
          updateNode(graphState.selected.instance.id, label, data)}
        >Update Node</button
      >
    {/if}
  {/if}
</div>

<div>
  {#if graphState.selected != null}
    {#if graphState.selected.type === "Node" && graphState.selected != null && graphState.selected.neighbors && graphState.selected.neighbors.length > 0}
      <h3>Add Edge</h3>
      <label for="target">Target Node:</label>
      <select id="target" bind:value={target}>
        {#each graphState.selected.neighbors as node}
          <option value={node.id}>{node.id}</option>
        {/each}
      </select>
      <button
        on:click={async () =>
          addEdge({
            id: await getUniqueId(),
            source: graphState.selected.instance.id,
            target,
          })}>Add Edge</button
      >
    {/if}
  {/if}
</div>
