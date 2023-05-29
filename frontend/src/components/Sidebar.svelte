<script>
  import AddNodeButton from "./sidebarComponents/AddNodeButton.svelte";
  import CreateProcess from "./sidebarComponents/CreateProcess.svelte";
  import BackgroundInfo from "./sidebarComponents/BackgroundInfo.svelte";
  import InteractWithActionsAndProcesses from "./sidebarComponents/InteractWithActionsAndProcesses.svelte";
  import Execution from "./sidebarComponents/Execution.svelte";

  import { blur, fade } from "svelte/transition";

  let sections = [
    { header: "Explanation", component: BackgroundInfo, open: false, ref: null },
    {
      header: "Create Process (graph: edges and nodes)",
      component: CreateProcess,
      open: false,
      ref: null,
    },
    { header: "Create Action (node)", component: AddNodeButton, open: false, ref: null },
    {
      header: "Edit Action or Process",
      component: InteractWithActionsAndProcesses,
      open: false,
      ref: null,
    },
    { header: "Execution of Processes", component: Execution, open: false, ref: null },
  ];

  function toggleSection(clickedSection) {
    sections = sections.map((section) => {
      if (section === clickedSection) {
        let open = !section.open;
        if(open && section.ref) {
          section.ref.scrollIntoView({ behavior: "smooth" });
        }
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
