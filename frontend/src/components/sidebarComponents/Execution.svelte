<script lang="ts">
    import systemStateStore from "stores/systemStateStore";
    import { graphStore } from "stores/graphStore";
    import { aiSystemStore } from "stores/aiSystemStore";
    import { Action, Process, Prompt } from "system_types";
    import { sendPrompt } from "helper_functions/graph";

    let selectedProcess: Process | null = null;
    let globalVariables = new Map<string, string>();
    let possibleActions: Action[] | null = null;
    let topological_order: string[] = [];
    let already_made_first_prompt = false;

    $: {
      selectedProcess = $systemStateStore.selectedProcess;
      globalVariables = $graphStore.global_variables;
      possibleActions = $aiSystemStore.actions;

      if (
        selectedProcess &&
            possibleActions &&
            topological_order.length === 0
      ) {
        topological_order = selectedProcess.topological_order;
      }

      if (topological_order.length > 0 && !already_made_first_prompt) {
        console.log("topological_order: ", topological_order);

        let first_action_id = topological_order[0];
        // then we form the prompt for the first action
        if (possibleActions != null) {
          let first_action = possibleActions.find(
            (action: Action) => action._id.$oid === first_action_id
          );
          if (first_action != null) {
            already_made_first_prompt = true;
            console.log("first_action: ", first_action);
            console.log("first_action.prompt: ", first_action.prompt);
            // populate the prompt with the global variables if needed
            let prompt = first_action.prompt;
            // if the first_action has input variables
            if (first_action.input_variables.length > 0) {
              let input_variables = first_action.input_variables;
              // then we need to replace the input variables with the global variables
              for (let i = 0; i < input_variables.length; i++) {
                let input_variable = input_variables[i];
                let global_variable =
                                globalVariables.get(input_variable);
                // if the global variable exists
                if (global_variable != null) {
                  prompt = prompt.replace(
                    input_variable,
                    global_variable
                  );
                } else {
                  console.log("global variable does not exist");
                  alert("global variable does not exist");
                }
              }
            }
            // send the prompt to the backend
            let send_prompt : Prompt = {
              prompt_text: prompt,
              system: first_action.system,
              action_id: first_action._id.$oid,
            };
            sendPrompt(send_prompt);
          }
        }
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
