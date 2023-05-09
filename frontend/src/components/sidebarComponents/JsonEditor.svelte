<!-- JsonEditor.svelte -->
<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import systemStateStore from "stores/systemStateStore";
    import websocketStore from "stores/websocketStore";
    import { Action, Process, UpdateAction } from "system_types";

    let mainObject : Action | Process | null | {} = {};

    $: {
      mainObject = $systemStateStore.selectedAction || $systemStateStore.selectedProcess;
      console.log("mainObject: ", mainObject);
    }

    const dispatch = createEventDispatcher();

    function save() {
      if (mainObject !== null && mainObject.prompt !== undefined){
        let updateAction : UpdateAction = {
          action: mainObject
        };
        console.log("sending: " + JSON.stringify(updateAction));
        $websocketStore.send(JSON.stringify(updateAction));
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
                {#if typeof value === "object" && value !== null}
                    <!-- <svelte:self subobject={value} on:save={(e) => jsonObject[key] = e.detail} /> -->
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
