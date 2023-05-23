<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import websocketStore from "stores/websocketStore";
  import { Action, Process, UpdateAction } from "system_types";
  import { isProcess, isAction } from "helper_functions/type_checker";
  let mainObject : Action | Process;

  $: {
    mainObject = $systemStateStore.selectedAction || $systemStateStore.selectedProcess;
  }

  function save() {
    if (isAction(mainObject)){
      let updateAction : UpdateAction = {
        action: mainObject
      };
      // console.log("sending: " + JSON.stringify(updateAction));
      $websocketStore.send(JSON.stringify(updateAction));
    }
  }
</script>

<div class="json-editor">
  {#if mainObject !== null && Object.keys(mainObject).length > 0}
    {#each Object.entries(mainObject) as [key, value], index (index)}
      {#if key !== "_id" && key != "graph"} <!-- Skip entities with key of "_id" -->
        <div class="object-field">
          <label for="input-{index}">{key}:</label>
          {#if typeof value === "boolean" && (isAction(mainObject) || isProcess(mainObject))}
            <input id="input-{index}" type="checkbox" bind:checked={mainObject[key]} />
          {:else if typeof value === "number" && (isAction(mainObject) || isProcess(mainObject))}
            <input id="input-{index}" type="number" bind:value={mainObject[key]} />
          {:else if (isAction(mainObject) || isProcess(mainObject))}
            <textarea id="input-{index}" rows="1" bind:value={mainObject[key]} />
          {/if}
        </div>
      {/if}
    {/each}
    <button on:click={save}>Save</button>
  {/if}
</div>
