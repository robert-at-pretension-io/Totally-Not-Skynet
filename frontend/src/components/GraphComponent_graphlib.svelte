<script lang="ts">
  import { onMount, setContext } from "svelte";
  import cytoscape, { Core } from "cytoscape";
  import dagre from "cytoscape-dagre";
  import GraphStyles from "./GraphStyles.js";
  import systemStateStore from "stores/systemStateStore.js";
  import { selectNode, selectEdge } from "../helper_functions/graph";

  import { SystemState, Edge, GraphAction } from "generated/system_types_pb.js";
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
            "width": "2px",
          },
        },
      ],
    });

    cyInstance.on("select", "node", async (evt) => {
      const selectedNode = evt.target.data();
      await selectNode(selectedNode.id);
    });

    cyInstance.on("select", "edge", async (event) => {
      let edge = new Edge();
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

  systemStateStore.subscribe((system_state: SystemState) => {
    let graph_state = system_state.getGraphState();
    let action_list = graph_state?.getActionHistoryList();

    if (cyInstance) {
      if (action_list != undefined && action_list.length > 0) {
        let last_action = action_list[action_list.length - 1];

        let action_type = last_action.getAction();

        // Node/edge agnostic actions:
        switch (action_type) {
        case GraphAction.Action.RESET:
          cyInstance.remove(cyInstance.elements());
          break;
        case GraphAction.Action.NONE:
          // Do the thing
          break;
        case GraphAction.Action.SELECT:
          // Do the thing
          break;
        case GraphAction.Action.DESELECT:
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
            case GraphAction.Action.ADD:
              g.setEdge({ v: source, w: target });
              cyInstance.add({
                data: { source: source, target: target },
              });

              break;
            case GraphAction.Action.REMOVE:
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
          let id = node?.getNodeInfo()?.getId();
          let name = node?.getNodeInfo()?.getName();

          if (name != undefined && id != undefined) {
            switch (action_type) {
            case GraphAction.Action.ADD:
              g.setNode(id, name);
              cyInstance.add({
                data: { id: id, label: name },
              });
              break;
            case GraphAction.Action.REMOVE:
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
      alert("cytoscape not initialized");
    }
  });
</script>

<div class="graph" bind:this={refElement}>
  {#if cyInstance}
    <slot />
  {/if}
</div>
