<script lang="ts">

  import type { Action } from "../../system_types";

  import websocketStore from "stores/websocketStore";

  let action: Action = {
    _id: { $oid: "" },
    prompt: "",
    name: "",
    system: "",
    input_variables: [],
    output_variables: [],
  };

  async function localAddNode() {

    // The id should come from the mongod database after the creation of the action

    // create the action in the database by sending a message to the backend

    $websocketStore.send(JSON.stringify({create_action: action}));

  }
</script>

<!-- <style>
  form {
    display: flex;
    flex-direction: column;
    gap: 1em;
  }
</style> -->

<form on:submit|preventDefault={localAddNode}>
  <label for="prompt">Prompt</label>
  <input id="prompt" bind:value={action.prompt} type="text" required>

  <label for="name">Name</label>
  <input id="name" bind:value={action.name} type="text" required>

  <label for="system">System</label>
  <input id="system" bind:value={action.system} type="text" required>

  <button type="submit">Submit</button>
</form>