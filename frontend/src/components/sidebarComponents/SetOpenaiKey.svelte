<script lang="ts">

    import websocketStore from "../../stores/websocketStore";
    let apiKey = localStorage.getItem("apiKey") || "Api Key";
    let mongo_uri = localStorage.getItem("mongo_uri") || "Mongo Uri";

function set_api_key() {

  localStorage.setItem("apiKey", apiKey);
  localStorage.setItem("mongo_uri", mongo_uri);
  $websocketStore.send(JSON.stringify({openai_api_key: apiKey, mongo_db_uri: mongo_uri}));

  $websocketStore.send(JSON.stringify({initial_message: "initial message"}));

  // console.log("Set api key to: " + apiKey);
  // console.log("Set mongo uri to: " + mongo_uri);
  
}

  </script>
  
  <div id="cy" />
  <input type="text" bind:value={apiKey} placeholder="Enter the openai api key here." />
  <input type="text" bind:value={mongo_uri} placeholder="Enter the mongo uri here." />
  <button on:click={set_api_key}>Save Settings</button>
<!--   
  <style>
    input {
      width: 100%;
      height: 100%;
      box-sizing: border-box;
      white-space: pre-wrap;
    }
  </style> -->
