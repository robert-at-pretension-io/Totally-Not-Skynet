<script lang="ts">
  import { websocketStore } from "stores/websocketStore";
  export let prompt: Prompt;
  import {
    CrudBundle,
    Prompt,
    VerbTypeNames,
    Node,
    GraphNodeInfo,
  } from "../../../generated/system_types_pb";
  import { sendWebsocketMessage } from "helper_functions/websocket";

  let system_text = "";
  let prompt_text = "";
  let description = "";
  let name = "";

  async function submitPrompt() {
    prompt.setPrompt(prompt_text);
    prompt.setSystem(system_text);

    // create and send crud bundle:

    let crud_bundle = new CrudBundle();

    let verb = VerbTypeNames.POST;

    let node = new Node();

    node.setPrompt(prompt);

    let node_info = new GraphNodeInfo();
    node_info.setName(name);
    node.setDescription(description);
    node.setNodeInfo(node_info);
    crud_bundle.setNode(node);
    crud_bundle.setVerb(verb);

    let websocket = $websocketStore.websocket as WebSocket;

    await sendWebsocketMessage(crud_bundle, websocket);
  }
</script>

<form on:submit|preventDefault={submitPrompt}>
  <div>
    <div>
      <input bind:value={name} placeholder="Name" />

      <input bind:value={description} placeholder="Description" />
      <!--   
      <div>
        <h4>Input Variables</h4>
        {#each inputVariablesList as _inputVar, index}
          <input
            bind:value={inputVariablesList[index]}
            placeholder={`Input variable ${index + 1}`}
          />
          <button on:click={() => inputVariablesList.splice(index, 1)}>x</button>
        {/each}
        <button on:click={() => inputVariablesList.push("")}
          >Add Input Variable</button
        >
      </div>
  
      <div>
        <h4>Output Variables</h4>
        {#each outputVariablesList as _outputVar, index}
          <input
            bind:value={outputVariablesList[index]}
            placeholder={`Output variable ${index + 1}`}
          />
          <button on:click={() => outputVariablesList.splice(index, 1)}>x</button>
        {/each}
        <button on:click={() => outputVariablesList.push("")}
          >Add Output Variable</button
        >
      </div> -->

      <label for="prompt" class="required-label">Prompt</label>
      <input
        id="prompt"
        bind:value={prompt_text}
        type="text"
        required
        class="required-input"
      />

      <label for="system" class="required-label">System</label>
      <input
        id="system"
        bind:value={system_text}
        type="text"
        required
        class="required-input"
      />
      <button type="submit">Submit</button>
    </div>
  </div>
</form>

<style>
  .required-input:invalid {
    border: 2px solid red;
  }

  .required-label::after {
    content: " *";
    color: red;
  }
</style>
