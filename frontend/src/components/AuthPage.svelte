<script>
  import { websocketStore } from "stores/websocketStore";
  import { authenticate } from "helper_functions/authentication";
  import { Secrets, UserSettings } from "generated/system_types";

  let username = "";
  let password = "";
  let api_key = "";
  let displayedApiKey = "";

  // Load the API key from localStorage
  api_key = localStorage.getItem("api_key") || "";
  displayedApiKey = maskApiKey(api_key);

  function maskApiKey(key) {
    // Show only the first 4 characters and mask the rest
    return key.substr(0, 4) + key.slice(4).replace(/./g, "*");
  }

  $: displayedApiKey = maskApiKey(api_key);

  function handleApiKeyChange(event) {
    // Update the actual API key when the user edits the input
    api_key = event.target.value;
  }

  function handleSubmit() {
    console.log("Username:", username);
    console.log("Password:", password);

    let secret = new Secrets();

    secret.email = username;
    secret.password = password;

    let user_settings = new UserSettings();
    user_settings.openai_api_key = api_key;

    localStorage.setItem("api_key", api_key);

    secret.user_settings = user_settings;

    // set authentication to true

    let websocket = $websocketStore.websocket;
    authenticate(websocket, secret);
  }

  // Send POST with authentication Message
</script>

<div class="container">
  <div class="fields">
    <input type="text" bind:value={username} placeholder="Username" />
    <input type="password" bind:value={password} placeholder="Password" />
    <textarea
      type="text"
      value={displayedApiKey}
      class="auto-expand"
      on:input={handleApiKeyChange}
      placeholder="API Key"
    /> <button class="submit-button" on:click={handleSubmit}>Submit</button>
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

  .auto-expand {
    overflow-y: hidden; /* Hide vertical scrollbar */
    resize: none; /* Disable resizing */
    min-height: 20px; /* Set a minimum height */
    max-height: 400px; /* Set a maximum height */
  }
</style>
