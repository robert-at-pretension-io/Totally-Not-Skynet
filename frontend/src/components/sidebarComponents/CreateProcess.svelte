<script lang='ts'>

    import { aiSystemStore } from "stores/aiSystemStore";
    import { AiSystemState } from "system_types";
    export let process = {
      _id: '',
      name: '',
      steps: [],
      trigger: '',
      triggers_next_process: '',
      waits_for_branch_completion: false,
      description: '',
      creates_process_branch: false,
      branch_step: ''
    };

    let invalidSteps : String[] = [];
  
    function handleStepsChange(event) {
      process.steps = event.target.value.split(',');

      //check to see that all of the steps are valid (i.e. exist in the system)
        let steps = process.steps;
        let unsubscribe = aiSystemStore.subscribe( (value) => {
            let system: AiSystemState = value;
            let valid = true;
            for (let step of steps) {
                if (!system.actions.find( (action) => action.name == step)) {
                    invalidSteps.push(step);
                }
            }
            unsubscribe();
        });
        

    }
  
    function handleSubmit() {
      // Here you can handle the submission of the form,
      // for example, send the updated `process` object to an API
      console.log(process);
    }
  </script>
  
  <form on:submit|preventDefault={handleSubmit}>
    <label>
      ID: <input type="text" bind:value={process._id} />
    </label>
    <label>
      Name: <input type="text" bind:value={process.name} />
    </label>
    <label>
      Steps: <input type="text" value={process.steps.join(',')} on:change={handleStepsChange} />
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