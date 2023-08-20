<script lang="ts">
  import CommandComponent from "./subComponents/CommandComponent.svelte";
  import ConditionalComponent from "./subComponents/ConditionalComponent.svelte";
  import ProcessComponent from "./subComponents/ProcessComponent.svelte";
  import PromptComponent from "./subComponents/PromptComponent.svelte";
  import {
    NodeTypeNames,
    Prompt,
    Process,
    Conditional,
    Command,
  } from "generated/system_types_pb.js";

  let id = "";
  let name = "";
  let typeName: number = NodeTypeNames.PROMPT;
  let description = "";
  let inputVariablesList: string[] = [];
  let outputVariablesList: string[] = [];

  let prompt = new Prompt();
  let process = new Process();
  let conditional = new Conditional();
  let command = new Command();

  let key_list = Object.keys(NodeTypeNames);
  let num_array = Array.from({ length: key_list.length }, (_, i) => i);

  // Dummy function to collect the Node object (replace with actual logic)
  function getNodeObject() {
    return {
      id,
      name,
      typeName,
      description,
      inputVariablesList,
      outputVariablesList,
      // Add logic for prompt, process, conditional, command
    };
  }
</script>

<div>
  <div>
    <input bind:value={name} placeholder="Name" />
    <select bind:value={typeName}>
      {#each num_array as array_index}
        <option value={array_index}>{key_list[array_index]}</option>
      {/each}
    </select>
    <input bind:value={description} placeholder="Description" />

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

    <!-- Output Variables -->
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
    </div>

    {#if typeName === NodeTypeNames.PROMPT}
      <PromptComponent bind:prompt />
    {/if}
    {#if typeName === NodeTypeNames.PROCESS}
      <ProcessComponent bind:process />
    {/if}
    {#if typeName === NodeTypeNames.CONDITIONAL}
      <ConditionalComponent bind:conditional />
    {/if}
    {#if typeName === NodeTypeNames.COMMAND}
      <CommandComponent bind:command />
    {/if}
    <button on:click={() => console.log(getNodeObject())}>Save</button>
  </div>
</div>
