<script>
    import { aiSystemStore } from "stores/aiSystemStore";
    let selectedAction;
    let selectedProcess;
  
    // Subscribe to the graphStore to get the latest values
    let actions = [];
    let processes = [];

    $: {
      actions = $aiSystemStore.actions;
      processes = $aiSystemStore.processes;
    }

  </script>
  
  <!-- Dropdown menu for actions -->
  <select bind:value={selectedAction}>
    <option value="">Select an action</option>
    {#each actions as action}
      <option value={action.name}>{action.name}</option>
    {/each}
  </select>
  
  <!-- Dropdown menu for processes -->
  <select bind:value={selectedProcess}>
    <option value="">Select a process</option>
    {#each processes as process}
      <option value={process.name}>{process.name}</option>
    {/each}
  </select>
