<script>
  import {
    AuthenticationMessage,
    CrudBundle,
    VerbTypeNames,
  } from "../generated/system_types_pb";
  import { sendWebsocketMessage } from "helper_functions/websocket";
  import systemStateStore from "stores/systemStateStore";
  import { websocketStore } from "stores/websocketStore";
  import { authenticate } from "helper_functions/authentication";

  let username = "";
  let password = "";

  function handleSubmit() {
    console.log("Username:", username);
    console.log("Password:", password);

    // set authentication to true
    $systemStateStore.setAuthenticated(true);

    let websocket = $websocketStore.websocket;
    authenticate(websocket, "", "");
  }

  // Send POST with authentication Message
</script>

<div class="container">
  <div class="fields">
    <input type="text" bind:value={username} placeholder="Username" />
    <input type="password" bind:value={password} placeholder="Password" />
    <button class="submit-button" on:click={handleSubmit}>Submit</button>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
  }
  .fields {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .submit-button {
    background-color: green;
    color: white;
    padding: 10px 20px;
    border: none;
    cursor: pointer;
    font-size: 16px;
    transition: 0.3s;
  }
  .submit-button:hover {
    background-color: darkgreen;
  }
</style>
