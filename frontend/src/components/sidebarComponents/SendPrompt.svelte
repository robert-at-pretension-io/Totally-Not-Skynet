<script lang="ts">

    import websocketStore from "../../stores/websocketStore";
  
    let prompt = "Prompt";

function send_prompt() {
  
  $websocketStore.send(JSON.stringify({prompt_text: prompt}));

  console.log("Sending prompt: " + prompt);
  
}

  </script>
  
  <div id="cy" />
  <input type="text" bind:value={prompt} placeholder="Enter the openai api key here." />
  <button on:click={send_prompt}>Send Prompt</button>
  
  <!-- <style>
    input {
      width: 100%;
      height: 100%;
      box-sizing: border-box;
      white-space: pre-wrap;
    }
  </style> -->
