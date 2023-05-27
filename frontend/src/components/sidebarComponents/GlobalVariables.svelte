<script lang="ts">
    import { graphStore } from "stores/graphStore";
  
    let name = "";
    let value = "";
    let editName = "";
    let editValue = "";
  
    async function addVariable() {
      if (name && value) {
        let currentGlobalVariables;
        graphStore.update(store => {
          currentGlobalVariables = new Map(store.global_variables);
          currentGlobalVariables.set(name, value);
          return {
            ...store,
            global_variables: currentGlobalVariables,
          };
        });
      }
      // reset form fields
      name = "";
      value = "";
    }
  
    async function editVariable() {
      if (editName && editValue) {
        let currentGlobalVariables;
        graphStore.update(store => {
          currentGlobalVariables = new Map(store.global_variables);
          if (currentGlobalVariables.has(editName)) {
            currentGlobalVariables.set(editName, editValue);
            return {
              ...store,
              global_variables: currentGlobalVariables,
            };
          }
        });
      }
      // reset form fields
      editName = "";
      editValue = "";
    }
  
    function selectVariable(name, value) {
      editName = name;
      editValue = value;
    }
  </script>
  
  <div class="sidebar">
    <h2>Global Variables</h2>
  
    <form on:submit|preventDefault={addVariable}>
      <label>
        Name:
        <input bind:value={name} type="text" required />
      </label>
  
      <label>
        Value:
        <input bind:value={value} type="text" required />
      </label>
  
      <button type="submit">Add Variable</button>
    </form>
  
    {#if editName}
  <form on:submit|preventDefault={editVariable}>
        <label>
          Edit Name:
          <input bind:value={editName} type="text" required />
        </label>
  
        <label>
          Edit Value:
          <input bind:value={editValue} type="text" required />
        </label>
  
        <button type="submit">Edit Variable</button>
      </form>
    {/if}
  
    <h3>Added Variables</h3>
    <ul>
        {#if $graphStore.global_variables.size === 0}
          <li>No global variables added</li>
        {:else}
          {#each Array.from($graphStore.global_variables.entries()) as [name, value]}
            <li on:click={() => selectVariable(name, value)}>{name}: {value}</li>
          {/each}
        {/if}
      </ul>
  </div>
