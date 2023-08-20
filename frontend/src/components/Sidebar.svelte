<script>
  import CreateProcess from "./sidebarComponents/CreateProcess.svelte";
  import BackgroundInfo from "./sidebarComponents/BackgroundInfo.svelte";
  import InteractWithActionsAndProcesses from "./sidebarComponents/InteractWithActionsAndProcesses.svelte";
  import NewNode from "./sidebarComponents/newNode.svelte";

  import { blur, fade } from "svelte/transition";

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
      header: "Create a New Process",
      component: CreateProcess,
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
