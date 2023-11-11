<script lang="ts">
  import { websocketStore } from "stores/websocketStore";
  import {
    Prompt,
    VerbTypes,
    Node,
    GraphNodeInfo,
    Envelope,
    Letter,
    Body,
    NodeTypes,
    VariableDefinition,
  } from "../../../generated/system_types";
  import { v4 as uuidv4 } from "uuid";
  import { sendEnvelope } from "helper_functions/websocket";

  let system_text = "";
  let prompt_text = "";
  let description = "";
  let name = "";
  let input_variables: VariableDefinition[] = [];
  let output_variables: VariableDefinition[] = [];
  let new_input_variable: VariableDefinition;
  let new_output_variable: VariableDefinition;
  export let prompt: Prompt;

  function submitPrompt() {
    prompt.prompt = prompt_text;
    prompt.system = system_text;

    let node = new Node();

    if (new_input_variable != undefined) {
      input_variables.push(new_input_variable);
    }

    if (new_output_variable != "") {
      output_variables = [...output_variables, new_output_variable];
    }

    let input_variables = new VariableDefinition();

    // input_variables.

    node.input_variables = input_variables;
    node.output_variables = output_variables;

    let node_info = new GraphNodeInfo();
    node_info.name = name;
    node_info.id = new uuidv4();
    node_info.description = description;
    node.node_info = node_info;
    node.node_content.prompt = prompt;
    node.node_type = NodeTypes.PROMPT;

    let websocket = $websocketStore.websocket as WebSocket;

    let body = new Body();

    body.node = node;

    let letter = new Letter();

    letter.body = body;
    letter.verb = VerbTypes.Create;

    sendEnvelope(websocket, [letter]);

    reset_component();
  }

  function reset_component() {
    system_text = "";
    prompt_text = "";
    description = "";
    name = "";
    input_variables = [];
    output_variables = [];
    new_input_variable = "";
    new_output_variable = "";
  }
</script>

<form on:submit|preventDefault={submitPrompt}>
  <div>
    <div>
      <input bind:value={name} placeholder="Name" />

      <input bind:value={description} placeholder="Description" />

      <div>
        <h4>Input Variables</h4>
        {#each input_variables as _inputVar, index}
          <button
            type="button"
            on:click={() => {
              input_variables.splice(index, 1);
              input_variables = input_variables;
            }}>`Remove input var: {input_variables[index]}`</button
          >
        {/each}
        <input
          bind:value={new_input_variable}
          placeholder="New Input Variable"
        />
        <button
          type="button"
          on:click={() => {
            input_variables = [...input_variables, new_input_variable];
            new_input_variable = "";
          }}>Add</button
        >
      </div>

      <div>
        <h4>Output Variables</h4>
        {#each output_variables as _outputVar, index}
          <button
            type="button"
            on:click={() => {
              output_variables.splice(index, 1);
              output_variables = output_variables;
            }}>`Remove output Var: {output_variables[index]}`</button
          >
        {/each}
        <input
          bind:value={new_output_variable}
          placeholder="New Output Variable"
        />
        <button
          type="button"
          on:click={() => {
            output_variables = [...output_variables, new_output_variable];
            new_output_variable = "";
          }}>Add</button
        >
      </div>

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
