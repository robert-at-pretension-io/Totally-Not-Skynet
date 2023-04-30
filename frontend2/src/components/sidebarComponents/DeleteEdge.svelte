<script lang="ts">
    import { onMount, setContext } from "svelte";
    import {
      getGraphState,
      resetLastAction,
      removeEdge,
    } from "../../helper_functions/graph";
    import type { GraphState } from "../../system_types";
    import { graphStore } from "../../stores/graphStore";
  import { Core } from "cytoscape";
   
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
      {:else}
        <h2>No selected Node or Edge</h2>
      {/if}
      <button on:click={() => {
        if(graphState.selected?.type === "Edge") {
          removeEdge(graphState.selected.instance.source, graphState.selected.instance.target);
        }
      }}>
        Delete Selected Edge
      </button>
    {:else}
      <h2>No selected Node or Edge</h2>
    {/if}
  </div>