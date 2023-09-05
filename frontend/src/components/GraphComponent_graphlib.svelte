<script lang="ts">
  import { onMount, setContext } from "svelte";
  import cytoscape, { Core } from "cytoscape";
  import dagre from "cytoscape-dagre";
  import systemStateStore from "stores/systemStateStore.js";
  import * as helper_functions from "../helper_functions/graph";
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

    layout = cyInstance.layout({
      name: "dagre" /* or whatever layout you're using */,
    });

    cyInstance.on("select", "node", (evt) => {
      console.log("event: ", evt);

      const selectedNode = evt.target.data();

      console.log("selectedNode: ", selectedNode);

      $systemStateStore = helper_functions.selectNode(
        selectedNode.id,
        $systemStateStore
      );
    });

    cyInstance.on("select", "edge", (evt) => {
      console.log("event: ", evt);

      let edge = new proto.Edge();

      let source = new proto.GraphNodeInfo();
      let target = new proto.GraphNodeInfo();

      source.setId(evt.target.data().source);
      target.setId(evt.target.data().target);

      edge.setSource(source);
      edge.setTarget(target);

      console.log("selectedEdge: ", evt.target.data());

      $systemStateStore = helper_functions.selectEdge(edge, $systemStateStore);
    });
  });

  setContext("graphSharedState", {
    getCyInstance: () => cyInstance,
  });

  let refElement: HTMLElement | null = null;
  let cyInstance: Core | null = null;
  let g = new graphlib.Graph();
  let last_action: proto.GraphAction;
  let layout: cytoscape.Layouts;

  systemStateStore.subscribe((system_state: proto.SystemState) => {
    let graph_state = system_state.getGraphState();
    let action_list =
      graph_state?.getActionHistoryList() as proto.GraphAction[];
    let latest_action: proto.GraphAction;

    if (action_list && action_list.length > 0) {
      latest_action = action_list[action_list.length - 1];
      console.log(
        "action taken in graph: ",
        action_list[action_list.length - 1].toObject()
      );

      if (last_action != undefined) {
        console.log("last action: ", last_action.toObject());

        if (
          last_action.getAction() == latest_action.getAction() &&
          last_action.getNode()?.getId() == latest_action.getNode()?.getId()
        ) {
          console.log("same action taken twice");
          return;
        }
      }
    } else {
      console.log("action_list is empty or undefined");
      return;
    }

    if (cyInstance) {
      layout.run();

      console.log("cyInstance is available");

      if (action_list != undefined && action_list.length > 0) {
        let action_type = latest_action.getAction();
        console.log("Action type:", action_type);

        // Node/edge agnostic actions:
        switch (action_type) {
        case proto.GraphAction.Action.RESET:
          console.log("Removing all elements from cyInstance");
          cyInstance.remove(cyInstance.elements());
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

        if (latest_action.hasEdge()) {
          console.log("Last action has edge");

          let edge = latest_action.getEdge();
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

              break;
            case proto.GraphAction.Action.REMOVE:
              {
                console.log("Removing edge from graph");
                let sourceNode = cyInstance.getElementById(source);
                let targetNode = cyInstance.getElementById(target);
                let edge = sourceNode.edgesTo(targetNode).first();

                cyInstance.remove(edge);
              }

              break;
            default:
              console.log("Edge action not handled");
              break;
            }
          }
        }

        if (latest_action.hasNode()) {
          console.log("Last action has node");

          let node = latest_action.getNode();
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

              break;
            case proto.GraphAction.Action.REMOVE:
              console.log("Removing node from graph");
              g.removeNode(name);

              alert(
                "Also need to remove the edges connected to this node from the system_state"
              );

              cyInstance.remove(cyInstance.$id(id));
              // layout.run();

              break;
            default:
              console.log("Node action not handled");
              break;
            }
          }
        }

        last_action = latest_action;
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
