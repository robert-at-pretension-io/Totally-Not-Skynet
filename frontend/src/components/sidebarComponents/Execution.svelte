<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import { graphStore } from "stores/graphStore";
  import { aiSystemStore } from "stores/aiSystemStore";
  import type { Action, Process, Prompt } from "./system_types";
  import {
    getInputVariablesByNodeId,
    getNodeName,
    sendPrompt,
  } from "helper_functions/graph";
  import { isProcess, newAction } from "helper_functions/type_checker";
  import { addGlobalVariable } from "helper_functions/graph";

  let selectedProcess: Process | null = null;
  let globalVariables = new Map<string, string>();
  let possibleActions: Action[] | null = null;
  let topological_order: string[] = [];
  let topological_order_names: (string | undefined)[] = [];
  let already_made_first_prompt = false;
  let needed_variables: string[] = [];
  let ready_to_make_first_prompt = false;

  function update_local_variables() {
    selectedProcess = $systemStateStore.selectedProcess;
    console.log("Updated selectedProcess: ", selectedProcess);
    topological_order = selectedProcess.topological_order;

    globalVariables = $graphStore.global_variables;
    console.log("Updated globalVariables: ", [...globalVariables]);

    possibleActions = $aiSystemStore.actions;
    console.log("Updated possibleActions: ", possibleActions);
  }

  async function processSelectedProcessAndActions(
    selectedProcess: Process | null,
    possibleActions: Action[] | null,
    topological_order: string[],
    topological_order_names: (string | undefined)[],
    globalVariables: Map<string, string>
  ): Promise<{
    topological_order_names: (string | undefined)[];
    ready_to_make_first_prompt: boolean;
    needed_variables: string[];
  }> {
    let ready_to_make_first_prompt = false;
    let needed_variables: string[] = [];

    if (selectedProcess && possibleActions) {
      if (topological_order.length > 0 && topological_order_names.length == 0) {
        let promiseArray = topological_order.map(getNodeName);

        let local_topological_order_names = await Promise.all(promiseArray);

        // Here, topological_order_names is an array with all the resolved values.
        // remove all of the undefined values:
        local_topological_order_names = local_topological_order_names.filter(
          (value) => value != undefined
        );

        topological_order_names = local_topological_order_names;
      }
      console.log("Updated topological_order: ", topological_order);

      let input_vars = await getInputVariablesByNodeId(topological_order[0]);

      if (input_vars != null) {
        needed_variables = input_vars.filter(
          (input_var) => !globalVariables.has(input_var)
        );

        if (needed_variables.length == 0) {
          ready_to_make_first_prompt = true;
        }
      }
    }

    return {
      topological_order_names,
      ready_to_make_first_prompt,
      needed_variables,
    };
  }

  async function createFirstPrompt(
    ready_to_make_first_prompt: boolean,
    already_made_first_prompt: boolean,
    topological_order: string[],
    possibleActions: Action[] | null,
    globalVariables: Map<string, string>
  ): Promise<{ already_made_first_prompt: boolean }> {
    if (ready_to_make_first_prompt && !already_made_first_prompt) {
      console.log("Creating first prompt...");

      let first_action_id = topological_order[0];
      if (possibleActions != null) {
        let first_action = possibleActions.find(
          (action: Action) => action._id.$oid === first_action_id
        );
        if (first_action != null) {
          already_made_first_prompt = true;
          console.log("Found first action: ", first_action);
          console.log("First action prompt: ", first_action.prompt);

          let prompt = first_action.prompt;
          if (first_action.input_variables.length > 0) {
            let input_variables = first_action.input_variables;
            console.log("First action input variables: ", input_variables);

            for (let i = 0; i < input_variables.length; i++) {
              let input_variable = input_variables[i];
              let global_variable = globalVariables.get(input_variable);
              if (global_variable != null) {
                prompt = prompt.replace(input_variable, global_variable);
                console.log(
                  `Replaced ${input_variable} with ${global_variable} in prompt.`
                );
                let send_prompt: Prompt = {
                  prompt_text: prompt,
                  system: first_action.system,
                  action_id: first_action._id.$oid,
                };
                console.log("Sending prompt: ", send_prompt);
                await sendPrompt(send_prompt);
              } else {
                console.log(
                  "Global variable does not exist for input variable: ",
                  input_variable
                );
                alert("global variable does not exist");
              }
            }
          }
        } else {
          console.log("No action found for first action ID: ", first_action_id);
        }
      } else {
        console.log("Possible actions is null.");
      }
    } else {
      console.log("Conditions for creating first prompt not met.");
    }

    return { already_made_first_prompt };
  }

  let processes: Process[] = [];

  processes = $aiSystemStore.processes;

  let newValues = {};

  function handleInputChange(variableName, event) {
    newValues[variableName] = event.target.value;
  }

  async function handleFormSubmit(variableName, event) {
    event.preventDefault();
    if (newValues[variableName]) {
      addGlobalVariable(variableName, newValues[variableName]);
    }

    let return_val = await processSelectedProcessAndActions(
      selectedProcess,
      possibleActions,
      topological_order,
      topological_order_names,
      globalVariables
    );

    topological_order_names = return_val.topological_order_names;
    ready_to_make_first_prompt = return_val.ready_to_make_first_prompt;
    needed_variables = return_val.needed_variables;
    update_local_variables();
  }
  // Function to handle dropdown change events
  async function onDropdownChange() {
    // console.log("onDropdownChange called: ", type, " selectedAction: ", selectedAction, " selectedProcess: ", selectedProcess);

    if (selectedProcess) {
      let this_process = $aiSystemStore.processes.find(
        (obj) => obj.name === selectedProcess
      );

      if (isProcess(this_process)) {
        $systemStateStore.selectedProcess = this_process;
        $systemStateStore.selectedAction = newAction();
      }

      let return_val = await processSelectedProcessAndActions(
        selectedProcess,
        possibleActions,
        topological_order,
        topological_order_names,
        globalVariables
      );

      topological_order_names = return_val.topological_order_names;
      ready_to_make_first_prompt = return_val.ready_to_make_first_prompt;
      needed_variables = return_val.needed_variables;
      update_local_variables();
    }
  }
</script>

{#if selectedProcess}
  <div>
    <h2>Description: {selectedProcess.description}</h2>
    <h2>Topological Order: {topological_order_names.join(", ")}</h2>

    <h2>Global Variables</h2>
    {#each [...globalVariables] as [key, value]}
      <p>{key}: {value}</p>
    {/each}

    <!-- check if the needed variables array has length 0:-->
    {#if needed_variables.length != 0}
      <h2>Needed Variables</h2>
      {#each needed_variables as needed_var (needed_var)}
        <form
          on:submit={async (event) => await handleFormSubmit(needed_var, event)}
        >
          <label for={needed_var}>{needed_var}</label>
          <input
            id={needed_var}
            on:input={(event) => handleInputChange(needed_var, event)}
          />
          <button type="submit">Update</button>
        </form>
      {/each}
    {/if}
  </div>
{:else}
  <h1>Selected a Process:</h1>
  <select
    bind:value={selectedProcess}
    on:change={async () => await onDropdownChange()}
  >
    <option value="">Select a process</option>
    {#each processes as process}
      <option value={process.name}>{process.name}</option>
    {/each}
  </select>
{/if}
