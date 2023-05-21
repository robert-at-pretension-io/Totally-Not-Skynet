<script>
  import AddNodeButton from "./sidebarComponents/AddNodeButton.svelte";
  import SetOpenaiKey from "./sidebarComponents/SetOpenaiKey.svelte";
  import InteractWithActionsAndProcesses from "./sidebarComponents/InteractWithActionsAndProcesses.svelte";
  import CreateProcess from "./sidebarComponents/CreateProcess.svelte";

  import { blur, fade } from "svelte/transition";

  let sections = [
    // { header: "Set API Key", component: SetOpenaiKey, open: true},
    { header : "Edit Actions and Processes", component: InteractWithActionsAndProcesses, open: true},
    { header: "Create Action", component: AddNodeButton, open: true },
    { header: "Create Process" , component: CreateProcess, open: true},
  ];

  function toggleSection(clickedSection) {
    sections = sections.map(section => {
      if(section === clickedSection) {
        return {...section, open: !section.open}; // just invert the `open` property of the clicked section
      } else {
        return section; // don't modify other sections
      }
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
