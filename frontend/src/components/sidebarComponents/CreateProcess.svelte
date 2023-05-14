<script lang='ts'>
    import { aiSystemStore } from "stores/aiSystemStore";
    import { AiSystemState, Process } from "system_types";
    import websocketStore from "stores/websocketStore";
    import Select from "svelte-select";

    let process : Process= {
      _id: "",
      name: "",
      steps: [""],
      trigger: "",
      triggers_next_process: "",
      waits_for_branch_completion: false,
      description: "",
      creates_process_branch: false,
      branch_step: ""
    };

    let invalidSteps : String[] = [];
  
    function handleStepsChange(selected : {value: string, label: string}, index : number) {
      process.steps[index] = selected.value;
      if (process.steps.length - 1 === index) {
        process.steps.push("");
      }
    }
  
    function checkStepsValidity() {
      let steps = process.steps;
      let unsubscribe = aiSystemStore.subscribe( (value) => {
        let system: AiSystemState = value;
        for (let step of steps) {
          if (!system.actions.find( (action) => action.name == step)) {
            // remove this action from the process.steps array
            process.steps = process.steps.filter( (value) => value != step);
            
          }
        }
        
      });
      unsubscribe();
    }
  
    function handleSubmit() {
      // if invalidSteps is not empty, then the user has entered an invalid step
      if (invalidSteps.length > 0) {
        alert("Invalid steps: " + invalidSteps.join(", "));
        invalidSteps = [];
        process.steps = [""];
        return;
      }
      if (process.name == "") {
        alert("Please enter a name for the process");
        return;
      }
      if (process.steps.length == 0) {
        alert("Please enter at least one step for the process");
        return;
      }

      checkStepsValidity();
        
      $websocketStore.send(JSON.stringify({"create_process": process}));

      process = {
        _id: "",
        name: "",
        steps: [""],
        trigger: "",
        triggers_next_process: "",
        waits_for_branch_completion: false,
        description: "",
        creates_process_branch: false,
        branch_step: ""
      };
    }
</script>

<form on:submit|preventDefault={handleSubmit}>

  <label>
    Name: <input type="text" bind:value={process.name} />
  </label>
  <label for="steps">
    Steps:
    {#each process.steps as step, index (index)}
        <Select id="steps_{index}"
                bind:value={step}
                items={$aiSystemStore.actions.map(action => ({value: action.name, label: action.name}))}
                on:change={(event) => handleStepsChange(event.detail, index)}
                placeholder="Select step..."
        />
    {/each}
</label>
    <label>
      Trigger: <input type="text" bind:value={process.trigger} />
    </label>
    <label>
      Triggers next process: <input type="text" bind:value={process.triggers_next_process} />
    </label>
    <label>
      Waits for branch completion: <input type="checkbox" bind:checked={process.waits_for_branch_completion} />
    </label>
    <label>
      Description: <input type="text" bind:value={process.description} />
    </label>
    <label>
      Creates process branch: <input type="checkbox" bind:checked={process.creates_process_branch} />
    </label>
    <label>
      Branch step: <input type="text" bind:value={process.branch_step} />
    </label>
    <button type="submit">Submit</button>
  </form>