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
    NodeContent,
    Conditional,
  } from "../../../generated/system_types";
  import { v4 as uuidv4 } from "uuid";
  import { sendEnvelope } from "helper_functions/websocket";

  let system_text = "";
  let prompt_text = "";
  let description = "";
  let name = "";
  let input_variables = [];
  let output_variables = [];
  let new_input_variable = "";
  let new_output_variable = "";

  function submitConditional() {
    // First, validate the Handlebars template
    if (!isValidHandlebarsTemplate(prompt_text)) {
      alert("Invalid Handlebars template.");
      return;
    }

    let conditional = new Conditional();

    let prompt = new Prompt();

    prompt.prompt = prompt_text;
    prompt.system = system_text;

    conditional.prompt = prompt;

    let node = new Node();

    if (new_input_variable != "") {
      input_variables = [...input_variables, new_input_variable];
    }

    // Extract and check variables from the template
    let variable_names = extractVariableNames(prompt_text);
    let allVariablesMatch = input_variables.every((inputVar) =>
      variable_names.includes(inputVar),
    );

    let noExtraVariables = variable_names.every((templateVar) =>
      input_variables.includes(templateVar),
    );

    // Combining both checks to ensure exact match
    let exactMatch = allVariablesMatch && noExtraVariables;

    if (new_output_variable != "") {
      output_variables = [...output_variables, new_output_variable];
    }

    if (exactMatch) {
      node.input_variables = input_variables;
      node.output_variables = output_variables;

      let node_content = new NodeContent();
      node_content.conditional = conditional;

      let node_info = new GraphNodeInfo();
      node_info.name = name;
      node_info.id = new uuidv4();
      node_info.description = description;

      node.node_info = node_info;
      node.node_content = node_content;
      node.node_type = NodeTypes.CONDITIONAL;

      let websocket = $websocketStore.websocket as WebSocket;

      let body = new Body();
      body.node = node;

      let letter = new Letter();
      letter.body = body;
      letter.verb = VerbTypes.Create;

      sendEnvelope(websocket, [letter]);

      reset_component();
    } else {
      alert(
        "All of the input variables do not have a definition within the template",
      );
    }
  }

  function isValidHandlebarsTemplate(template) {
    // A simple regex to check for basic Handlebars syntax
    const handlebarsRegex = /\{\{([^}]+)\}\}/g;
    return handlebarsRegex.test(template);
  }

  function extractVariableNames(template) {
    const handlebarsRegex = /\{\{([^}]+)\}\}/g;
    let match;
    const variables = new Set();

    while ((match = handlebarsRegex.exec(template)) !== null) {
      // The variable is in match[1], trim it to handle spaces
      variables.add(match[1].trim());
    }

    return Array.from(variables);
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

<form on:submit|preventDefault={submitConditional}>
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
