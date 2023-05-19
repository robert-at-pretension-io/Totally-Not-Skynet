<script>
      let isExpanded = true;

      function toggleSidebar(event) {
        if (event.type === "click" || ["Enter", " "].includes(event.key)) {
          isExpanded = !isExpanded;
        }
      }
  </script>
  
  <!-- <style>
    .sidebar {
      position: fixed;
      top: 0;
      right: 0;
      bottom: 0;
      width: 300px;
      background-color: lightgray;
      transition: width 0.3s;
    }
    
    .sidebar.collapsed {
      width: 30px;
    }
    
    .expandTab {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 30px;
  height: 100%;
  background-color: gray;
  cursor: pointer;
  text-align: center;
  writing-mode: vertical-lr;  /* This will change the text orientation to vertical */
  transform: rotate(180deg);  /* This will flip the text so it's readable top-to-bottom */
}

    .content {
        position: absolute;
        top: 0;
        left: 30px;
    }
  </style> -->
  
  <div class={isExpanded ? "right_sidebar" : "right_sidebar collapsed"}>
    {#if isExpanded}
      <button class="expandTab" on:click={toggleSidebar} on:keydown={toggleSidebar}>Shrink</button>
      <div class="content">
      <p>Some content...</p>
      </div>
      {:else}
      <button class="expandTab" on:click={toggleSidebar} on:keydown={toggleSidebar}>Expand</button>
    {/if}
  </div>
