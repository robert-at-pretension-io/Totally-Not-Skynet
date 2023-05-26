<script>
    import { graphStore } from "stores/graphStore";

    let graphState = $graphStore;
  </script>
  <aside>
    <h2>Graph Information</h2>
    
    <p>Last Action: {graphState.lastAction}</p>
    <p>Acted On: {graphState.actedOn}</p>
    <p>Last Acted On: {graphState.lastActedOn}</p>
    <p>Name: {graphState.name}</p>
    
    <h3>Global Variables</h3>
    <ul>
      {#each graphState.global_variables as variable (variable)}
        <li>{variable}</li>
      {/each}
    </ul>
  
    <h3>Input Variables</h3>
    <ul>
      {#each graphState.input_variables as variable (variable)}
        <li>{variable}</li>
      {/each}
    </ul>
  
    <h3>Output Variables</h3>
    <ul>
      {#each graphState.output_variables as variable (variable)}
        <li>{variable}</li>
      {/each}
    </ul>
  </aside>
