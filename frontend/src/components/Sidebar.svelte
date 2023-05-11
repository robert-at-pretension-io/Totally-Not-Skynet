<script>
  import AddNodeButton from "./sidebarComponents/AddNodeButton.svelte";
  // import ModifyNodesOrEdges from "./sidebarComponents/ModifyNodesOrEdges.svelte";
  // import DeleteEdge from "./sidebarComponents/DeleteEdge.svelte";
  import SetOpenaiKey from "./sidebarComponents/SetOpenaiKey.svelte";
  import SendPrompt from "./sidebarComponents/SendPrompt.svelte";
  import InteractWithActionsAndProcesses from "./sidebarComponents/InteractWithActionsAndProcesses.svelte";
  import JsonEditor from "./sidebarComponents/JsonEditor.svelte";
  import CreateProcess from "./sidebarComponents/CreateProcess.svelte";

  import { blur, fade } from "svelte/transition";

  let sections = [
    { header: "Set API Key", component: SetOpenaiKey, open: true},
    { header: "Send Prompt", component: SendPrompt, open: false},
    { header : "Interact with Actions and Processes", component: InteractWithActionsAndProcesses, open: false},
    // {header: "Edit Action or Process", component: JsonEditor, open: false},
    { header: "Add Action", component: AddNodeButton, open: false },
    { header: "Create Process" , component: CreateProcess, open: false}
    // {
    //   header: "Modify Nodes or Edges",
    //   component: ModifyNodesOrEdges,
    //   open: false,
    // },
    // { header: "Delete Edge", component: DeleteEdge, open: false },
    // { header: "View Available Actions"}
  ];

  function toggleSection(clickedSection) {
    sections = sections.map(section => {
      return {...section, open: section === clickedSection};
    });
  }
</script>

<div class="sidebar">
  {#each sections as section (section.header)}
    <div class="section">
      <div
        class="section-header"
        on:keydown={(event) => {
          if (event.key === "Enter") {
            toggleSection(section);
          }
        }}
        on:click={() => toggleSection(section)}
        tabindex="0"
      >
        {section.header}
      </div>
      {#if section.open}
        <div class="section-content" 
             in:fade={{duration: 700}} 
             out:blur={{duration: 700, amount: 5}}>
          <svelte:component this={section.component} />
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  /* styles for the sidebar */
  .sidebar {
    box-shadow: 0px 0px 0px 5px rgba(0, 0, 0, 0.541);
    position: fixed; /* Make the sidebar fixed so it doesn't move with the rest of the content */
    top: 0; /* Align the top of the sidebar with the top of the screen */
    left: 0; /* Align the left side of the sidebar with the left side of the screen */
    bottom: 0; /* Make the sidebar as tall as the screen */
    width: 300px;
    background-color: #f3f3f3; /* Add a background color */
    overflow-y: auto; /* Add scrollbar if the content is too tall */
  }

  /* styles for the sections */
  .section {
    margin-bottom: 10px;
  }

  /* styles for the section headers */
  .section-header {
    cursor: pointer;
    padding: 10px;
    background-color: #e0e0e0;
  }

  /* styles for the section content */
  .section-content {
    padding: 10px;
    background-color: #fff;
  }
</style>
