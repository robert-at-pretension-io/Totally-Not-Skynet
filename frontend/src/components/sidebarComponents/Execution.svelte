<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import { graphStore } from "stores/graphStore";
  import { aiSystemStore } from "stores/aiSystemStore";
  import { Action, Process, Prompt } from "./system_types";
  import { sendPrompt } from "helper_functions/graph";

  let selectedProcess: Process | null = null;
  let globalVariables = new Map<string, string>();
  let possibleActions: Action[] | null = null;
  let topological_order: string[] = [];
  let already_made_first_prompt = false;

  console.log("Initial state: ", {
    selectedProcess,
    globalVariables,
    possibleActions,
    topological_order,
    already_made_first_prompt,
  });

  $: {
    selectedProcess = $systemStateStore.selectedProcess;
    console.log("Updated selectedProcess: ", selectedProcess);

    globalVariables = $graphStore.global_variables;
    console.log("Updated globalVariables: ", [...globalVariables]);

    possibleActions = $aiSystemStore.actions;
    console.log("Updated possibleActions: ", possibleActions);

    if (selectedProcess && possibleActions && topological_order.length === 0) {
      topological_order = selectedProcess.topological_order;
      console.log("Updated topological_order: ", topological_order);
    }

    if (topological_order.length > 0 && !already_made_first_prompt) {
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
              } else {
                console.log(
                  "Global variable does not exist for input variable: ",
                  input_variable
                );
                alert("global variable does not exist");
              }
            }
          }

          let send_prompt: Prompt = {
            prompt_text: prompt,
            system: first_action.system,
            action_id: first_action._id.$oid,
          };
          console.log("Sending prompt: ", send_prompt);
          sendPrompt(send_prompt);
        } else {
          console.log("No action found for first action ID: ", first_action_id);
        }
      } else {
        console.log("Possible actions is null.");
      }
    } else {
      console.log("Conditions for creating first prompt not met.");
    }
  }
</script>

{#if selectedProcess}
  <div>
    <h1>Selected Process: {selectedProcess.name}</h1>
    <h2>Description: {selectedProcess.description}</h2>
    <h2>Topological Order: {topological_order.join(", ")}</h2>

    <h2>Global Variables</h2>
    {#each [...globalVariables] as [key, value]}
      <p>{key}: {value}</p>
    {/each}

    <h2>Possible Actions</h2>
    {#each possibleActions as action (action._id.$oid)}
      <div>
        <h3>Action Name: {action.name}</h3>
        <p>Prompt: {action.prompt}</p>
        <p>Description: {action.description}</p>
      </div>
    {/each}
  </div>
{:else}
  <p>No process selected.</p>
{/if}
