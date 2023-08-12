<script lang="ts">
  import systemStateStore from "stores/systemStateStore";
  import { onMount } from "svelte";
  import { Node } from "system_types";

  let selected_node: Node | null;
  let has_selected_node = false;

  onMount(() => {
    console.log("on mount: Node Info");
    has_selected_node = $systemStateStore.selected_node != null;

    console.log("has_selected_node: ", has_selected_node);
  });

  $: {
    console.log("Node Info: selectedNode: ", $systemStateStore.selected_node);
    has_selected_node = $systemStateStore.selected_node != null;

    if (has_selected_node) {
      selected_node = $systemStateStore.selected_node;
    }
  }
</script>

<!-- Display different info depending on the type of the node -->

{#if has_selected_node && selected_node != null}
  Name: {selected_node.Node.name} <br />
  Description: {selected_node.Node.description}
  <!-- {#if selected_node.Node.type_name === "Prompt"}
    <p>Prompt:</p>
  {:else if selected_node.Node.type_name === "Process"}
    <p>Process:</p>
  {:else if selected_node.Node.type_name === "Conditional"}
    <p>Conditional</p>
  {:else if selected_node.Node.type_name === "Command"}
    <p>Command</p>
  {/if} -->
{:else}
  <p>No node selected</p>
{/if}
