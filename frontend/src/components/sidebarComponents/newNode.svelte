<script lang="ts">
  import CommandComponent from "./newNodeSubComponents/CommandComponent.svelte";
  import ConditionalComponent from "./newNodeSubComponents/ConditionalComponent.svelte";
  import ProcessComponent from "./newNodeSubComponents/ProcessComponent.svelte";
  import PromptComponent from "./newNodeSubComponents/PromptComponent.svelte";
  import {
    // NodeTypes,
    Prompt,
    Process,
    Conditional,
    Command,
    NodeTypes,
  } from "../../generated/system_types";

  let typeName: number = NodeTypes.PROMPT;

  // let inputVariablesList: string[] = [];
  // let outputVariablesList: string[] = [];

  let prompt = new Prompt();
  let process = new Process();
  let conditional = new Conditional();
  let command = new Command();

  

  // This is the dynamic key-list for the dropdown menu, use it when all of the types are implemented
  // let key_list = Object.keys(NodeTypes).filter((key) => isNaN(Number(key)));

  // This is the static key-list for the dropdown menu
  let key_list = ["PROMPT", "PROCESS"];

  let num_array = Array.from({ length: key_list.length }, (_, i) => i);
</script>

<select bind:value={typeName}>
  {#each num_array as array_index}
    <option value={array_index}>{key_list[array_index]}</option>
  {/each}
</select>

<p>Selected type: {key_list[typeName]}</p>

{#if typeName === NodeTypes.PROMPT}
  <PromptComponent bind:prompt />
{/if}
{#if typeName === NodeTypes.PROCESS}
  <ProcessComponent bind:process />
{/if}
<!-- {#if typeName === NodeTypes.CONDITIONAL}
  <ConditionalComponent bind:conditional />
{/if}
{#if typeName === NodeTypes.COMMAND}
  <CommandComponent bind:command />
{/if} -->
