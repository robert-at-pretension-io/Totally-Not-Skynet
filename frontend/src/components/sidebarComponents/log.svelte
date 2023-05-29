<script lang="ts">
import systemStateStore from "stores/systemStateStore";

let localSystemState;
let responses : Map<string, string> = new Map();
let prompts : Map<string, string> = new Map();
let topological_order : string[] = [];

$: {
  localSystemState = $systemStateStore.executionContext;
  responses = localSystemState.responses;
  prompts = localSystemState.prompts;
  topological_order = localSystemState.topological_order;

  console.log("responses", responses);
  console.log("prompts", prompts);
  console.log("topological_order", topological_order);
}
</script>

<!-- loop through the responses and prompts and display them -->
{#each topological_order as node}
    {#if node in prompts}
        <div class="prompt">
            <h3>{node}</h3>
            <p>{prompts.get(node)}</p>
        </div>
    {/if}
    {#if node in responses}
    <div class="response">
        <h3>{node}</h3>
        <p>{responses.get(node)}</p>
    </div>
{/if}
{/each}
