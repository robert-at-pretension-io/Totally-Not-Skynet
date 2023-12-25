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
    AtomicExecutionLog,
    VerbTypes,
  } from "../../generated/system_types";
  import { onMount } from "svelte";

  import AtomicExecution from "./AtomicExecution.svelte";

  import { v4 as uuidv4 } from "uuid";
  import { sendEnvelope } from "helper_functions/websocket";
  import { websocketStore } from "stores/websocketStore";

  let selected_process: Node | undefined = undefined;

  let initial_variables = new Map<string, string>();
  let input_variables = [];

  let allVariablesDefined = false;
  let node_options = new Array<Node>();
  let nodes = new Array<Node>();
  let latest_execution = new Execution();
  let logs = new Array<AtomicExecutionLog>();

  onMount(() => {
    nodes = $systemStateStore.local_nodes;
    node_options = nodes.filter((node) => node.node_type === NodeTypes.PROCESS);

    console.log("ExecuteNode mounted");
    if ($systemStateStore.selected_process !== undefined) {
      selected_process = $systemStateStore.selected_process;
      input_variables = selected_process.input_variables;
    }

    if (
      $systemStateStore.execution_results !== undefined &&
      $systemStateStore.execution_results.length > 0
    ) {
      let last_index = $systemStateStore.execution_results.length - 1;
      latest_execution = $systemStateStore.execution_results[last_index];
      prompt_history = latest_execution.atomic_history;
      // reorder_prompt_history();
    }
  });

  $: {
    if ($systemStateStore.selected_process !== undefined) {
      selected_process = $systemStateStore.selected_process;
      input_variables = selected_process.input_variables;
      // This should reset the variable map each time the process changes (regardless of if the process has been selected locally or not.)
      initial_variables = new Map<string, string>();
    }

    if (
      $systemStateStore.execution_results !== undefined &&
      $systemStateStore.execution_results.length > 0
    ) {
      let last_index = $systemStateStore.execution_results.length - 1;
      latest_execution = $systemStateStore.execution_results[last_index];
      logs = latest_execution.atomic_history;

      // reorder_prompt_history();
    }
  }

  $: {
    nodes = $systemStateStore.local_nodes;
    node_options = nodes.filter((node) => node.node_type === NodeTypes.PROCESS);
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

  function onDropdownChange() {
    if (selected_process == "") {
      return;
    }

    $systemStateStore.selected_process = selected_process;
  }
</script>

<select bind:value={selected_process} on:change={() => onDropdownChange()}>
  <option value="">Select a process</option>
  {#each node_options as node}
    <option value={node}>{node.node_info.name}</option>
  {/each}
</select>

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

<AtomicExecution {logs} />

<style>
  .process-dropdown {
    margin-bottom: 1rem;
  }

  .input-section label {
    display: block;
    margin-bottom: 0.5rem;
  }

  .prompt-history {
    margin-top: 1rem;
    border-top: 1px solid #ccc;
    padding-top: 1rem;
  }

  .prompt-history p {
    margin-bottom: 0.5rem;
  }
</style>
