<script lang="ts">
  import { onMount, setContext } from "svelte";
  import cytoscape, { Core } from "cytoscape";
  import dagre from "cytoscape-dagre";
  import systemStateStore from "stores/systemStateStore.js";
  import { selectNode, selectEdge } from "../helper_functions/graph";
  import * as proto from "../generated/system_types_pb";

  import * as graphlib from "graphlib";

  onMount(async () => {
    cytoscape.use(dagre);

    cyInstance = cytoscape({
      container: refElement,
      style: [
        {
          selector: "node",
          style: {
            width: "label",
            height: "label",
            "font-size": "14px",
            "font-weight": "bold",
            content: "data(label)",
            "text-valign": "center",
            "text-wrap": "wrap",
            "text-max-width": "100px",
            "background-color": "#fff",
            "border-color": "#000",
            "border-width": "1px",
            "border-style": "solid",
          },
        },
        {
          selector: "edge",
          style: {
            "curve-style": "bezier",
            "target-arrow-shape": "triangle",
            "line-color": "#000",
            "target-arrow-color": "#000",
            width: "2px",
          },
        },
      ],
    });

    cyInstance.on("select", "node", async (evt) => {
      const selectedNode = evt.target.data();
      await selectNode(selectedNode.id);
    });

    cyInstance.on("select", "edge", async (event) => {
      let edge = new proto.Edge();
      edge.setSource(event.target.data().source);
      edge.setTarget(event.target.data().target);

      await selectEdge(edge);
    });
  });

  setContext("graphSharedState", {
    getCyInstance: () => cyInstance,
  });

  let refElement: HTMLElement | null = null;
  let cyInstance: Core | null = null;
  let g = new graphlib.Graph();

  systemStateStore.subscribe((system_state: proto.SystemState) => {
    let graph_state = system_state.getGraphState();
    let action_list = graph_state?.getActionHistoryList();

    console.log("action taken in graph: ", action_list);

    if (cyInstance) {
      if (action_list != undefined && action_list.length > 0) {
        let last_action = action_list[action_list.length - 1];

        let action_type = last_action.getAction();

        // Node/edge agnostic actions:
        switch (action_type) {
        case proto.GraphAction.Action.RESET:
          cyInstance.remove(cyInstance.elements());
          break;
        case proto.GraphAction.Action.NONE:
          // Do the thing
          break;
        case proto.GraphAction.Action.SELECT:
          // Do the thing
          break;
        case proto.GraphAction.Action.DESELECT:
          // Do the thing
          break;
        default:
          // Do the thing
          break;
        }

        if (last_action.hasEdge()) {
          let edge = last_action.getEdge();
          let source = edge?.getSource()?.getId();
          let target = edge?.getTarget()?.getId();

          if (source != undefined && target != undefined) {
            switch (action_type) {
            case proto.GraphAction.Action.ADD:
              g.setEdge({ v: source, w: target });
              cyInstance.add({
                data: { source: source, target: target },
              });
              break;
            case proto.GraphAction.Action.REMOVE:
              g.removeEdge(source, target);
              cyInstance.remove(
                cyInstance.$id(source).edgesTo(cyInstance.$id(target))
              );
              break;

            default:
              // Do the thing
              break;
            }
          }
        }

        if (last_action.hasNode()) {
          let node = last_action.getNode();
          let id = node?.getId();
          let name = node?.getName();

          if (name != undefined && id != undefined) {
            switch (action_type) {
            case proto.GraphAction.Action.ADD:
              g.setNode(id, name);
              cyInstance.add({
                data: { id: id, label: name },
              });
              break;
            case proto.GraphAction.Action.REMOVE:
              g.removeNode(name);
              cyInstance.remove(id);
              break;
            default:
              // Do the thing
              break;
            }
          }
        }
      }
    } else {
      console.log("cyInstance is null");
    }
  });
</script>

<div class="graph" bind:this={refElement}>
  {#if cyInstance}
    <slot />
  {/if}
</div>

<style>
  .graph {
    grid-column: 2;
    height: 100%;
  }
</style>
