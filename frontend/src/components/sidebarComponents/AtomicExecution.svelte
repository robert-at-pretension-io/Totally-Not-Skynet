<script lang="ts">
  import {
    AtomicExecutionLog,
    GraphNodeInfo,
  } from "../../generated/system_types";

  export let logs: AtomicExecutionLog[] = []; // The array of AtomicExecutionLogs passed to this component

  function renderHashMap(log: AtomicExecutionLog) {
    let hashMap = log.response;

    if (!(hashMap instanceof Map)) {
      console.error("Expected a Map object");
      return "";
    }

    return Array.from(hashMap.entries())
      .map(([key, value]) => {
        if (value.has_number_value) {
          return `<strong>${key}</strong>: ${value.number_value}<br>`;
        }
        if (value.has_string_value) {
          return `<strong>${key}</strong>: ${value.string_value}<br>`;
        }
        if (value.has_string_list) {
          // for this one we need to do some more work
          // return each of the list items as a bullet point on a new line:
          let listItems = value.string_list;

          let listItemsAsBullets = listItems.values.map((item) => {
            return `<li>${item}</li>`;
          });
          return `<strong>${key}</strong>: <ul>${listItemsAsBullets.join(
            ""
          )}</ul>`;
        }
      })
      .join(", ");
  }
</script>

{#each logs as log}
  <div class="log-entry">
    <h3>Process executed: {log.has_node_info ? log.node_info.name : "N/A"}</h3>
    <p><strong>Prompt:</strong> {log.prompt}</p>
    <div>
      <strong>Response:</strong>
      <div class="response">
        {@html log.response.size > 0 ? renderHashMap(log) : "No response"}
      </div>
    </div>
  </div>
  <hr />
{/each}

<style>
  .log-entry {
    margin-bottom: 1rem;
  }
  .response {
    margin-top: 0.5rem;
    background-color: #f0f0f0;
    padding: 0.5rem;
    border-radius: 5px;
  }
</style>
