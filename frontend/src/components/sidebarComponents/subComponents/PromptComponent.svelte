<script lang="ts">
  export let prompt: Prompt;
  import { Prompt } from "../../../generated/system_types_pb";

  let system_text = "";
  let prompt_text = "";

  async function submitPrompt() {
    prompt.setPrompt(prompt_text);
    prompt.setSystem(system_text);
  }
</script>

<form on:submit|preventDefault={submitPrompt}>
  <label for="prompt" class="required-label">Prompt</label>
  <input
    id="prompt"
    bind:value={prompt_text}
    type="text"
    required
    class="required-input"
  />

  <label for="system" class="required-label">System</label>
  <input
    id="system"
    bind:value={system_text}
    type="text"
    required
    class="required-input"
  />
  <button type="submit">Submit</button>
</form>

<style>
  .required-input:invalid {
    border: 2px solid red;
  }

  .required-label::after {
    content: " *";
    color: red;
  }
</style>
