<script>
  import AddNodeButton from "./sidebarComponents/AddNodeButton.svelte";
  import ModifyNodesOrEdges from "./sidebarComponents/ModifyNodesOrEdges.svelte";
  import DeleteEdge from "./sidebarComponents/DeleteEdge.svelte";

  let sections = [
    { header: "Add Node", component: AddNodeButton, open: false },
    {
      header: "Modify Nodes or Edges",
      component: ModifyNodesOrEdges,
      open: false,
    },
    { header: "Delete Edge", component: DeleteEdge, open: false },
    { header: "View Available Actions"}
  ];
</script>

<div class="sidebar">
  {#each sections as section}
    <div class="section">
      <div
        class="section-header"
        on:keydown={(event) => {
          if (event.key === "Enter") {
            section.open = !section.open;
          }
        }}
        on:click={() => (section.open = !section.open)}
      >
        {section.header}
      </div>
      {#if section.open}
        <div class="section-content">
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
