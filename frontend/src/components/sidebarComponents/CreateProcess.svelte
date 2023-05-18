<script lang="ts">
  import { onMount } from "svelte";
  import { aiSystemStore } from "stores/aiSystemStore";
  import type { Action } from "system_types";
  import { addNode } from "../../helper_functions/graph";
  
  let actions: Action[] = [];
  let selectedActions: Action[] = [];

  onMount(async () => {
    aiSystemStore.subscribe((value) => {

      actions = value.actions;

      // loop through the actions and print their values:
      for (let i = 0; i < actions.length; i++) {
        console.log("Action " + i + ": " + JSON.stringify(actions[i]));
      }
    });
  });

  function localAddNodes() {
    // for each of the selected actions, add a node to the graph
    selectedActions.forEach(action => {
      addNode(action._id.$oid);
    });
  }

  function toggleSelect(action: Action) {
    console.log("toggleSelect called on action: ", action);
    const index = selectedActions.findIndex(a => a._id.$oid === action._id.$oid);
    if (index !== -1) {
      // action is currently selected, remove it
      selectedActions = selectedActions.filter(a => a._id.$oid !== action._id.$oid);
    } else {
      // action is not currently selected, add it
      selectedActions = [...selectedActions, action];
    }
    console.log("selectedActions after toggleSelect:", selectedActions);
  }

  function isSelected(action: Action) {
    console.log("isSelected called on action: ", action);
    let is_selected = selectedActions.some(a => a._id.$oid === action._id.$oid);
    console.log("selectedActions during isSelected:", selectedActions);
    console.log("isSelected returning: ", is_selected);
    return is_selected;
  }
</script>

<div class="sidebar">
  <h2>Actions</h2>
  <ul>
    {#each actions as action (action._id)}
      <li>
        <button class:selected={isSelected(action)} type="button" on:click={() => toggleSelect(action)}>{action.name}</button>
      </li>
    {/each}
  </ul>
  <button on:click={localAddNodes}>Add Node(s)</button>
</div>

<style>

  button.selected {
    /* style your selected actions here */
    background-color: #3079d8;
    color: royalblue;
  }

  /* Additional styles for the button if needed */
  button {
    border: solid;
    background-color: transparent;
    padding: 0;
    cursor: pointer;
  }
</style>
