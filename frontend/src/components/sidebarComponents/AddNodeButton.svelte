<script lang="ts">
  import type { Prompt } from "../../system_types";

  import systemStateStore from "stores/systemStateStore";

  let action: Prompt = {
    _id: { $oid: "" },
    prompt: "",
    system: "",
  };

  async function localAddNode() {
    $systemStateStore.websocket.send(JSON.stringify({ create_action: action }));
  }
</script>

<form on:submit|preventDefault={localAddNode}>
  <label for="prompt">Prompt</label>
  <input id="prompt" bind:value={action.prompt} type="text" required />

  <label for="name">Name</label>
  <input id="name" bind:value={action.name} type="text" required />

  <label for="system">System</label>
  <input id="system" bind:value={action.system} type="text" required />

  <button type="submit">Submit</button>
</form>
