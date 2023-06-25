<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import { returnProcesses } from "helper_functions/graph";
  import { isProcess, newAction } from "helper_functions/type_checker";
  import type { Node } from "system_types";
  import Log from "./log.svelte";
  import { onMount } from "svelte";
  import { some } from "fp-ts/lib/Option";

  let selectedNode: Node | null = null;
  let processes : Node[]= [];

  onMount(async () => {
    processes = await returnProcesses();
  });

  let newValues = {};

  function handleInputChange(event: Event) {
    if (event.target) {
      console.log("handleInputChange called: ", event.target);
    }
  }

  async function handleFormSubmit(event: Event) {
    // should send the "HandleNode" action to the backend
    console.log("handleFormSubmit called: ", selectedNode);
  }
  async function onDropdownChange() {
    if (selectedNode) {
      $systemStateStore.selectedNode = some(selectedNode);

    }
  }
</script>

{#if selectedNode}
  <div>

    <!-- <h2>Global Variables</h2>
    {#each [...globalVariables] as [key, value]}
      <p>{key}: {value}</p>
    {/each} -->

    <!-- check if the needed variables array has length 0:-->
    <!-- {#if !ready_to_make_first_prompt}
      <h2>Needed Variables</h2>
      {#each needed_variables as needed_var (needed_var)}
        <form on:submit={async (event) => await handleFormSubmit(event)}>
          <label for={needed_var}>{needed_var}</label>
          <input
            id={needed_var}
            on:input={(event) => handleInputChange(needed_var, event)}
          />
          <button type="submit">Update</button>
        </form>
      {/each}
    {:else if !already_made_first_prompt}
      <button class="add-button" on:click={sendFirstPrompt}>Execute</button>
    {:else}
    -->
      <!-- Add the displaying of output log here 
      <Log />
    {/if} -->
  </div>
{:else}
  <h1>Selected a Process:</h1>
  <select
    bind:value={selectedNode}
    on:change={async () => await onDropdownChange()}
  >
    <option value="">Select a process</option>
    {#each processes as process}
      <option value={process.name}>{process.name}</option>
    {/each}
  </select>
{/if}
