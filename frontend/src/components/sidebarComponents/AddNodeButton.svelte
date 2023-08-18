<script lang="ts">
  import { Prompt } from "generated/system_types_pb.js";

  import systemStateStore from "stores/systemStateStore";

  let prompt: Prompt = new Prompt();

  async function localAddNode() {
    $systemStateStore.websocket.send(JSON.stringify({ create_action: action }));
  }
</script>

<form on:submit|preventDefault={localAddNode}>
  <label for="prompt">Prompt</label>
  <input id="prompt" bind:value={prompt.getPrompt()} type="text" required />

  <label for="system">System</label>
  <input id="system" bind:value={prompt.getSystem()} type="text" required />

  <button type="submit">Submit</button>
</form>
