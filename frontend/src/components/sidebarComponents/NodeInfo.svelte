<script lang="ts">
    import { Option, isSome, none } from "fp-ts/Option";
    import { unsafeCoerce } from "fp-ts/lib/function";
    import systemStateStore from "stores/systemStateStore";
    import type { Node } from "system_types";

    let selected_node: Node;
    let has_selected_node: boolean = false;

    $: {
        has_selected_node = isSome($systemStateStore.selectedNode);

        if (has_selected_node) {
            selected_node: Node = unsafeCoerce($systemStateStore.selectedNode);
        }
    }
</script>

<!-- Display different info depending on the type of the node -->

{#if has_selected_node}
    {#if selected_node.type_name === "Prompt"}
        <p>Prompt:</p>
    {:else if selected_node.type_name === "Process"}
        <p>Process:</p>
    {:else if selected_node.type_name === "Conditional"}
        <p>Conditional</p>
    {:else if selected_node.type_name === "Command"}
        <p>Command</p>
    {/if}
{:else}
    <p>No node selected</p>
{/if}
