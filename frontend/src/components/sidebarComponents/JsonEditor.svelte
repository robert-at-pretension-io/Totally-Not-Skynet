<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import { aiSystemStore } from "stores/aiSystemStore";
  import websocketStore from "stores/websocketStore";
  import { Action, Process, UpdateAction } from "system_types";
  import Select from "svelte-select";
import { isProcess, isAction } from "helper_functions/type_checker";
  let mainObject : Action | Process | null | {} = {};

  let options : {value: string, label: string}[]= [];

  $: {
    Action= $systemStateStore.selectedAction || $systemStateStore.selectedProcess;
    console.log("mainObject: ", mainObject);

    if (isProcess(mainObject)) {
      options = mainObject.steps.map(step => ({value: step, label: step}));
    }
  }

  function save() {
    if (isAction(mainObject)){
      let updateAction : UpdateAction = {
        action: mainObject
      };
      console.log("sending: " + JSON.stringify(updateAction));
      $websocketStore.send(JSON.stringify(updateAction));
    }
  }

  function handleStepsChange(selected : {value : string, label: string}, index : number) {
    if (isProcess(mainObject)){
      mainObject.steps[index] = selected.value;
      options[index] = {value: selected.value, label: selected.value};
      mainObject = {...mainObject}; // trigger reactivity
    }
  }

</script>

<style>
textarea {
  resize: vertical;
  width: 100%;
}
</style>

<div class="json-editor">
  {#if mainObject !== null && Object.keys(mainObject).length > 0}
      {#each Object.entries(mainObject) as [key, value], index (index)}
          <div class="object-field">
              <label for="input-{index}">{key}:</label>
              {#if key === "steps" && Array.isArray(value)}
              {#each value as step, index (index)}
              <Select id="steps_{index}"
value={options[index]}
items={$aiSystemStore.actions.map(action => ({value: action.name, label: action.name}))}
on:change={(event) => handleStepsChange(event.detail, index)}
placeholder="Select step..."
/>
              {/each}
              {:else if typeof value === "boolean"}
                  <input id="input-{index}" type="checkbox" bind:checked={mainObject[key]} />
              {:else if typeof value === "number"}
                  <input id="input-{index}" type="number" bind:value={mainObject[key]} />
              {:else}
                  <textarea id="input-{index}" rows="1" bind:value={mainObject[key]} />
              {/if}
          </div>
      {/each}
      <button on:click={save}>Save</button>
  {/if}
</div>