<script lang="ts">
  import CommandComponent from "./newNodeSubComponents/CommandComponent.svelte";
  import ConditionalComponent from "./newNodeSubComponents/ConditionalComponent.svelte";
  import ProcessComponent from "./newNodeSubComponents/ProcessComponent.svelte";
  import PromptComponent from "./newNodeSubComponents/PromptComponent.svelte";
  import LoopComponent from "./newNodeSubComponents/LoopComponent.svelte";

  import {
    // NodeTypes,
    Prompt,
    Process,
    Conditional,
    Command,
    NodeTypes,
  } from "../../generated/system_types";
  import CodeComponent from "./newNodeSubComponents/CodeComponent.svelte";

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
  // let key_list = ["PROMPT", "PROCESS", "LOOP", "CONDITIONAL"];
  // the key_list will be the keys of the NodeTypes enum
  let key_list = Object.keys(NodeTypes).filter((key) => isNaN(Number(key)));

  let num_array = Array.from({ length: key_list.length }, (_, i) => i);

  $: {
    console.log("Typename is now: ", typeName);
  }
</script>

<select bind:value={typeName}>
  {#each num_array as array_index}
    <option value={array_index}>{key_list[array_index]}</option>
  {/each}
</select>

<p>Selected type: {key_list[typeName]}</p>

{#if typeName === NodeTypes.PROMPT}
  <PromptComponent />
{/if}
{#if typeName === NodeTypes.PROCESS}
  <ProcessComponent />
{/if}
{#if typeName === NodeTypes.LOOP}
  <LoopComponent />
{/if}
<!-- {#if typeName === NodeTypes.CODE}
  <CodeComponent />
{/if} -->
{#if typeName === NodeTypes.CONDITIONAL}
  <ConditionalComponent />
{/if}

{#if typeName === NodeTypes.COMMAND}
  <CommandComponent />
{/if}
