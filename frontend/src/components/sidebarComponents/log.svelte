<script lang="ts">
  import { onDestroy } from "svelte";
  import systemStateStore from "stores/systemStateStore";
  import { SystemState } from "system_types";
  
  let responses : Map<string, string> = new Map();
  let prompts : Map<string, string> = new Map();
  let topological_order : string[] = [];
  
  const unsubscribe = systemStateStore.subscribe((value : SystemState) => {
    responses = value.executionContext.responses;
    prompts = value.executionContext.prompts;
    topological_order = value.executionContext.topological_order;
  });
  
  onDestroy(() => {
    unsubscribe();
  });
  </script>
  
  {#each topological_order as node}
      <h2>{node}</h2>
      {#if prompts.has(node)}
          <div class="prompt">
              <h3>{node}</h3>
              <p>{prompts.get(node)}</p>
          </div>
      {/if}
      {#if responses.has(node)}
      <div class="response">
          <h3>{node}</h3>
          <p>{responses.get(node)}</p>
      </div>
  {/if}
  {/each}
