<script>
  import modifyNode from "./sidebarComponents/modifyNode.svelte";
  import NewNode from "./sidebarComponents/newNode.svelte";

  import { blur, fade } from "svelte/transition";
  import { onMount } from "svelte";
  import ExecuteNode from "./sidebarComponents/executeNode.svelte";
  import systemStateStore from "stores/systemStateStore";
  import SelectedNodeInfo from "./sidebarComponents/selectedNodeInfo.svelte";

  onMount(() => {
    console.log("Sidebar mounted");
  });

  let sections = [
    {
      header: "Create a New Node",
      component: NewNode,
      open: false,
    },
    {
      header: "Modify Node",
      component: modifyNode,
      open: false,
    },
    {
      header: "Execute Process",
      component: ExecuteNode,
      open: false,
    },
    {
      header: "Show Selected Node Info",
      component: SelectedNodeInfo,
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
  /* Sidebar */
  .sidebar {
    grid-column: 1;
    position: sticky;
    top: 0;
    max-width: 200px;
    height: 100vh;
    background-color: #ecf0f1;
    overflow-y: auto;
    box-shadow: 0px 0px 0px 5px rgba(0, 0, 0, 0.1);
    border-radius: 12px;
    padding: 20px;
  }

  .section {
    margin-bottom: 25px;
  }

  .section-header {
    font-size: 18px;
    padding: 10px;
    cursor: pointer;
    background-color: #bdc3c7;
    border: 1px solid #a5a9ab;
    border-radius: 8px;
    transition:
      background-color 0.3s ease,
      transform 0.2s ease;
  }

  .section-header:hover {
    background-color: #a5a9ab;
    transform: scale(1.03);
  }

  .section-content {
    margin-top: 15px;
    padding: 15px;
    border: 1px solid #a5a9ab;
    border-radius: 8px;
    background-color: #fff;
  }
</style>
