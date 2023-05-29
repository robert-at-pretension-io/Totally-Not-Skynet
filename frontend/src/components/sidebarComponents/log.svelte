<script lang="ts">
  import { onDestroy } from "svelte";
  import systemStateStore from "stores/systemStateStore";
  import { SystemState } from "system_types";
  import { getParentOutputVariables, setLocalExecutionVariable, addVariablesToPrompt, sendPrompt } from "helper_functions/graph";
  import type { Prompt } from "../../system_types";
  
  let responses : Map<string, string> = new Map();
  let prompts : Map<string, string> = new Map();
  let topological_order : string[] = [];
  let current_node : string | null = "";
  let local_variables : Map<string, string> = new Map();
  
  const unsubscribe = systemStateStore.subscribe((value : SystemState) => {
    responses = value.executionContext.responses;
    prompts = value.executionContext.prompts;
    topological_order = value.executionContext.topological_order;
    current_node = value.executionContext.current_node;
    local_variables = value.executionContext.local_variables;
  });

  // do something when the current_node changes
  $: if (current_node != null) {
    // if the current_node is not the first in the topological order
    if (topological_order.indexOf(current_node) > 0) {
      // get the previous node
      let previous_node = topological_order[topological_order.indexOf(current_node) - 1];
      // if the previous node has a response
      if (responses.has(previous_node)) {
        console.log("response has previous_node: ", previous_node);
        // get the prompt for the current node

        // and if the current node has a prompt
        if (prompts.has(current_node) && !responses.has(current_node)) {
          // send the response to the backend
          //$systemStateStore.websocket.send(JSON.stringify({ response: responses.get(previous_node) }));
          console.log("sending response here: ", responses.get(previous_node));
          getParentOutputVariables(current_node).then((output_variables) => {
            console.log("output_variables: ", output_variables);
            output_variables.forEach((variable) => {
              if (!local_variables.has(variable)) {
                setLocalExecutionVariable(variable, responses.get(previous_node) || "").then((value) => {
                  console.log("setLocalExecutionVariable: ", value);
                  // now we are ready to send the new prompt
                  // first we need to replace the variables in the prompt
                  if (current_node != null) {

                    let prompt = prompts.get(current_node);
                    if (prompt != undefined){
                      let this_prompt_text = addVariablesToPrompt(prompt, value);
                      
                      // send the prompt to the backend

                      let prompt_object : Prompt = {
                        prompt_text: this_prompt_text,
                        action_id: current_node,
                        system: "test",
                      };

                      sendPrompt(prompt_object).then((value) => {
                        console.log("sent prompt: ", value);
                      });

                    }
                
                  }

                });
                
              }
            });
          });
        }
        // send the response to the backend
      }
    }
    console.log("current_node: ", current_node);
  }
  
  onDestroy(() => {
    unsubscribe();
  });
  </script>
  
  {#each topological_order as node}
      <h2>{node}</h2>
      {#if prompts.has(node)}
          <div class="prompt">
              <h3>{node}</h3>
              <p>{prompts.get(node)}</p>
          </div>
      {/if}
      {#if responses.has(node)}
      <div class="response">
          <h3>{node}</h3>
          <p>{responses.get(node)}</p>
      </div>
  {/if}
  {/each}
