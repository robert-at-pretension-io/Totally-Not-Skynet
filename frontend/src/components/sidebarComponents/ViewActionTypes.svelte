<script lang="ts">
    import type { Node } from "../../system_types";
  
    import { getUniqueId, addNode } from "../../helper_functions/graph";
  
    let label = "Node Label";
    let data = {};
  
    async function localAddNode() {
      let id = await getUniqueId();
  
      let newNode: Node = {
        id: id,
        label: label,
        data: data,
      };
  
      await addNode(newNode);
    }
  </script>
  
  <!-- For each of the actions available, add them into a scrollable list -->
  
  <style>

  </style>
  