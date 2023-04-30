<script lang="ts">
  import type { Node } from "../../system_types";

  import { getUniqueId, addNode } from "../../helper_functions/graph";

  let label = "Node Label";

  async function localAddNode() {
    let id = await getUniqueId();

    let newNode: Node = {
      id: id,
      label: label,
      data: {},
    };

    await addNode(newNode);
  }
</script>

<div id="cy" />
<input type="text" bind:value={label} placeholder="Enter label for new node" />
<button on:click={localAddNode}>Add Node</button>

<style>
  input {
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    white-space: pre-wrap;
  }
</style>
