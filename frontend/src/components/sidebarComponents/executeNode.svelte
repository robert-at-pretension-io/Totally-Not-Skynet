<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import {
    Body,
    Execution,
    GraphNodeInfo,
    Letter,
    Node,
    NodeTypes,
    Process,
    VerbTypes,
  } from "../../generated/system_types";
  import { onMount } from "svelte";

  import { v4 as uuidv4 } from "uuid";
  import { sendEnvelope } from "helper_functions/websocket";
  import { websocketStore } from "stores/websocketStore";

  let selected_process: Node | undefined = undefined;

  let initial_variables = new Map<string, string>();
  let input_variables = [];

  let allVariablesDefined = false;

  onMount(() => {
    console.log("ExecuteNode mounted");
    if ($systemStateStore.selected_process !== undefined) {
      selected_process = $systemStateStore.selected_process;
      input_variables = selected_process.input_variables;
    }
  });

  $: {
    if ($systemStateStore.selected_process !== undefined) {
      selected_process = $systemStateStore.selected_process;
      input_variables = selected_process.input_variables;
    }
  }

  $: allVariablesDefined = Array.from(initial_variables.values()).every(
    (value) => value.trim() !== ""
  );

  function updateInitialVariables(key: string, value: string) {
    initial_variables.set(key, value);
  }

  async function handleSubmit() {
    let letter = new Letter();

    let body = new Body();

    let execution = new Execution();

    execution.process = selected_process.node_content.process;
    execution.current_variable_definitions = initial_variables;
    execution.execution_id = uuidv4();
    execution.current_node =
      selected_process.node_content.process.topological_order[0];

    body.execution_details = execution;

    letter.body = body;

    letter.verb = VerbTypes.Execute;

    let websocket = $websocketStore.websocket as WebSocket;

    sendEnvelope(websocket, [letter]);
  }
</script>

<form on:submit|preventDefault={handleSubmit}>
  {#each input_variables as variable}
    <label>
      {variable}:
      <input
        type="text"
        on:input={(e) => updateInitialVariables(variable, e.target.value)}
      />
    </label>
  {/each}
  {#if allVariablesDefined}
    <button type="submit">Submit</button>
  {/if}
</form>
<!--

  For each of the input variables in the selected process, create a text input box, have each of the values of the text boxes update the initial_variables map, and then have the submit button send the initial_variables map to the backend to be executed.

-->

<!-- 
{#each ordered_nodes as node (node.node_info.id)}
  <h2><strong>{key_list[node.type_name]}</strong> : {node.node_info.name}</h2>
  {node.node_info.description}
  <br />
  <p><strong>Input Variables:</strong> {node.input_variables.join(", ")}</p>
  <p><strong>Output Variables:</strong> {node.output_variables.join(", ")}</p>
  <p><strong>Node Content:</strong> {node.node_content}</p>

  <hr />
{/each} -->
