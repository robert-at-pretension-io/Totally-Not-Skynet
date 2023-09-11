<script>
  import BackgroundInfo from "./sidebarComponents/BackgroundInfo.svelte";
  import InteractWithActionsAndProcesses from "./sidebarComponents/InteractWithActionsAndProcesses.svelte";
  import NewNode from "./sidebarComponents/newNode.svelte";

  import { blur, fade } from "svelte/transition";
  import { onMount } from "svelte";
  import CreateNode from "./sidebarComponents/CreateNode.svelte";
  import { authenticate } from "helper_functions/authentication";
  import { websocketStore } from "stores/websocketStore";
  import systemStateStore from "stores/systemStateStore";

  // onmount

  onMount(() => {
    console.log("Sidebar mounted");
  });

  let sections = [
    {
      header: "Background Information",
      component: BackgroundInfo,
      open: false,
    },

    {
      header: "Create a New Node",
      component: NewNode,
      open: false,
    },
    {
      header: "Create a process",
      component: CreateNode,
      open: false,
    },

    {
      header: "Edit Action or Process",
      component: InteractWithActionsAndProcesses,
      open: false,
    },
  ];

  function toggleSection(clickedSection) {
    sections = sections.map((section) => {
      if (section === clickedSection) {
        let open = !section.open;
        return { ...section, open }; // just invert the `open` property of the clicked section
      } else {
        return section; // don't modify other sections
      }
    });
  }
</script>

<div class="sidebar">
  {#each sections as section (section.header)}
    <div class="section" bind:this={section.ref}>
      <div
        class="section-header"
        on:keydown={(event) => {
          if (event.key === "Enter") {
            toggleSection(section);
          }
        }}
        on:click={() => toggleSection(section)}
      >
        {section.header}
      </div>
      {#if section.open}
        <div
          class="section-content"
          in:fade={{ duration: 100 }}
          out:blur={{ duration: 100, amount: 5 }}
        >
          <svelte:component this={section.component} />
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  /* Sidebar container */
  .sidebar {
    grid-column: 1;
    position: sticky;
    top: 0;
    height: 100vh;
    background-color: #2ecc71;
    /* Green */
    overflow-y: auto;
    box-shadow: 0px 0px 0px 5px rgba(0, 0, 0, 0.541);
    border-radius: 12px;
  }

  /* Section styling */
  .section {
    margin-bottom: 20px;
  }

  /* Section header */
  .section-header {
    font-size: 18px;
    padding: 10px;
    cursor: pointer;
    background-color: #e9e9e9;
    border: 1px solid #ccc;
    border-radius: 5px;
    transition: background-color 0.3s ease;
  }

  /* Hover effect on section header */
  .section-header:hover {
    background-color: #ddd;
  }

  /* Section content */
  .section-content {
    margin-top: 10px;
    padding: 15px;
    border: 1px solid #ccc;
    border-radius: 5px;
    background-color: #fff;
  }
</style>
