<script>
  import { aiSystemStore } from "stores/aiSystemStore";
  import systemStateStore from "stores/systemStateStore";
  import JsonEditor from "./JsonEditor.svelte";

  let selectedAction = "";
  let selectedProcess = "";

  // Subscribe to the graphStore to get the latest values
  let actions = [];
  let processes = [];

  $: {
    actions = $aiSystemStore.actions;
    processes = $aiSystemStore.processes;
  }

  // Function to handle dropdown change events
  function onDropdownChange(type) {
    // console.log("onDropdownChange called: ", type, " selectedAction: ", selectedAction, " selectedProcess: ", selectedProcess);
    if (type === "action") {
      selectedProcess = "";
    } else {
      selectedAction = "";
    }

    if (selectedAction) {
      // Set the selected action in the systemStateStore
      // it should get the action from the aiSystemStore
      // with the name selectedAction
      let this_action = $aiSystemStore.actions.find(
        (obj) => obj.name === selectedAction
      );
      $systemStateStore.selectedAction = this_action;
      $systemStateStore.selectedProcess = null;
    }
    if (selectedProcess) {
      // Set the selected process in the systemStateStore
      // it should get the process from the aiSystemStore
      // with the name selectedProcess
      let this_process = $aiSystemStore.processes.find(
        (obj) => obj.name === selectedProcess
      );

      // console.log("this_process: ", this_process);

      $systemStateStore.selectedProcess = this_process;
      $systemStateStore.selectedAction = null;
    }
  }
</script>

<!-- Dropdown menu for actions -->
<select
  bind:value={selectedAction}
  on:change={() => onDropdownChange("action")}
>
  <option value="">Select an action</option>
  {#each actions as action}
    <option value={action.name}>{action.name}</option>
  {/each}
</select>

<!-- Dropdown menu for processes -->
<select
  bind:value={selectedProcess}
  on:change={() => onDropdownChange("process")}
>
  <option value="">Select a process</option>
  {#each processes as process}
    <option value={process.name}>{process.name}</option>
  {/each}
</select>

<JsonEditor />
