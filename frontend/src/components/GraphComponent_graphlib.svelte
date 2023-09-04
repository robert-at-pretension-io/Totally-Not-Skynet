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
      await selectNode(selectedNode.id, $systemStateStore);
    });

    cyInstance.on("select", "edge", async (event) => {
      let edge = new proto.Edge();
      edge.setSource(event.target.data().source);
      edge.setTarget(event.target.data().target);

      await selectEdge(edge, $systemStateStore);
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
    let action_list =
      graph_state?.getActionHistoryList() as proto.GraphAction[];

    if (action_list && action_list.length > 0) {
      console.log(
        "action taken in graph: ",
        action_list[action_list.length - 1].toObject()
      );
    }

    if (cyInstance) {
      let layout = cyInstance.layout({
        name: "dagre" /* or whatever layout you're using */,
      });

      console.log("cyInstance is available");

      if (action_list != undefined && action_list.length > 0) {
        console.log("action_list is not empty");

        let last_action = action_list[action_list.length - 1];
        console.log("Last action:", last_action.toObject());

        let action_type = last_action.getAction();
        console.log("Action type:", action_type);

        // Node/edge agnostic actions:
        switch (action_type) {
        case proto.GraphAction.Action.RESET:
          console.log("Removing all elements from cyInstance");
          cyInstance.remove(cyInstance.elements());
          layout.run();
          break;
        case proto.GraphAction.Action.NONE:
          console.log("No action to be taken");
          break;
        case proto.GraphAction.Action.SELECT:
          console.log("Selecting element");
          break;
        case proto.GraphAction.Action.DESELECT:
          console.log("Deselecting element");
          break;

        default:
          console.log("Default case hit");
          break;
        }

        if (last_action.hasEdge()) {
          console.log("Last action has edge");

          let edge = last_action.getEdge();
          let source = edge?.getSource()?.getId();
          let target = edge?.getTarget()?.getId();

          console.log("Edge source:", source, ", target:", target);

          if (source != undefined && target != undefined) {
            switch (action_type) {
            case proto.GraphAction.Action.ADD:
              console.log("Adding edge to graph");
              g.setEdge({ v: source, w: target });
              cyInstance.add({
                data: { source: source, target: target },
              });
              layout.run();

              break;
            case proto.GraphAction.Action.REMOVE:
              console.log("Removing edge from graph");
              g.removeEdge(source, target);
              cyInstance.remove(
                cyInstance.$id(source).edgesTo(cyInstance.$id(target))
              );
              layout.run();

              break;
            default:
              console.log("Edge action not handled");
              break;
            }
          }
        }

        if (last_action.hasNode()) {
          console.log("Last action has node");

          let node = last_action.getNode();
          let id = node?.getId();
          let name = node?.getName();

          console.log("Node ID:", id, ", Name:", name);

          if (name != undefined && id != undefined) {
            switch (action_type) {
            case proto.GraphAction.Action.ADD:
              console.log("Adding node to graph");
              g.setNode(id, name);
              cyInstance.add({
                data: { id: id, label: name },
              });
              layout.run();

              break;
            case proto.GraphAction.Action.REMOVE:
              console.log("Removing node from graph");
              g.removeNode(name);
              cyInstance.remove(id);
              layout.run();

              break;
            default:
              console.log("Node action not handled");
              break;
            }
          }
        }
      } else {
        console.log("action_list is empty or undefined");
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
