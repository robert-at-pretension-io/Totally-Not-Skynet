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
  } from "../../generated/system_types";

  let typeName: number = NodeTypeNames.PROMPT;

  // let inputVariablesList: string[] = [];
  // let outputVariablesList: string[] = [];

  let prompt = new Prompt();
  let process = new Process();
  let conditional = new Conditional();
  let command = new Command();

  let key_list = Object.keys(NodeTypeNames).filter((key) => isNaN(Number(key)));

  console.log("keylist is:", key_list);

  let num_array = Array.from({ length: key_list.length }, (_, i) => i);
</script>

<select bind:value={typeName}>
  {#each num_array as array_index}
    <option value={array_index}>{key_list[array_index]}</option>
  {/each}
</select>

<p>Selected type: {key_list[typeName]}</p>

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
