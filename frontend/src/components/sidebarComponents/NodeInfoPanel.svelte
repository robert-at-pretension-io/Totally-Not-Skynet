<script lang="ts">
    import { Node } from "generated/system_types";
    export let node: Node;
</script>

<div class="node-info-panel">
    <h3>Node Info</h3>
    <p><strong>Type:</strong> {node.type_name}</p>
    <p><strong>Input Variables:</strong> {node.input_variables.join(", ")}</p>
    <p><strong>Output Variables:</strong> {node.output_variables.join(", ")}</p>

    <!-- Here you can expand on the `node_content` oneof logic -->
    {#if node.prompt}
        <p><strong>Prompt:</strong> Todo</p>
    {:else if node.process}
        <p><strong>Process:</strong> Todo</p>
    {:else if node.conditional}
        <p><strong>Conditional:</strong> Todo</p>
    {:else if node.command}
        <p><strong>Command:</strong> Todo</p>
    {/if}
</div>
