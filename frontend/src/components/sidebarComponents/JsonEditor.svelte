<!-- JsonEditor.svelte -->
<script>
    import { createEventDispatcher } from "svelte";
    import systemStateStore from "stores/systemStateStore";

    let mainObject = {};

    $: {
      mainObject = $systemStateStore.selectedAction || $systemStateStore.selectedProcess;
      console.log("mainObject: ", mainObject);
    }

    const dispatch = createEventDispatcher();

    function save() {
      dispatch("save", mainObject);
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
