<script lang="ts">

    import websocketStore from "../../stores/websocketStore";
  
    let apiKey = localStorage.getItem("apiKey") || "Api Key";

function set_api_key() {
  
  localStorage.setItem("apiKey", apiKey);
  $websocketStore.send(JSON.stringify({key: apiKey}));

  console.log("Set api key to: " + apiKey);
  
}

  </script>
  
  <div id="cy" />
  <input type="text" bind:value={apiKey} placeholder="Enter the openai api key here." />
  <button on:click={set_api_key}>Set Key</button>
<!--   
  <style>
    input {
      width: 100%;
      height: 100%;
      box-sizing: border-box;
      white-space: pre-wrap;
    }
  </style> -->
